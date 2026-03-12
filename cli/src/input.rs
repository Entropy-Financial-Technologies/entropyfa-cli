use crate::output;
use serde_json::Value;

/// Parse the `--json` CLI argument. Required for all compute commands.
/// Prints a JSON error envelope and exits if missing or invalid.
pub fn parse_json_arg(json_input: Option<String>, command_name: &str) -> Value {
    match json_input {
        Some(s) => match serde_json::from_str(&s) {
            Ok(v) => v,
            Err(e) => {
                output::print_error(
                    "invalid_json",
                    &format!(
                        "invalid JSON for `{}`: {}\n\nUsage: entropyfa compute {} --json '<JSON>'",
                        command_name, e, command_name
                    ),
                );
                std::process::exit(1);
            }
        },
        None => {
            output::print_error(
                "missing_json",
                &format!(
                    "--json is required for `{}`\n\nUsage: entropyfa compute {} --json '<JSON>'",
                    command_name, command_name
                ),
            );
            std::process::exit(1);
        }
    }
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
