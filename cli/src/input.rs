use serde_json::Value;
use std::io::{self, IsTerminal, Read};

/// Read JSON from stdin if piped; returns None if stdin is a terminal or empty.
pub fn read_stdin_json() -> Option<Value> {
    if io::stdin().is_terminal() {
        return None;
    }
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).ok()?;
    let trimmed = buf.trim();
    if trimmed.is_empty() {
        return None;
    }
    serde_json::from_str(trimmed).ok()
}

/// Deep-merge `overrides` into `base`. Object keys in overrides win;
/// non-object values in overrides replace base entirely.
#[allow(dead_code)]
pub fn merge_json(base: Option<Value>, overrides: Value) -> Value {
    match base {
        None => overrides,
        Some(Value::Object(mut base_map)) => {
            if let Value::Object(over_map) = overrides {
                for (k, v) in over_map {
                    let merged = if let Some(existing) = base_map.remove(&k) {
                        merge_json(Some(existing), v)
                    } else {
                        v
                    };
                    base_map.insert(k, merged);
                }
                Value::Object(base_map)
            } else {
                overrides
            }
        }
        Some(_) => overrides,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn merge_none_base() {
        let result = merge_json(None, json!({"a": 1}));
        assert_eq!(result, json!({"a": 1}));
    }

    #[test]
    fn merge_deep() {
        let base = json!({"income": {"wages": 100000}, "year": 2025});
        let over = json!({"income": {"interest": 5000}, "year": 2026});
        let result = merge_json(Some(base), over);
        assert_eq!(result["income"]["wages"], 100000);
        assert_eq!(result["income"]["interest"], 5000);
        assert_eq!(result["year"], 2026);
    }
}
