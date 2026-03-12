use std::process::Command;

fn entropyfa() -> Command {
    Command::new(env!("CARGO_BIN_EXE_entropyfa"))
}

fn run_ok(cmd: &mut Command) -> serde_json::Value {
    let output = cmd.output().expect("failed to execute process");
    assert_eq!(
        output.status.code(),
        Some(0),
        "expected exit code 0, stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).expect("non-utf8 stdout");
    serde_json::from_str(&stdout).expect("stdout is not valid JSON")
}

fn run_err(cmd: &mut Command) -> serde_json::Value {
    let output = cmd.output().expect("failed to execute process");
    assert_eq!(
        output.status.code(),
        Some(1),
        "expected exit code 1, got {:?}, stdout: {}",
        output.status.code(),
        String::from_utf8_lossy(&output.stdout)
    );
    let stdout = String::from_utf8(output.stdout).expect("non-utf8 stdout");
    serde_json::from_str(&stdout).expect("stdout is not valid JSON")
}

// ---------------------------------------------------------------------------
// 1. --version
// ---------------------------------------------------------------------------

#[test]
fn version_flag_shows_version() {
    let output = entropyfa()
        .arg("--version")
        .output()
        .expect("failed to execute");
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        stdout.contains("entropyfa"),
        "version output should contain 'entropyfa': {stdout}"
    );
}

// ---------------------------------------------------------------------------
// 2. --help
// ---------------------------------------------------------------------------

#[test]
fn help_flag_shows_usage() {
    let output = entropyfa()
        .arg("--help")
        .output()
        .expect("failed to execute");
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        stdout.contains("data"),
        "help should mention 'data' subcommand: {stdout}"
    );
    assert!(
        stdout.contains("compute"),
        "help should mention 'compute' subcommand: {stdout}"
    );
}

// ---------------------------------------------------------------------------
// 3. data coverage → ok:true with entries array
// ---------------------------------------------------------------------------

#[test]
fn data_coverage_returns_entries() {
    let v = run_ok(entropyfa().args(["data", "coverage"]));
    assert_eq!(v["ok"], true);
    let entries = v["data"]["entries"].as_array().expect("entries is array");
    assert!(
        !entries.is_empty(),
        "coverage should return at least one entry"
    );
    assert!(
        v["data"]["count"].as_u64().unwrap() > 0,
        "count should be > 0"
    );
}

// ---------------------------------------------------------------------------
// 4. data coverage --category tax → only tax entries
// ---------------------------------------------------------------------------

#[test]
fn data_coverage_filtered_by_category() {
    let v = run_ok(entropyfa().args(["data", "coverage", "--category", "tax"]));
    assert_eq!(v["ok"], true);
    let entries = v["data"]["entries"].as_array().expect("entries is array");
    assert!(!entries.is_empty(), "should have tax entries");
    for entry in entries {
        assert_eq!(
            entry["category"].as_str().unwrap(),
            "tax",
            "all entries should be category=tax, got: {entry}"
        );
    }
}

// ---------------------------------------------------------------------------
// 5. data lookup brackets → 7 brackets
// ---------------------------------------------------------------------------

#[test]
fn data_lookup_tax_brackets_single() {
    let v = run_ok(entropyfa().args([
        "data",
        "lookup",
        "--category",
        "tax",
        "--key",
        "brackets",
        "--filing-status",
        "single",
    ]));
    assert_eq!(v["ok"], true);
    let brackets = v["data"].as_array().expect("brackets is array");
    assert_eq!(
        brackets.len(),
        7,
        "single filing status should have 7 brackets, got {}",
        brackets.len()
    );
}

// ---------------------------------------------------------------------------
// 6. data lookup nonexistent category → ok:false
// ---------------------------------------------------------------------------

#[test]
fn data_lookup_unknown_category_returns_error() {
    let v = run_err(entropyfa().args([
        "data",
        "lookup",
        "--category",
        "nonexistent",
        "--key",
        "foo",
    ]));
    assert_eq!(v["ok"], false);
    assert!(
        v["error"]["code"].as_str().is_some(),
        "error should have a code"
    );
}

// ---------------------------------------------------------------------------
// 7. compute federal-tax --schema → schema JSON with gather_from_user
// ---------------------------------------------------------------------------

#[test]
fn compute_federal_tax_schema() {
    let v = run_ok(entropyfa().args(["compute", "federal-tax", "--schema"]));
    assert_eq!(v["ok"], true);
    assert!(
        v["data"]["gather_from_user"].is_object(),
        "schema should contain gather_from_user"
    );
    assert!(
        v["data"]["input_schema"].is_object(),
        "schema should contain input_schema"
    );
}

// ---------------------------------------------------------------------------
// 8. compute rmd --schema → schema JSON
// ---------------------------------------------------------------------------

#[test]
fn compute_rmd_schema() {
    let v = run_ok(entropyfa().args(["compute", "rmd", "--schema"]));
    assert_eq!(v["ok"], true);
    assert!(
        v["data"]["gather_from_user"].is_object(),
        "rmd schema should contain gather_from_user"
    );
}

