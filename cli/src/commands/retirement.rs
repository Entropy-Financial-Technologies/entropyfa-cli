use crate::{assembler, input, output, schema};

pub fn run_rmd(schema_flag: bool, json_input: Option<String>) {
    if schema_flag {
        output::print_success(schema::retirement::rmd_schema());
        return;
    }

    let input = input::parse_json_arg(json_input, "rmd");
    match assembler::retirement::assemble_rmd(&input) {
        Ok(req) => {
            let errors = entropyfa_engine::validation::validate_retirement_rmd_request(&req);
            if !errors.is_empty() {
                output::print_error("validation_error", &errors.join("; "));
                std::process::exit(1);
            }
            match entropyfa_engine::compute::retirement::rmd::run_retirement_rmd(&req) {
                Ok(result) => output::print_success(
                    serde_json::to_value(&result).expect("serialize RmdResponse"),
                ),
                Err(e) => {
                    output::print_error("compute_error", &e);
                    std::process::exit(1);
                }
            }
        }
        Err(e) => {
            output::print_error(classify_assembly_error(&e), &e);
            std::process::exit(1);
        }
    }
}

pub fn run_rmd_schedule(schema_flag: bool, json_input: Option<String>) {
    if schema_flag {
        output::print_success(schema::retirement::rmd_schedule_schema());
        return;
    }

    let input = input::parse_json_arg(json_input, "rmd-schedule");
    match assembler::retirement::assemble_rmd_schedule(&input) {
        Ok(req) => {
            let errors =
                entropyfa_engine::validation::validate_retirement_rmd_schedule_request(&req);
            if !errors.is_empty() {
                output::print_error("validation_error", &errors.join("; "));
                std::process::exit(1);
            }
            match entropyfa_engine::compute::retirement::rmd::run_retirement_rmd_schedule(&req) {
                Ok(result) => output::print_success(
                    serde_json::to_value(&result).expect("serialize RmdScheduleResponse"),
                ),
                Err(e) => {
                    output::print_error("compute_error", &e);
                    std::process::exit(1);
                }
            }
        }
        Err(e) => {
            output::print_error(classify_assembly_error(&e), &e);
            std::process::exit(1);
        }
    }
}

pub fn run_roth(schema_flag: bool, json_input: Option<String>) {
    if schema_flag {
        output::print_success(schema::retirement::roth_schema());
        return;
    }

    let input = input::parse_json_arg(json_input, "roth-conversion");
    match assembler::retirement::assemble_roth(&input) {
        Ok(req) => {
            let errors = entropyfa_engine::validation::validate_roth_conversion_request(&req);
            if !errors.is_empty() {
                output::print_error("validation_error", &errors.join("; "));
                std::process::exit(1);
            }
            let result =
                entropyfa_engine::compute::retirement::roth_conversion::analyze_roth_conversion(
                    &req,
                );
            output::print_success(
                serde_json::to_value(&result).expect("serialize RothConversionResponse"),
            );
        }
        Err(e) => {
            output::print_error("assembly_error", &e);
            std::process::exit(1);
        }
    }
}

pub fn run_roth_strategy(schema_flag: bool, json_input: Option<String>) {
    if schema_flag {
        output::print_success(schema::retirement::roth_strategy_schema());
        return;
    }

    let input = input::parse_json_arg(json_input, "roth-conversion-strategy");
    match assembler::retirement::assemble_roth_strategy(&input) {
        Ok(req) => {
            let errors =
                entropyfa_engine::validation::validate_roth_conversion_strategy_request(&req);
            if !errors.is_empty() {
                output::print_error("validation_error", &errors.join("; "));
                std::process::exit(1);
            }
            match entropyfa_engine::compute::retirement::roth_conversion::compute_roth_conversion_strategy(&req) {
                Ok(result) => output::print_success(
                    serde_json::to_value(&result).expect("serialize RothStrategyResponse"),
                ),
                Err(e) => {
                    output::print_error("compute_error", &e);
                    std::process::exit(1);
                }
            }
        }
        Err(e) => {
            output::print_error("assembly_error", &e);
            std::process::exit(1);
        }
    }
}

fn classify_assembly_error(error: &str) -> &'static str {
    if error.starts_with("reference_pack_missing:") {
        "reference_pack_missing"
    } else if error.starts_with("reference_pack_invalid:") {
        "reference_pack_invalid"
    } else {
        "assembly_error"
    }
}
