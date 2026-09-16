pub mod scanner;
pub mod banner_grab;
pub mod os_fingerprint;

use crate::cli::banner;
use crate::http::client::HttpClient;
use std::time::Instant;

pub struct NetworkScanResult {
    pub target: String,
    pub scan_type: String,
    pub live_hosts: Vec<LiveHost>,
    pub open_ports: Vec<OpenPort>,
    pub services: Vec<Service>,
    pub os_fingerprint: Option<OSFingerprint>,
    pub start_time: Instant,
    pub total_requests: usize,
}

pub struct LiveHost {
    pub ip: String,
    pub host_status: bool,
    pub latency_ms: f64,
}

pub struct OpenPort {
    pub ip: String,
    pub port: u16,
    pub protocol: String,
    pub state: String,
    pub scan_method: String,
}

pub struct Service {
    pub port: u16,
    pub name: String,
    pub version: String,
    pub product: String,
    pub cpe: String,
}

pub struct OSFingerprint {
    pub os: String,
    pub accuracy: u8,
    pub details: String,
}

impl NetworkScanResult {
    pub fn new(target: String, scan_type: String) -> Self {
        Self {
            target,
            scan_type,
            live_hosts: Vec::new(),
            open_ports: Vec::new(),
            services: Vec::new(),
            os_fingerprint: None,
            start_time: Instant::now(),
            total_requests: 0,
        }
    }

    pub fn print_summary(&self) {
        banner::print_phase_banner(&format!("{} Scan Results", &self.scan_type), &self.target);
        
        for host in &self.live_hosts {
            banner::success(&format!(
                "Host: {} (Latency: {:.1}ms, Status: {})",
                host.ip, host.latency_ms,
                if host.host_status { "UP" } else { "DOWN" }
            ));
        }

        for port in &self.open_ports {
            banner::finding(
                if port.state == "OPEN" { "HIGH" } else { "INFO" },
                &format!("Port {}/{}", port.port, port.protocol),
                &format!("{}:{} ({})", self.target, port.port, port.scan_method),
                &format!("State: {}", port.state)
            );
        }

        for service in &self.services {
            banner::info(&format!(
                "Service: {} v{} on port {} ({})",
                service.name, service.version, service.port, service.product
            ));
        }

        if let Some(os) = &self.os_fingerprint {
            banner::info(&format!(
                "OS Fingerprint: {} (Accuracy: {}%)",
                os.os, os.accuracy
            ));
        }

        let duration = self.start_time.elapsed();
        banner::print_complete(
            &format!("{:.2}s", duration.as_secs_f64()),
            self.live_hosts.len(),
            self.total_requests
        );
    }
}

pub async fn run_scan(
    target: &str,
    scan_type: &str,
    root: bool,
    ports: Option<&str>,
    client: &HttpClient
) -> anyhow::Result<NetworkScanResult> {
    let mut result = NetworkScanResult::new(target.to_string(), scan_type.to_string());
    
    match scan_type.to_lowercase().as_str() {
        "syn" => {
            if !root {
                eprintln!("  ➥ [WARNING] Root is required to use this power");
                eprintln!("  ➥ [INFO] You are not root. Root your device to use this power.");
                return Err(anyhow::anyhow!("Root privileges required for SYN scan"));
            }
            scanner::syn_scan(target, &mut result).await?;
        }
        "connect" => {
            scanner::connect_scan(target, ports, &mut result, client).await?;
        }
        "udp" => {
            scanner::udp_scan(target, &mut result, client).await?;
        }
        "ack" => {
            scanner::ack_scan(target, &mut result).await?;
        }
        "fin" => {
            scanner::fin_scan(target, &mut result).await?;
        }
        _ => {
            scanner::connect_scan(target, ports, &mut result, client).await?;
        }
    }
    
    Ok(result)
}