// ---------------------------------------------------------------------------
// 9. compute federal-tax with valid JSON → ok:true
// ---------------------------------------------------------------------------

#[test]
fn compute_federal_tax_valid_input() {
    use std::io::Write;

    let input = r#"{"filing_status":"single","income":{"wages":100000}}"#;

    let mut child = entropyfa()
        .args(["compute", "federal-tax"])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("failed to spawn");

    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(input.as_bytes())
        .unwrap();

    let output = child.wait_with_output().expect("failed to wait");
    assert_eq!(
        output.status.code(),
        Some(0),
        "expected exit 0, stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8(output.stdout).unwrap();
    let v: serde_json::Value = serde_json::from_str(&stdout).expect("valid JSON");
    assert_eq!(v["ok"], true);
    assert!(
        v["data"]["total_tax"].as_f64().unwrap() > 0.0,
        "total_tax should be positive for $100k wages"
    );
    assert!(
        v["data"]["effective_rate"].as_f64().is_some(),
        "response should include effective_rate"
    );
}

// ---------------------------------------------------------------------------
// 10. compute federal-tax with invalid JSON → ok:false + exit code 1
// ---------------------------------------------------------------------------

#[test]
fn compute_federal_tax_invalid_json() {
    use std::io::Write;

    let mut child = entropyfa()
        .args(["compute", "federal-tax"])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("failed to spawn");

    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(b"this is not json")
        .unwrap();

    let output = child.wait_with_output().expect("failed to wait");
    assert_eq!(
        output.status.code(),
        Some(1),
        "expected exit 1 for bad JSON, got {:?}",
        output.status.code()
    );

    let stdout = String::from_utf8(output.stdout).unwrap();
    let v: serde_json::Value = serde_json::from_str(&stdout).expect("error envelope is valid JSON");
    assert_eq!(v["ok"], false);
    assert!(
        v["error"]["code"].as_str().is_some(),
        "error should have a code"
    );
}

// ---------------------------------------------------------------------------
// 11. compute estate-tax with valid JSON → ok:true
// ---------------------------------------------------------------------------

#[test]
fn compute_estate_tax_valid_input() {
    use std::io::Write;

    let input = r#"{"gross_estate":20000000,"deductions":{"marital":0,"charitable":0,"debts_and_expenses":100000}}"#;

    let mut child = entropyfa()
        .args(["compute", "estate-tax"])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("failed to spawn");

    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(input.as_bytes())
        .unwrap();

    let output = child.wait_with_output().expect("failed to wait");
    assert_eq!(
        output.status.code(),
        Some(0),
        "expected exit 0, stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8(output.stdout).unwrap();
    let v: serde_json::Value = serde_json::from_str(&stdout).expect("valid JSON");
    assert_eq!(v["ok"], true);
    assert!(
        v["data"]["net_estate_tax"].as_f64().is_some(),
        "response should include net_estate_tax"
    );
}

// ---------------------------------------------------------------------------
// 12. Each --schema command works
// ---------------------------------------------------------------------------

#[test]
fn compute_estate_tax_schema() {
    let v = run_ok(entropyfa().args(["compute", "estate-tax", "--schema"]));
    assert_eq!(v["ok"], true);
    assert!(
        v["data"]["gather_from_user"].is_object(),
        "estate-tax schema should contain gather_from_user"
    );
}

#[test]
fn compute_pension_schema() {
    let v = run_ok(entropyfa().args(["compute", "pension", "--schema"]));
    assert_eq!(v["ok"], true);
    assert!(
        v["data"]["gather_from_user"].is_object(),
        "pension schema should contain gather_from_user"
    );
}

#[test]
fn compute_simulate_schema() {
    let v = run_ok(entropyfa().args(["compute", "simulate", "--schema"]));
    assert_eq!(v["ok"], true);
    assert!(
        v["data"]["gather_from_user"].is_object(),
        "simulate schema should contain gather_from_user"
    );
}

#[test]
fn compute_solve_schema() {
    let v = run_ok(entropyfa().args(["compute", "solve", "--schema"]));
    assert_eq!(v["ok"], true);
    assert!(
        v["data"]["gather_from_user"].is_object(),
        "solve schema should contain gather_from_user"
    );
}

#[test]
fn compute_roth_schema() {
    let v = run_ok(entropyfa().args(["compute", "roth", "--schema"]));
    assert_eq!(v["ok"], true);
    assert!(
        v["data"]["gather_from_user"].is_object(),
        "roth schema should contain gather_from_user"
    );
}

#[test]
fn compute_roth_strategy_schema() {
    let v = run_ok(entropyfa().args(["compute", "roth-strategy", "--schema"]));
    assert_eq!(v["ok"], true);
    assert!(
        v["data"]["gather_from_user"].is_object(),
        "roth-strategy schema should contain gather_from_user"
    );
}

#[test]
fn compute_rmd_schedule_schema() {
    let v = run_ok(entropyfa().args(["compute", "rmd-schedule", "--schema"]));
    assert_eq!(v["ok"], true);
    assert!(
        v["data"]["gather_from_user"].is_object(),
        "rmd-schedule schema should contain gather_from_user"
    );
}
