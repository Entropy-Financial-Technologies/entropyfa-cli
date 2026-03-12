use serde_json::{json, Value};

pub fn print_success(data: Value) {
    let envelope = json!({ "ok": true, "data": data });
    let json_str = serde_json::to_string(&envelope).unwrap_or_else(|e| {
        format!(
            "{{\"ok\":false,\"error\":{{\"code\":\"serialization_error\",\"message\":\"{}\"}}}}",
            e
        )
    });
    println!("{}", json_str);
}

pub fn print_error(code: &str, message: &str) {
    let envelope = json!({
        "ok": false,
        "error": { "code": code, "message": message }
    });
    let json_str = serde_json::to_string(&envelope).unwrap_or_else(|_| {
        format!(
            "{{\"ok\":false,\"error\":{{\"code\":\"{}\",\"message\":\"{}\"}}}}",
            code, message
        )
    });
    println!("{}", json_str);
}
