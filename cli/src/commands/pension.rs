use serde_json::json;

use crate::{assembler, input, output, schema};

pub fn run_pension(schema_flag: bool) {
    if schema_flag {
        output::print_success(schema::pension::pension_schema());
        return;
    }

    let input = input::read_stdin_json().unwrap_or(json!({}));
    match assembler::pension::assemble_pension(&input) {
        Ok(req) => {
            let errors = entropyfa_engine::validation::validate_pension_comparison_request(&req);
            if !errors.is_empty() {
                output::print_error("validation_error", &errors.join("; "));
                std::process::exit(1);
            }
            let result =
                entropyfa_engine::compute::pension::comparison::run_pension_comparison(&req);
            output::print_success(
                serde_json::to_value(&result).expect("serialize PensionComparisonResponse"),
            );
        }
        Err(e) => {
            output::print_error("assembly_error", &e);
            std::process::exit(1);
        }
    }
}
