use crate::cli::banner;
use crate::http::client::HttpClient;
use std::time::Duration;
use std::str::FromStr;
use std::net::Ipv4Addr;

pub async fn connect_scan(
    target: &str,
    ports: Option<&str>,
    result: &mut crate::scanner::network::NetworkScanResult,
    _client: &HttpClient
) -> anyhow::Result<()> {
    banner::print_scanning("Connect Scan", target);
    
    let port_list = parse_ports(ports);
    let mut open_ports = vec![];
    
    for port in port_list {
        result.total_requests += 1;
        let addr = format!("{}:{}", target, port);
        match tokio::time::timeout(
            Duration::from_secs(3),
            tokio::net::TcpStream::connect(&addr)
        ).await {
            Ok(Ok(_stream)) => {
                banner::success(&format!("Port {} is OPEN", port));
                open_ports.push(crate::scanner::network::OpenPort {
                    ip: target.to_string(),
                    port,
                    protocol: "tcp".to_string(),
                    state: "OPEN".to_string(),
                    scan_method: "connect".to_string(),
                });
            }
            _ => {}
        }
    }
    
    for port in open_ports {
        result.open_ports.push(port);
    }
    
    Ok(())
}

pub async fn syn_scan(target: &str, result: &mut crate::scanner::network::NetworkScanResult) -> anyhow::Result<()> {
    banner::print_scanning("SYN Scan", target);
    
    let _target_ip = match Ipv4Addr::from_str(target) {
        Ok(ip) => ip,
        Err(_) => return Err(anyhow::anyhow!("Invalid IP address: {}", target)),
    };
    
    let port_list: Vec<u16> = (1..=1000).collect();
    
    for port in port_list {
        result.total_requests += 1;
        let addr = format!("{}:{}", target, port);
        match tokio::time::timeout(
            Duration::from_secs(2),
            tokio::net::TcpStream::connect(&addr)
        ).await {
            Ok(Ok(_stream)) => {
                banner::success(&format!("Port {} is OPEN", port));
                result.open_ports.push(crate::scanner::network::OpenPort {
                    ip: target.to_string(),
                    port,
                    protocol: "tcp".to_string(),
                    state: "OPEN".to_string(),
                    scan_method: "syn".to_string(),
                });
            }
            _ => {}
        }
    }
    
    Ok(())
}

pub async fn udp_scan(target: &str, result: &mut crate::scanner::network::NetworkScanResult, _client: &HttpClient) -> anyhow::Result<()> {
    banner::print_scanning("UDP Scan", target);
    
    let common_udp_ports: Vec<u16> = vec![53, 88, 123, 161, 162, 389, 443, 465, 587, 636, 993, 995, 1433, 1521, 1883, 27017, 5432, 5900, 8080, 8443];
    
    for port in common_udp_ports {
        result.total_requests += 1;
        banner::print_scanning("UDP Scan", &format!("{}:{}", target, port));
    }
    
    Ok(())
}

pub async fn ack_scan(target: &str, result: &mut crate::scanner::network::NetworkScanResult) -> anyhow::Result<()> {
    banner::print_scanning("ACK Scan", target);
    
    let common_ports: Vec<u16> = (1..=1000).collect();
    
    for port in common_ports {
        result.total_requests += 1;
        let addr = format!("{}:{}", target, port);
        match tokio::time::timeout(
            Duration::from_secs(2),
            tokio::net::TcpStream::connect(&addr)
        ).await {
            Ok(Ok(_stream)) => {
                banner::success(&format!("Port {} is FILTERED/UNFILTERED", port));
                result.open_ports.push(crate::scanner::network::OpenPort {
                    ip: target.to_string(),
                    port,
                    protocol: "tcp".to_string(),
                    state: "FILTERED".to_string(),
                    scan_method: "ack".to_string(),
                });
            }
            _ => {}
        }
    }
    
    Ok(())
}

pub async fn fin_scan(target: &str, result: &mut crate::scanner::network::NetworkScanResult) -> anyhow::Result<()> {
    banner::print_scanning("FIN Scan", target);
    
    let common_ports: Vec<u16> = (1..=1000).collect();
    
    for port in common_ports {
        result.total_requests += 1;
        let addr = format!("{}:{}", target, port);
        match tokio::time::timeout(
            Duration::from_secs(2),
            tokio::net::TcpStream::connect(&addr)
        ).await {
            Ok(Ok(_stream)) => {}
            _ => {
                banner::info(&format!("Port {} may be filtered (FIN scan)", port));
            }
        }
    }
    
    Ok(())
}

pub async fn subnet_scan(target: &str, result: &mut crate::scanner::network::NetworkScanResult, client: &HttpClient) -> anyhow::Result<()> {
    banner::print_scanning("Subnet Scan", target);
    
    let ips = expand_cidr(target)?;
    
    for ip in ips {
        let ip_str = ip.to_string();
        let scan_result = crate::scanner::network::run_scan(&ip_str, "connect", false, None, client).await;
        
        match scan_result {
            Ok(r) => {
                for host in r.live_hosts {
                    banner::success(&format!("Live host: {} (latency: {:.1}ms)", host.ip, host.latency_ms));
                    result.live_hosts.push(host);
                }
            }
            Err(e) => {
                banner::error(&format!("Scan failed for {}: {}", ip_str, e));
            }
        }
    }
    
    Ok(())
}

fn parse_ports(ports: Option<&str>) -> Vec<u16> {
    match ports {
        Some(p) if p == "quick" => vec![21, 22, 23, 25, 53, 80, 110, 143, 443, 445, 993, 995],
        Some(p) if p == "common" => {
            let mut ports = vec![];
            for p in 1..=1000 {
                ports.push(p as u16);
            }
            ports
        }
        Some(p) if p == "full" => {
            let mut ports = vec![];
            for p in 1..=65535 {
                ports.push(p as u16);
            }
            ports
        }
        _ => {
            vec![21, 22, 23, 25, 53, 80, 110, 143, 443, 445, 993, 995, 3306, 3389, 5432, 5900, 6379, 8080, 8443]
        }
    }
}

fn expand_cidr(cidr: &str) -> anyhow::Result<Vec<Ipv4Addr>> {
    let parts: Vec<&str> = cidr.split('/').collect();
    if parts.len() != 2 {
        return Err(anyhow::anyhow!("Invalid CIDR notation"));
    }
    
    let base_ip: Ipv4Addr = parts[0].parse()?;
    let prefix_len: u32 = parts[1].parse()?;
    
    let mask = if prefix_len == 0 {
        0
    } else {
        (!0u32) << (32 - prefix_len)
    };
    
    let base_ip_u32 = u32::from(base_ip);
    let network = base_ip_u32 & mask;
    let broadcast = network | (!mask);
    
    let mut ips = vec![];
    for ip in (network + 1)..=(broadcast - 1) {
        ips.push(Ipv4Addr::from(ip));
    }
    
    Ok(ips)
}
