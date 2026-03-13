use std::time::Duration;

use reqwest::header::CONTENT_TYPE;
use serde_json::Value;

const RESULT_HOOK_TIMEOUT_SECS: u64 = 10;
const RESPONSE_BODY_LIMIT: usize = 200;

pub fn post_result(url: &str, envelope: &Value) -> Result<(), String> {
    if url.trim().is_empty() {
        return Err("hook URL is empty".to_string());
    }

    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(RESULT_HOOK_TIMEOUT_SECS))
        .user_agent(format!("entropyfa/{}", env!("CARGO_PKG_VERSION")))
        .build()
        .map_err(|err| format!("failed to build HTTP client: {err}"))?;

    let response = client
        .post(url)
        .header(CONTENT_TYPE, "application/json")
        .json(envelope)
        .send()
        .map_err(|err| format!("request error: {err}"))?;

    let status = response.status();
    if status.is_success() {
        return Ok(());
    }

    let body = response
        .text()
        .map(|text| truncate(text.trim(), RESPONSE_BODY_LIMIT))
        .unwrap_or_default();

    if body.is_empty() {
        Err(format!("HTTP {status}"))
    } else {
        Err(format!("HTTP {status}: {body}"))
    }
}

fn truncate(text: &str, max_len: usize) -> String {
    let mut output = String::new();
    for ch in text.chars().take(max_len) {
        output.push(ch);
    }
    output
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::{post_result, truncate};

    #[test]
    fn rejects_empty_hook_url() {
        let error = post_result("   ", &json!({ "ok": true })).expect_err("empty URL fails");
        assert_eq!(error, "hook URL is empty");
    }

    #[test]
    fn truncates_by_char_boundary() {
        assert_eq!(truncate("abcdef", 3), "abc");
        assert_eq!(truncate("a🙂b🙂c", 4), "a🙂b🙂");
    }
}
