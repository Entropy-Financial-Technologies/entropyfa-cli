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
    assert!(
        stdout.contains("update"),
        "help should mention 'update' alias: {stdout}"
    );
}

#[test]
fn update_alias_help_works() {
    let output = entropyfa()
        .args(["update", "--help"])
        .output()
        .expect("failed to execute");
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        stdout.contains("Update entropyfa to the latest version"),
        "update alias help should describe the upgrade command: {stdout}"
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
        "federal_income_tax_brackets",
        "--filing-status",
        "single",
    ]));
    assert_eq!(v["ok"], true);
    assert_eq!(v["data"]["category"], "tax");
    assert_eq!(v["data"]["key"], "federal_income_tax_brackets");
    assert_eq!(v["data"]["year"], 2026);
    assert_eq!(v["data"]["verification_status"], "authoritative");
    assert_eq!(v["data"]["pipeline_reviewed"], true);
    let sources = v["data"]["sources"].as_array().expect("sources is array");
    assert!(!sources.is_empty(), "sources should not be empty");
    assert!(
        sources[0]["url"].is_string(),
        "reviewed source should include a URL"
    );
    let brackets = v["data"]["value"].as_array().expect("brackets is array");
    assert_eq!(
        brackets.len(),
        7,
        "single filing status should have 7 brackets, got {}",
        brackets.len()
    );
}

#[test]
fn data_lookup_medicare_base_premiums() {
    let v = run_ok(entropyfa().args([
        "data",
        "lookup",
        "--category",
        "insurance",
        "--key",
        "medicare_base_premiums",
    ]));
    assert_eq!(v["ok"], true);
    assert_eq!(v["data"]["category"], "insurance");
    assert_eq!(v["data"]["key"], "medicare_base_premiums");
    assert_eq!(v["data"]["verification_status"], "authoritative");
    assert_eq!(v["data"]["pipeline_reviewed"], true);
    let sources = v["data"]["sources"].as_array().expect("sources is array");
    assert!(!sources.is_empty(), "sources should not be empty");
    assert_eq!(v["data"]["value"]["part_b_standard_monthly_premium"], 202.9);
    assert_eq!(v["data"]["value"]["part_b_annual_deductible"], 283.0);
    assert_eq!(v["data"]["value"]["part_d_base_beneficiary_premium"], 38.99);
}

#[test]
fn data_lookup_full_retirement_age_rules() {
    let v = run_ok(entropyfa().args([
        "data",
        "lookup",
        "--category",
        "social_security",
        "--key",
        "full_retirement_age_rules",
    ]));
    assert_eq!(v["ok"], true);
    assert_eq!(v["data"]["category"], "social_security");
    assert_eq!(v["data"]["key"], "full_retirement_age_rules");
    assert_eq!(v["data"]["verification_status"], "authoritative");
    assert_eq!(v["data"]["pipeline_reviewed"], true);
    assert_eq!(
        v["data"]["value"]["benefit_scope"],
        "retirement_and_spousal"
    );
    assert_eq!(v["data"]["value"]["january_1_births_use_prior_year"], true);
    let rules = v["data"]["value"]["rules"]
        .as_array()
        .expect("rules is array");
    assert_eq!(rules.len(), 13);
    assert_eq!(rules[0]["birth_year_max"], 1937);
    assert_eq!(rules[12]["birth_year_min"], 1960);
    assert_eq!(rules[12]["full_retirement_age_years"], 67);
}

#[test]
fn data_lookup_irmaa_mfs_lived_apart() {
    let v = run_ok(entropyfa().args([
        "data",
        "lookup",
        "--category",
        "insurance",
        "--key",
        "irmaa_brackets",
        "--filing-status",
        "married_filing_separately",
        "--lived-with-spouse-during-year",
        "false",
    ]));
    assert_eq!(v["ok"], true);
    assert_eq!(v["data"]["category"], "insurance");
    assert_eq!(v["data"]["key"], "irmaa_brackets");
    assert_eq!(v["data"]["verification_status"], "authoritative");
    assert_eq!(v["data"]["pipeline_reviewed"], true);
    assert_eq!(
        v["data"]["value"]["filing_status"],
        "married_filing_separately"
    );
    assert_eq!(v["data"]["value"]["lived_with_spouse_during_year"], false);
    let sources = v["data"]["sources"].as_array().expect("sources is array");
    assert!(!sources.is_empty(), "sources should not be empty");
    assert!(
        sources[0]["url"].is_string(),
        "reviewed source should include a URL"
    );
    let brackets = v["data"]["value"]["brackets"]
        .as_array()
        .expect("brackets is array");
    assert_eq!(brackets.len(), 6);
}

