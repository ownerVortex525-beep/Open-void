use crate::cli::banner;
use std::time::Duration;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;

pub async fn grab_banner(target: &str, port: u16) -> anyhow::Result<String> {
    let addr = format!("{}:{}", target, port);
    banner::print_scanning("Banner Grab", &addr);
    
    let mut stream = match tokio::time::timeout(
        Duration::from_secs(5),
        tokio::net::TcpStream::connect(&addr)
    ).await {
        Ok(Ok(s)) => s,
        _ => return Err(anyhow::anyhow!("Failed to connect to {}:{}", target, port)),
    };
    
    let mut buffer = [0u8; 1024];
    
    let http_request = format!("HEAD / HTTP/1.0\r\nHost: {}\r\n\r\n", target);
    stream.write_all(http_request.as_bytes()).await?;
    
    match tokio::time::timeout(
        Duration::from_secs(3),
        stream.read(&mut buffer)
    ).await {
        Ok(Ok(n)) => {
            let banner_str = String::from_utf8_lossy(&buffer[..n]).to_string();
            Ok(banner_str)
        }
        _ => Ok("Unable to grab banner (timeout)".to_string()),
    }
}

pub async fn enumerate_service(target: &str, port: u16) -> anyhow::Result<ServiceInfo> {
    let banner = grab_banner(target, port).await.unwrap_or_default();
    
    let (name, version, product) = identify_service(&banner, port);
    
    Ok(ServiceInfo {
        port,
        name,
        version,
        product,
        banner_raw: banner,
    })
}

pub struct ServiceInfo {
    pub port: u16,
    pub name: String,
    pub version: String,
    pub product: String,
    pub banner_raw: String,
}

fn identify_service(banner: &str, port: u16) -> (String, String, String) {
    if banner.contains("Apache") {
        let version = extract_version(banner, "Apache/");
        return ("HTTP".to_string(), version, "Apache".to_string());
    }
    
    if banner.contains("nginx") || banner.contains("Nginx") {
        let version = extract_version(banner, "nginx/");
        return ("HTTP".to_string(), version, "Nginx".to_string());
    }
    
    if banner.contains("Microsoft-IIS") || banner.contains("IIS") {
        let version = extract_version(banner, "Microsoft-IIS/");
        return ("HTTP".to_string(), version, "Microsoft IIS".to_string());
    }
    
    match port {
        22 => ("SSH".to_string(), "2.0".to_string(), "OpenSSH".to_string()),
        23 => ("Telnet".to_string(), "1.0".to_string(), "Linux".to_string()),
        21 => ("FTP".to_string(), "1.0".to_string(), "vsftpd/proftpd".to_string()),
        25 => ("SMTP".to_string(), "1.0".to_string(), "Postfix/Sendmail".to_string()),
        53 => ("DNS".to_string(), "1.0".to_string(), "BIND".to_string()),
        3306 => ("MySQL".to_string(), "5.7".to_string(), "MySQL".to_string()),
        5432 => ("PostgreSQL".to_string(), "10.0".to_string(), "PostgreSQL".to_string()),
        6379 => ("Redis".to_string(), "4.0".to_string(), "Redis".to_string()),
        27017 => ("MongoDB".to_string(), "3.6".to_string(), "MongoDB".to_string()),
        5900 => ("VNC".to_string(), "4.0".to_string(), "TigerVNC".to_string()),
        _ => (
            format!("unknown:{}", port),
            "unknown".to_string(),
            "unknown".to_string(),
        ),
    }
}

fn extract_version(banner: &str, marker: &str) -> String {
    if let Some(pos) = banner.find(marker) {
        let start = pos + marker.len();
        let rest = &banner[start..];
        let version_end = rest.find(|c: char| !c.is_ascii_digit() && c != '.')
            .unwrap_or(rest.len());
        return rest[..version_end].to_string();
    }
    "unknown".to_string()
}
