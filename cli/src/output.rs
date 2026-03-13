use std::sync::OnceLock;

use serde_json::{json, Value};

static RESULT_HOOK_URL: OnceLock<Option<String>> = OnceLock::new();

pub fn set_result_hook_url(url: Option<String>) {
    let _ = RESULT_HOOK_URL.set(url);
}

pub fn success_envelope(data: Value) -> Value {
    json!({ "ok": true, "data": data })
}

pub fn error_envelope(code: &str, message: &str) -> Value {
    json!({
        "ok": false,
        "error": { "code": code, "message": message }
    })
}

pub fn print_envelope(envelope: &Value) {
    let json_str = serde_json::to_string(envelope).unwrap_or_else(|e| {
        format!(
            "{{\"ok\":false,\"error\":{{\"code\":\"serialization_error\",\"message\":\"{}\"}}}}",
            e
        )
    });
    println!("{}", json_str);

    if let Some(url) = RESULT_HOOK_URL.get().and_then(|url| url.as_deref()) {
        if let Err(err) = crate::webhook::post_result(url, envelope) {
            eprintln!("warning: failed to POST result to {url}: {err}");
        }
    }
}

pub fn print_success(data: Value) {
    print_envelope(&success_envelope(data));
}

pub fn print_error(code: &str, message: &str) {
    print_envelope(&error_envelope(code, message));
}
