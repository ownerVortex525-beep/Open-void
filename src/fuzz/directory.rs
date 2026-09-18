use crate::cli::banner;
use crate::http::client::HttpClient;
use crate::fuzz::FuzzResult;
use std::time::Duration;

static DEFAULT_EXTENSIONS: &[&str] = &["", ".html", ".php", ".js", ".css", ".xml", ".json", ".asp", ".aspx"];
static ADMIN_PATHS: &[&str] = &[
    "admin", "administrator", "wp-admin", "wp-login", "cpanel", "adminpanel",
    "login", "admin/login", "backend", "manager", "console", "dashboard",
    "phpmyadmin", "admin123", "secret", "private", ".git", ".env", ".ssh",
    "config", "backup", "db", "database", "setup", "install", "test",
    "dev", "staging", "uat", "intranet", "portal", "members", "users",
    "api", "api/v1", "api/v2", "docs", "swagger", "openapi",
];

static SENSITIVE_FILES: &[&str] = &[
    ".git/config", ".env", "config.php", "config.json", "config.xml",
    "settings.php", ".htaccess", "web.config", "app.config",
    "id_rsa", "id_dsa", ".htpasswd", "passwd", "shadow",
    "backup.sql", "database.sql", "dump.sql",
    "robots.txt", "sitemap.xml", "sitemap_index.xml",
    "favicon.ico", "crossdomain.xml", "clientaccesspolicy.xml",
];

pub async fn fuzz_directories(
    url: &str,
    extensions: Option<&str>,
    result: &mut FuzzResult,
    client: &HttpClient
) -> anyhow::Result<()> {
    banner::print_scanning("Directory Fuzzing", url);

    let exts: Vec<&str> = match extensions {
        Some(ext_str) => ext_str.split(',').collect(),
        None => DEFAULT_EXTENSIONS.iter().map(|s| *s).collect(),
    };

    let paths: Vec<String> = ADMIN_PATHS.iter()
        .chain(SENSITIVE_FILES.iter())
        .flat_map(|p| {
            exts.iter().map(move |e| format!("/{}{}", p, e))
        })
        .collect();

    let base_url = url.trim_end_matches('/');
    let client_clone = client;

    // Run with concurrency limit of 10 using buffer_unordered
    let futures: Vec<_> = paths.iter().map(|path| {
        let full_url = format!("{}{}", base_url, path);
        let c = client_clone;
        let path = path.clone();
        async move {
            let req = c.get(&full_url);
            match tokio::time::timeout(Duration::from_secs(3), req).await {
                Ok(Ok(response)) => {
                    let status = response.status().as_u16();
                    let content_length = response.content_length().unwrap_or(0) as usize;
                    let content_type = response.headers().get("content-type")
                        .map(|v| v.to_str().unwrap_or("unknown").to_string())
                        .unwrap_or_default();
                    (path, status, content_length, content_type)
                }
                _ => (path, 0, 0, String::new()),
            }
        }
    }).collect();

    use futures::StreamExt;
    let stream = futures::stream::iter(futures).buffer_unordered(10);
    tokio::pin!(stream);

    while let Some((path, status, content_length, content_type)) = stream.next().await {
        result.total_requests += 1;
        if status != 404 && status != 0 {
            banner::success(&format!("Found: {} (Status: {})", path, status));
            result.found_paths.push(crate::fuzz::FuzzFinding {
                path: path.clone(),
                status,
                content_length,
                content_type: content_type.clone(),
            });
            result.found_count += 1;
        }
    }

    Ok(())
}
