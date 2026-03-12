pub mod pension;
pub mod retirement;
pub mod simulation;
pub mod tax;

use entropyfa_engine::data::types::{DataError, FilingStatus};

/// Parse filing_status from JSON input, returning a friendly error.
pub fn parse_filing_status(input: &serde_json::Value) -> Result<FilingStatus, String> {
    let s = input["filing_status"]
        .as_str()
        .ok_or_else(|| "missing required field: filing_status".to_string())?;
    FilingStatus::parse(s).map_err(|e: DataError| e.to_string())
}
