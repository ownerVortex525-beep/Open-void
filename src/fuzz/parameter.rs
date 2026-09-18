use crate::cli::banner;
use crate::http::client::HttpClient;
use crate::fuzz::FuzzResult;
use std::time::Duration;

static COMMON_PARAMS: &[&str] = &[
    "id", "user", "name", "page", "cat", "category", "q", "query",
    "search", "view", "include", "page_id", "file", "lang", "p", "pid",
    "item", "itemid", "categoryid", "catid", "product", "prod", "id",
    "uid", "userid", "username", "user_id", "doc", "docid", "doc_id",
    "mod", "module", "action", "do", "exe", "command", "option",
    "redirect", "url", "next", "dest", "destination", "continue",
    "return", "ref", "referer", "site", "src", "path", "pg", "php",
];

static TEST_VALUES: &[&str] = &["1", "0", "true", "false", "a", "test"];

pub async fn fuzz_parameters(
    url: &str,
    result: &mut FuzzResult,
    client: &HttpClient
) -> anyhow::Result<()> {
    banner::print_scanning("Parameter Discovery", url);

    let has_query = url.contains('?');
    let base_url = if has_query {
        url.split('?').next().unwrap_or(url)
    } else {
        url
    };

    let client_clone = client;

    let futures: Vec<_> = COMMON_PARAMS.iter().flat_map(|param| {
        TEST_VALUES.iter().map(move |value| {
            let test_url = if has_query {
                format!("{}&{}={}", base_url, param, value)
            } else {
                format!("{}?{}={}", base_url, param, value)
            };
            let c = client_clone;
            let param = param.to_string();
            let value = value.to_string();
            async move {
                let req = c.get(&test_url);
                match tokio::time::timeout(Duration::from_secs(2), req).await {
                    Ok(Ok(response)) => {
                        let status = response.status().as_u16();
                        let content_length = response.content_length().unwrap_or(0) as usize;
                        (param, value, status, content_length)
                    }
                    _ => (param, value, 0, 0),
                }
            }
        })
    }).collect();

    use futures::StreamExt;
    let stream = futures::stream::iter(futures).buffer_unordered(10);
    tokio::pin!(stream);

    while let Some((param, value, status, content_length)) = stream.next().await {
        result.total_requests += 1;
        if status != 404 && status != 0 && content_length > 0 {
            banner::success(&format!("Parameter reflecting: {}={}", param, value));
            result.found_params.push(crate::fuzz::FuzzFinding {
                path: format!("{}={}", param, value),
                status,
                content_length,
                content_type: String::new(),
            });
            result.found_count += 1;
        }
    }

    Ok(())
}
