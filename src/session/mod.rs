use crate::cli::banner;
use crate::utils::log::{LiveLogger};
use std::io::{Read, Write, BufRead, BufReader};
use std::net::{TcpStream, TcpListener};
use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};
use std::sync::Arc;
use std::sync::Mutex;
use std::collections::HashMap;
use std::time::{Duration, Instant};
use std::thread;

static LOGGER: std::sync::OnceLock<LiveLogger> = std::sync::OnceLock::new();

fn logger() -> &'static LiveLogger {
    LOGGER.get_or_init(|| LiveLogger::new())
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum SessionType {
    ReverseTcp,
    ReverseHttp,
    BindTcp,
    NamedPipe,
    PortForwarded,
}

#[derive(Clone, Debug)]
pub struct TunnelInfo {
    pub lhost: String,
    pub lport: u16,
    pub rhost: String,
    pub rport: u16,
    pub session_type: SessionType,
}

#[derive(Clone)]
pub struct Session {
    pub sid: u32,
    pub session_type: SessionType,
    pub tunnel: TunnelInfo,
    pub payload_uuid: String,
    pub connected_at: Instant,
    pub last_seen: Instant,
    pub active: bool,
    pub platform: String,
}

#[derive(Clone)]
pub struct SessionManager {
    sessions: Arc<Mutex<HashMap<u32, Session>>>,
    sid_pool: Arc<AtomicU32>,
    active: Arc<Mutex<bool>>,
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(Mutex::new(HashMap::new())),
            sid_pool: Arc::new(AtomicU32::new(1)),
            active: Arc::new(Mutex::new(false)),
        }
    }

    pub fn register(&self, session_type: SessionType, tunnel: TunnelInfo) -> u32 {
        let sid = self.sid_pool.fetch_add(1, Ordering::SeqCst);
        let rhost = tunnel.rhost.clone();
        let rport = tunnel.rport;
        let type_name = match session_type {
            SessionType::ReverseTcp => "reverse_tcp",
            SessionType::ReverseHttp => "reverse_http",
            SessionType::BindTcp => "bind_tcp",
            SessionType::NamedPipe => "named_pipe",
            SessionType::PortForwarded => "port_forwarded",
        };
        let session = Session {
            sid,
            session_type,
            tunnel,
            payload_uuid: uuid::Uuid::new_v4().to_string(),
            connected_at: Instant::now(),
            last_seen: Instant::now(),
            active: true,
            platform: "unknown".to_string(),
        };
        
        self.sessions.lock().unwrap().insert(sid, session);
        logger().success(&format!("Session {} registered | type: {} | from: {}:{}", 
            sid, type_name, rhost, rport));
        sid
    }

    pub fn get(&self, sid: u32) -> Option<Session> {
        self.sessions.lock().unwrap().get(&sid).cloned()
    }

    pub fn list(&self) -> Vec<Session> {
        self.sessions.lock().unwrap().values().cloned().collect()
    }

    pub fn deregister(&self, sid: u32) {
        self.sessions.lock().unwrap().remove(&sid);
        logger().info(&format!("Session {} deregistered", sid));
    }

    pub fn set_active(&self, active: bool) {
        *self.active.lock().unwrap() = active;
    }

    pub fn is_active(&self) -> bool {
        *self.active.lock().unwrap()
    }

    pub fn count(&self) -> usize {
        self.sessions.lock().unwrap().len()
    }

    pub fn list_sessions(&self) -> Vec<Session> {
        self.list()
    }
}