#[test]
fn data_lookup_irmaa_mfs_requires_spouse_flag() {
    let v = run_err(entropyfa().args([
        "data",
        "lookup",
        "--category",
        "insurance",
        "--key",
        "irmaa_brackets",
        "--filing-status",
        "married_filing_separately",
    ]));
    assert_eq!(v["ok"], false);
    assert_eq!(v["error"]["code"], "lookup_error");
    assert!(v["error"]["message"]
        .as_str()
        .unwrap()
        .contains("lived_with_spouse_during_year"));
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
// 9. compute federal-tax with --json → ok:true
// ---------------------------------------------------------------------------

#[test]
fn compute_federal_tax_valid_input() {
    let input = r#"{"filing_status":"single","income":{"wages":100000}}"#;
    let v = run_ok(entropyfa().args(["compute", "federal-tax", "--json", input]));
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
// 10. compute federal-tax without --json → ok:false + exit code 1
// ---------------------------------------------------------------------------

#[test]
fn compute_federal_tax_missing_json_flag() {
    let v = run_err(entropyfa().args(["compute", "federal-tax"]));
    assert_eq!(v["ok"], false);
    assert_eq!(v["error"]["code"], "missing_json");
}

// ---------------------------------------------------------------------------
// 11. compute federal-tax with invalid --json → ok:false + exit code 1
// ---------------------------------------------------------------------------

#[test]
fn compute_federal_tax_invalid_json() {
    let v = run_err(entropyfa().args(["compute", "federal-tax", "--json", "not-json"]));
    assert_eq!(v["ok"], false);
    assert_eq!(v["error"]["code"], "invalid_json");
}

// ---------------------------------------------------------------------------
// 12. compute estate-tax with --json → ok:true
// ---------------------------------------------------------------------------

#[test]
fn compute_estate_tax_valid_input() {
    let input = r#"{"gross_estate":20000000,"deductions":{"marital":0,"charitable":0,"debts_and_expenses":100000}}"#;
    let v = run_ok(entropyfa().args(["compute", "estate-tax", "--json", input]));
    assert_eq!(v["ok"], true);
    assert!(
        v["data"]["net_estate_tax"].as_f64().is_some(),
        "response should include net_estate_tax"
    );
}

// ---------------------------------------------------------------------------
// 13. Each --schema command works (renamed commands)
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
fn compute_pension_comparison_schema() {
    let v = run_ok(entropyfa().args(["compute", "pension-comparison", "--schema"]));
    assert_eq!(v["ok"], true);
    assert!(
        v["data"]["gather_from_user"].is_object(),
        "pension-comparison schema should contain gather_from_user"
    );
}

#[test]
fn compute_projection_schema() {
    let v = run_ok(entropyfa().args(["compute", "projection", "--schema"]));
    assert_eq!(v["ok"], true);
    assert!(
        v["data"]["gather_from_user"].is_object(),
        "projection schema should contain gather_from_user"
    );
    let when_to_use = v["data"]["when_to_use"]
        .as_str()
        .expect("when_to_use should be a string");
    assert!(
        when_to_use.contains("--visual"),
        "projection schema should mention --visual: {when_to_use}"
    );
}

#[test]
fn compute_projection_schema_mentions_bucketed_and_legacy_requests() {
    let v = run_ok(entropyfa().args(["compute", "projection", "--schema"]));
    assert_eq!(v["ok"], true);

    let when_to_use = v["data"]["when_to_use"]
        .as_str()
        .expect("when_to_use should be a string");
    assert!(
        when_to_use.contains("bucket"),
        "projection schema should mention bucketed requests: {when_to_use}"
    );
    assert!(
        when_to_use.contains("legacy"),
        "projection schema should mention legacy compatibility: {when_to_use}"
    );

    let properties = v["data"]["input_schema"]["properties"]
        .as_object()
        .expect("input_schema.properties should be an object");
    for key in [
        "buckets",
        "filing_status",
        "household",
        "spending_policy",
        "tax_policy",
        "rmd_policy",
        "starting_balance",
        "return_assumption",
    ] {
        assert!(
            properties.contains_key(key),
            "projection schema should expose {key}"
        );
    }

    let tax_policy = v["data"]["input_schema"]["properties"]["tax_policy"]
        .as_object()
        .expect("tax_policy should be an object");
    let tax_policy_description = tax_policy
        .get("description")
        .and_then(|value| value.as_str())
        .expect("tax_policy description should be a string");
    assert!(
        tax_policy_description.contains("embedded data"),
        "tax_policy should mention embedded data: {tax_policy_description}"
    );
    assert!(
        tax_policy_description.contains("modeled"),
        "tax_policy should mention modeled behavior after supported years: {tax_policy_description}"
    );

    let filing_status = v["data"]["input_schema"]["properties"]["filing_status"]
        .as_object()
        .expect("filing_status should be an object");
    let filing_status_description = filing_status
        .get("description")
        .and_then(|value| value.as_str())
        .expect("filing_status description should be a string");
    assert!(
        filing_status_description.contains("tax"),
        "filing_status should explain why it matters: {filing_status_description}"
    );

    let household = v["data"]["input_schema"]["properties"]["household"]
        .as_object()
        .expect("household should be an object");
    let household_description = household
        .get("description")
        .and_then(|value| value.as_str())
        .expect("household description should be a string");
    assert!(
        household_description.contains("RMD"),
        "household should explain when it matters: {household_description}"
    );

    let terminal_dashboard = v["data"]["output_summary"]["terminal_dashboard"]
        .as_str()
        .expect("terminal_dashboard summary should be a string");
    assert!(
        terminal_dashboard.contains("aggregate"),
        "projection schema should note that the terminal dashboard stays aggregate-only: {terminal_dashboard}"
    );
}

#[test]
fn compute_projection_help_shows_visual_flag() {
    let output = entropyfa()
        .args(["compute", "projection", "--help"])
        .output()
        .expect("failed to execute");
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("help stdout should be utf-8");
    assert!(
        stdout.contains("--visual"),
        "projection help should mention --visual: {stdout}"
    );
    assert!(
        !stdout.contains("--tui"),
        "projection help should not mention removed alias --tui: {stdout}"
    );
}

#[test]
fn compute_projection_does_not_auto_add_visual_percentiles() {
    let v = run_ok(entropyfa().args([
        "compute",
        "projection",
        "--json",
        "{\"starting_balance\":1000,\"time_horizon_months\":12,\"return_assumption\":{\"annual_mean\":0.05,\"annual_std_dev\":0.1},\"num_simulations\":10,\"seed\":1,\"mode\":\"monte_carlo\"}",
    ]));
    assert_eq!(v["ok"], true);
    assert!(
        v["data"]["request_echo"]["custom_percentiles"].is_null(),
        "custom_percentiles should stay null unless --visual or --percentiles is requested: {}",
        v
    );
}

#[test]
fn compute_projection_bucketed_request_returns_bucket_aware_output() {
    let v = run_ok(entropyfa().args([
        "compute",
        "projection",
        "--json",
        "{\"buckets\":[{\"id\":\"taxable\",\"bucket_type\":\"taxable\",\"starting_balance\":60000,\"return_assumption\":{\"annual_mean\":0.06,\"annual_std_dev\":0.1}},{\"id\":\"ira\",\"bucket_type\":\"tax_deferred\",\"starting_balance\":40000,\"return_assumption\":{\"annual_mean\":0.05,\"annual_std_dev\":0.08}}],\"time_horizon_months\":12,\"spending_policy\":{\"withdrawal_order\":[\"taxable\",\"ira\"]},\"num_simulations\":32,\"seed\":7}",
    ]));

    assert_eq!(v["ok"], true);
    assert_eq!(
        v["data"]["request_echo"]["buckets"]
            .as_array()
            .unwrap()
            .len(),
        2
    );
    assert!(
        v["data"]["monte_carlo"]["bucket_terminal_percentiles"].is_object(),
        "bucketed projection should include Monte Carlo bucket summaries"
    );
    assert!(
        v["data"]["monte_carlo"]["bucket_depletion_counts"].is_object(),
        "bucketed projection should include bucket depletion counts"
    );
    assert!(
        v["data"]["linear"]["ending_balances_by_bucket"].is_object(),
        "bucketed projection should include linear bucket balances"
    );
    assert!(
        v["data"]["monte_carlo"]["bucket_terminal_percentiles"]["__household_cash__"].is_object(),
        "bucketed projection should include the synthetic household cash summary bucket"
    );
}

#[test]
fn compute_projection_legacy_request_still_works() {
    let v = run_ok(entropyfa().args([
        "compute",
        "projection",
        "--json",
        "{\"starting_balance\":1000,\"time_horizon_months\":12,\"return_assumption\":{\"annual_mean\":0.05,\"annual_std_dev\":0.1},\"num_simulations\":10,\"seed\":1,\"mode\":\"monte_carlo\"}",
    ]));

    assert_eq!(v["ok"], true);
    assert_eq!(
        v["data"]["request_echo"]["starting_balance"].as_f64(),
        Some(1000.0)
    );
    assert!(
        v["data"]["request_echo"]["buckets"]
            .as_array()
            .expect("buckets should be an array")
            .is_empty(),
        "legacy projection should not echo any buckets"
    );
    assert!(
        v["data"]["monte_carlo"]["bucket_terminal_percentiles"].is_null(),
        "legacy projection should not include bucket summaries"
    );
}

#[test]
fn compute_projection_rejects_reserved_household_cash_bucket_id() {
    let v = run_err(entropyfa().args([
        "compute",
        "projection",
        "--json",
        "{\"buckets\":[{\"id\":\"__household_cash__\",\"bucket_type\":\"taxable\",\"starting_balance\":60000,\"return_assumption\":{\"annual_mean\":0.06,\"annual_std_dev\":0.1}},{\"id\":\"ira\",\"bucket_type\":\"tax_deferred\",\"starting_balance\":40000,\"return_assumption\":{\"annual_mean\":0.05,\"annual_std_dev\":0.08}}],\"time_horizon_months\":12,\"spending_policy\":{\"withdrawal_order\":[\"__household_cash__\",\"ira\"],\"rebalance_tax_withholding_from\":\"__household_cash__\"},\"num_simulations\":32,\"seed\":7}",
    ]));

    assert_eq!(v["ok"], false);
    assert_eq!(v["error"]["code"], "validation_error");
    assert!(
        v["error"]["message"]
            .as_str()
            .expect("error message should be a string")
            .contains("__household_cash__"),
        "reserved bucket id should be rejected"
    );
}

#[test]
fn compute_projection_rejects_invalid_filing_status_for_tax_enabled_bucketed_requests() {
    let v = run_err(entropyfa().args([
        "compute",
        "projection",
        "--json",
        "{\"buckets\":[{\"id\":\"taxable\",\"bucket_type\":\"taxable\",\"starting_balance\":60000,\"return_assumption\":{\"annual_mean\":0.06,\"annual_std_dev\":0.1}},{\"id\":\"ira\",\"bucket_type\":\"tax_deferred\",\"starting_balance\":40000,\"return_assumption\":{\"annual_mean\":0.05,\"annual_std_dev\":0.08}}],\"time_horizon_months\":12,\"spending_policy\":{\"withdrawal_order\":[\"taxable\",\"ira\"]},\"tax_policy\":{\"mode\":\"modeled\"},\"filing_status\":\"invalid\",\"num_simulations\":32,\"seed\":7}",
    ]));

    assert_eq!(v["ok"], false);
    assert_eq!(v["error"]["code"], "validation_error");
}

#[test]
fn compute_projection_visual_adds_dashboard_percentiles() {
    let v = run_ok(entropyfa().args([
        "compute",
        "projection",
        "--visual",
        "--json",
        "{\"starting_balance\":1000,\"time_horizon_months\":12,\"return_assumption\":{\"annual_mean\":0.05,\"annual_std_dev\":0.1},\"num_simulations\":10,\"seed\":1,\"mode\":\"monte_carlo\"}",
    ]));
    assert_eq!(v["ok"], true);
    assert_eq!(
        v["data"]["request_echo"]["custom_percentiles"],
        serde_json::json!([20, 80])
    );
}

#[test]
fn compute_goal_solver_schema() {
    let v = run_ok(entropyfa().args(["compute", "goal-solver", "--schema"]));
    assert_eq!(v["ok"], true);
    assert!(
        v["data"]["gather_from_user"].is_object(),
        "goal-solver schema should contain gather_from_user"
    );
}

#[test]
fn compute_roth_conversion_schema() {
    let v = run_ok(entropyfa().args(["compute", "roth-conversion", "--schema"]));
    assert_eq!(v["ok"], true);
    assert!(
        v["data"]["gather_from_user"].is_object(),
        "roth-conversion schema should contain gather_from_user"
    );
}

#[test]
fn compute_roth_conversion_strategy_schema() {
    let v = run_ok(entropyfa().args(["compute", "roth-conversion-strategy", "--schema"]));
    assert_eq!(v["ok"], true);
    assert!(
        v["data"]["gather_from_user"].is_object(),
        "roth-conversion-strategy schema should contain gather_from_user"
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
