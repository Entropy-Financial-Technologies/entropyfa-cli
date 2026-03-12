use serde_json::json;

use crate::{assembler, input, output, schema};

pub fn run_federal_tax(schema_flag: bool) {
    if schema_flag {
        output::print_success(schema::tax::federal_tax_schema());
        return;
    }

    let input = input::read_stdin_json().unwrap_or(json!({}));
    match assembler::tax::assemble_federal_tax(&input) {
        Ok(req) => {
            let errors = entropyfa_engine::validation::validate_federal_tax_request(&req);
            if !errors.is_empty() {
                output::print_error("validation_error", &errors.join("; "));
                std::process::exit(1);
            }
            let result = entropyfa_engine::compute::tax::federal::run_federal_tax(&req);
            output::print_success(
                serde_json::to_value(&result).expect("serialize FederalTaxResponse"),
            );
        }
        Err(e) => {
            output::print_error("assembly_error", &e);
            std::process::exit(1);
        }
    }
}

pub fn run_estate_tax(schema_flag: bool) {
    if schema_flag {
        output::print_success(schema::tax::estate_tax_schema());
        return;
    }

    let input = input::read_stdin_json().unwrap_or(json!({}));
    match assembler::tax::assemble_estate_tax(&input) {
        Ok(req) => {
            let errors = entropyfa_engine::validation::validate_estate_tax_request(&req);
            if !errors.is_empty() {
                output::print_error("validation_error", &errors.join("; "));
                std::process::exit(1);
            }
            let result = entropyfa_engine::compute::tax::estate::run_estate_tax(&req);
            output::print_success(
                serde_json::to_value(&result).expect("serialize EstateTaxResponse"),
            );
        }
        Err(e) => {
            output::print_error("assembly_error", &e);
            std::process::exit(1);
        }
    }
}