pub trait Handler: Send + Sync {
    fn handler_type(&self) -> &'static str;
    fn setup(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    fn start(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    fn stop(&mut self);
    fn handle_connection(&self, stream: TcpStream) -> Result<(), Box<dyn std::error::Error>>;
}

pub struct ReverseTcpHandler {
    pub manager: SessionManager,
    pub lhost: String,
    pub lport: u16,
    pub running: bool,
}

impl ReverseTcpHandler {
    pub fn new(lhost: &str, lport: u16) -> Self {
        Self {
            manager: SessionManager::new(),
            lhost: lhost.to_string(),
            lport,
            running: false,
        }
    }

    pub fn listen(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let bind_addr = format!("{}:{}", self.lhost, self.lport);
        let listener = TcpListener::bind(&bind_addr)?;
        listener.set_nonblocking(true)?;
        self.running = true;
        self.manager.set_active(true);
        
        banner::print_phase_banner("SESSION HANDLER", &format!("Reverse TCP on {}", bind_addr));
        logger().success(&format!("Listening on {} | waiting for connections...", bind_addr));
        logger().info(&format!("Use: cf-void -R --lhost {} --lport {}", self.lhost, self.lport));
        
        for stream in listener.incoming() {
            if !self.running {
                break;
            }
            
            match stream {
                Ok(stream) => {
                    let peer_addr = stream.peer_addr()?;
                    logger().success(&format!("Connection from {}:{}", peer_addr.ip(), peer_addr.port()));
                    
                    let tunnel = TunnelInfo {
                        lhost: self.lhost.clone(),
                        lport: self.lport,
                        rhost: peer_addr.ip().to_string(),
                        rport: peer_addr.port(),
                        session_type: SessionType::ReverseTcp,
                    };
                    
                    let sid = self.manager.register(SessionType::ReverseTcp, tunnel);
                    logger().info(&format!("Session {} active - type 'use {}' to interact", sid, sid));
                    
                    // Handle session in background thread
                    let manager = self.manager.clone();
                    thread::spawn(move || {
                        if let Err(e) = Self::handle_session(&manager, sid, stream) {
                            logger().fail(&format!("Session {} error: {}", sid, e));
                        }
                    });
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    std::thread::sleep(Duration::from_millis(100));
                }
                Err(e) => {
                    logger().fail(&format!("Connection error: {}", e));
                }
            }
        }
        
        Ok(())
    }

    pub fn handle_session(manager: &SessionManager, sid: u32, stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
        Self::interact_stream(&stream).map_err(|e| {
            manager.deregister(sid);
            e
        })
    }

    pub fn interact_stream(stream: &TcpStream) -> Result<(), Box<dyn std::error::Error>> {
        banner::print_phase_banner("SESSION INTERACTION", "Type commands (exit to quit, back to return to menu)");

        let mut reader = BufReader::new(stream.try_clone()?);
        stream.set_read_timeout(Some(Duration::from_secs(5)))?;
        let stdin = std::io::stdin();

        loop {
            print!("> ");
            std::io::stdout().flush()?;

            let mut input = String::new();
            match stdin.lock().read_line(&mut input) {
                Ok(0) => break,
                Ok(_) => {}
                Err(_) => break,
            }

            let cmd = input.trim();
            if cmd == "exit" || cmd == "quit" {
                logger().info("Exiting session...");
                break;
            }
            if cmd == "back" {
                logger().info("Returning to main menu...");
                break;
            }
            if cmd == "help" {
                println!("Commands: exit, quit, back, help, bg, clear");
                continue;
            }
            if cmd == "bg" {
                logger().info("Session backgrounded. Return with 'sessions'");
                break;
            }
            if cmd == "clear" {
                print!("\x1B[2J\x1B[1;1H");
                std::io::stdout().flush()?;
                continue;
            }
            if cmd.is_empty() {
                continue;
            }

            // Send command
            if let Ok(mut writer) = stream.try_clone() {
                writer.write_all(format!("{}\n", input).as_bytes())?;
                writer.flush()?;

                logger().progress(&format!("Sending: {}", cmd));
                std::thread::sleep(Duration::from_millis(100));

                let mut buffer = [0; 4096];
                loop {
                    match reader.read(&mut buffer) {
                        Ok(n) if n > 0 => {
                            let output = String::from_utf8_lossy(&buffer[..n]);
                            print!("{}", output);
                            std::io::stdout().flush()?;
                        }
                        Ok(_) => break,
                        Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => break,
                        Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => break,
                        Err(e) => {
                            logger().fail(&format!("Connection closed: {}", e));
                            break;
                        }
                    }
                }
            }
        }

        stream.set_read_timeout(None)?;
        Ok(())
    }
}

impl Handler for ReverseTcpHandler {
    fn handler_type(&self) -> &'static str {
        "reverse_tcp"
    }

    fn setup(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.listen()
    }

    fn stop(&mut self) {
        self.running = false;
        self.manager.set_active(false);
        logger().info("Listener stopped");
    }

    fn handle_connection(&self, stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
        Self::interact_stream(&stream)
    }
}

// Port forwarding (Metasploit-style)
pub struct PortForwarder {
    pub id_pool: AtomicU32,
    pub forwards: Arc<Mutex<Vec<ForwardEntry>>>,
}

#[derive(Clone)]
pub struct ForwardEntry {
    pub id: u32,
    pub lhost: String,
    pub lport: u16,
    pub rhost: String,
    pub rport: u16,
    pub running: bool,
}

impl PortForwarder {
    pub fn new() -> Self {
        Self {
            id_pool: AtomicU32::new(1),
            forwards: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn add(&self, lhost: &str, lport: u16, rhost: &str, rport: u16) -> Result<u32, Box<dyn std::error::Error + Send + Sync>> {
        let id = self.id_pool.fetch_add(1, Ordering::SeqCst);
        let listener = TcpListener::bind(format!("{}:{}", lhost, lport))?;
        listener.set_nonblocking(true)?;
        
        logger().success(&format!("Port forward {}: {}:{} -> {}:{}", 
            id, lhost, lport, rhost, rport));
        
        self.forwards.lock().unwrap().push(ForwardEntry {
            id,
            lhost: lhost.to_string(),
            lport,
            rhost: rhost.to_string(),
            rport,
            running: true,
        });

        let rhost_owned = rhost.to_string();
        let rport_owned = rport;
        let f_id = id;
        thread::spawn(move || {
            for stream in listener.incoming() {
                if let Ok(stream) = stream {
                    match TcpStream::connect(format!("{}:{}", rhost_owned, rport_owned)) {
                        Ok(mut remote) => {
                            logger().success(&format!("Forward {}: connection established", f_id));
                            
                            // Bidirectional pipe
                            let mut local_in = stream.try_clone().unwrap();
                            let mut remote_out = remote.try_clone().unwrap();
                            let mut local_out = stream.try_clone().unwrap();
                            let mut remote_in = remote.try_clone().unwrap();
                            
                            let h1 = thread::spawn(move || {
                                let mut buf = [0; 4096];
                                loop {
                                    match local_in.read(&mut buf) {
                                        Ok(0) => break,
                                        Ok(n) => { let _ = remote_out.write_all(&buf[..n]); }
                                        Err(_) => break,
                                    }
                                }
                            });
                            
                            let h2 = thread::spawn(move || {
                                let mut buf = [0; 4096];
                                loop {
                                    match remote_in.read(&mut buf) {
                                        Ok(0) => break,
                                        Ok(n) => { let _ = local_out.write_all(&buf[..n]); }
                                        Err(_) => break,
                                    }
                                }
                            });
                            
                            let _ = h1.join();
                            let _ = h2.join();
                        }
                        Err(e) => {
                            logger().fail(&format!("Forward {}: connect failed: {}", f_id, e));
                        }
                    }
                }
            }
        });

        Ok(id)
    }

    pub fn list(&self) -> Vec<ForwardEntry> {
        self.forwards.lock().unwrap().clone()
    }

    pub fn delete(&self, id: u32) -> bool {
        let mut f = self.forwards.lock().unwrap();
        let pos = f.iter().position(|e| e.id == id);
        if let Some(pos) = pos {
            f.remove(pos);
            logger().info(&format!("Port forward {} deleted", id));
            true
        } else {
            false
        }
    }
}

pub struct ReverseHttpHandler {
    pub manager: SessionManager,
    pub lhost: String,
    pub lport: u16,
    pub running: bool,
}

impl ReverseHttpHandler {
    pub fn new(lhost: &str, lport: u16) -> Self {
        Self {
            manager: SessionManager::new(),
            lhost: lhost.to_string(),
            lport,
            running: false,
        }
    }

    pub fn listen(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        banner::print_phase_banner("HTTP HANDLER", &format!("HTTP on {}:{}", self.lhost, self.lport));
        banner::success(&format!("HTTP handler listening on {}:{}", self.lhost, self.lport));
        banner::info(&format!("Use with: --payload-lang powershell --ps-variant web_delivery --lhost {} --lport {}", self.lhost, self.lport));
        Ok(())
    }
}

impl Handler for ReverseHttpHandler {
    fn handler_type(&self) -> &'static str {
        "reverse_http"
    }

    fn setup(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.listen()
    }

    fn stop(&mut self) {
        self.running = false;
    }

    fn handle_connection(&self, _stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}
