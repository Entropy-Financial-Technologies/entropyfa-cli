use crate::data::types::{DataError, FilingStatus};
use crate::models::tax_request::{NiitParams, PayrollParams, SaltDeductionParams, TaxBracket};

pub fn capital_gains_brackets_for_year(
    year: u32,
    status: FilingStatus,
) -> Result<Vec<TaxBracket>, DataError> {
    match year {
        2026 => Ok(capital_gains_brackets(status)),
        _ => Err(DataError::UnsupportedYear(year)),
    }
}

pub fn standard_deductions_for_year(year: u32, status: FilingStatus) -> Result<f64, DataError> {
    match year {
        2026 => Ok(standard_deductions(status)),
        _ => Err(DataError::UnsupportedYear(year)),
    }
}

pub fn niit_for_year(year: u32, status: FilingStatus) -> Result<NiitParams, DataError> {
    match year {
        2026 => Ok(niit(status)),
        _ => Err(DataError::UnsupportedYear(year)),
    }
}

pub fn payroll_for_year(year: u32, status: FilingStatus) -> Result<PayrollParams, DataError> {
    match year {
        2026 => Ok(payroll(status)),
        _ => Err(DataError::UnsupportedYear(year)),
    }
}

pub fn capital_loss_limit_for_year(year: u32, status: FilingStatus) -> Result<f64, DataError> {
    match year {
        2026 => Ok(capital_loss_limit(status)),
        _ => Err(DataError::UnsupportedYear(year)),
    }
}

pub fn salt_deduction_parameters_for_year(
    year: u32,
    status: FilingStatus,
) -> Result<SaltDeductionParams, DataError> {
    match year {
        2026 => Ok(salt_deduction_parameters(status)),
        _ => Err(DataError::UnsupportedYear(year)),
    }
}

// ---------------------------------------------------------------------------
// Federal income tax brackets (2025-2026, reviewed artifacts)
// ---------------------------------------------------------------------------

pub fn brackets(status: FilingStatus) -> Vec<TaxBracket> {
    brackets_for_year(2026, status).expect("default federal tax brackets year is supported")
}

pub fn brackets_for_year(year: u32, status: FilingStatus) -> Result<Vec<TaxBracket>, DataError> {
    match year {
        2025 => Ok(brackets_2025(status)),
        2026 => Ok(brackets_2026(status)),
        _ => Err(DataError::UnsupportedYear(year)),
    }
}

fn brackets_2025(status: FilingStatus) -> Vec<TaxBracket> {
    match status {
        FilingStatus::Single => vec![
            TaxBracket {
                min: 0.0,
                max: Some(11925.0),
                rate: 0.1,
            },
            TaxBracket {
                min: 11925.0,
                max: Some(48475.0),
                rate: 0.12,
            },
            TaxBracket {
                min: 48475.0,
                max: Some(103350.0),
                rate: 0.22,
            },
            TaxBracket {
                min: 103350.0,
                max: Some(197300.0),
                rate: 0.24,
            },
            TaxBracket {
                min: 197300.0,
                max: Some(250525.0),
                rate: 0.32,
            },
            TaxBracket {
                min: 250525.0,
                max: Some(626350.0),
                rate: 0.35,
            },
            TaxBracket {
                min: 626350.0,
                max: None,
                rate: 0.37,
            },
        ],
        FilingStatus::MarriedFilingJointly => vec![
            TaxBracket {
                min: 0.0,
                max: Some(23850.0),
                rate: 0.1,
            },
            TaxBracket {
                min: 23850.0,
                max: Some(96950.0),
                rate: 0.12,
            },
            TaxBracket {
                min: 96950.0,
                max: Some(206700.0),
                rate: 0.22,
            },
            TaxBracket {
                min: 206700.0,
                max: Some(394600.0),
                rate: 0.24,
            },
            TaxBracket {
                min: 394600.0,
                max: Some(501050.0),
                rate: 0.32,
            },
            TaxBracket {
                min: 501050.0,
                max: Some(751600.0),
                rate: 0.35,
            },
            TaxBracket {
                min: 751600.0,
                max: None,
                rate: 0.37,
            },
        ],
        FilingStatus::MarriedFilingSeparately => vec![
            TaxBracket {
                min: 0.0,
                max: Some(11925.0),
                rate: 0.1,
            },
            TaxBracket {
                min: 11925.0,
                max: Some(48475.0),
                rate: 0.12,
            },
            TaxBracket {
                min: 48475.0,
                max: Some(103350.0),
                rate: 0.22,
            },
            TaxBracket {
                min: 103350.0,
                max: Some(197300.0),
                rate: 0.24,
            },
            TaxBracket {
                min: 197300.0,
                max: Some(250525.0),
                rate: 0.32,
            },
            TaxBracket {
                min: 250525.0,
                max: Some(375800.0),
                rate: 0.35,
            },
            TaxBracket {
                min: 375800.0,
                max: None,
                rate: 0.37,
            },
        ],
        FilingStatus::HeadOfHousehold => vec![
            TaxBracket {
                min: 0.0,
                max: Some(17000.0),
                rate: 0.1,
            },
            TaxBracket {
                min: 17000.0,
                max: Some(64850.0),
                rate: 0.12,
            },
            TaxBracket {
                min: 64850.0,
                max: Some(103350.0),
                rate: 0.22,
            },
            TaxBracket {
                min: 103350.0,
                max: Some(197300.0),
                rate: 0.24,
            },
            TaxBracket {
                min: 197300.0,
                max: Some(250500.0),
                rate: 0.32,
            },
            TaxBracket {
                min: 250500.0,
                max: Some(626350.0),
                rate: 0.35,
            },
            TaxBracket {
                min: 626350.0,
                max: None,
                rate: 0.37,
            },
        ],
        FilingStatus::QualifyingSurvivingSpouse => vec![
            TaxBracket {
                min: 0.0,
                max: Some(23850.0),
                rate: 0.1,
            },
            TaxBracket {
                min: 23850.0,
                max: Some(96950.0),
                rate: 0.12,
            },
            TaxBracket {
                min: 96950.0,
                max: Some(206700.0),
                rate: 0.22,
            },
            TaxBracket {
                min: 206700.0,
                max: Some(394600.0),
                rate: 0.24,
            },
            TaxBracket {
                min: 394600.0,
                max: Some(501050.0),
                rate: 0.32,
            },
            TaxBracket {
                min: 501050.0,
                max: Some(751600.0),
                rate: 0.35,
            },
            TaxBracket {
                min: 751600.0,
                max: None,
                rate: 0.37,
            },
        ],
    }
}

fn brackets_2026(status: FilingStatus) -> Vec<TaxBracket> {
    match status {
        FilingStatus::Single => vec![
            TaxBracket {
                min: 0.0,
                max: Some(12400.0),
                rate: 0.1,
            },
            TaxBracket {
                min: 12400.0,
                max: Some(50400.0),
                rate: 0.12,
            },
            TaxBracket {
                min: 50400.0,
                max: Some(105700.0),
                rate: 0.22,
            },
            TaxBracket {
                min: 105700.0,
                max: Some(201775.0),
                rate: 0.24,
            },
            TaxBracket {
                min: 201775.0,
                max: Some(256225.0),
                rate: 0.32,
            },
            TaxBracket {
                min: 256225.0,
                max: Some(640600.0),
                rate: 0.35,
            },
            TaxBracket {
                min: 640600.0,
                max: None,
                rate: 0.37,
            },
        ],
        FilingStatus::MarriedFilingJointly => vec![
            TaxBracket {
                min: 0.0,
                max: Some(24800.0),
                rate: 0.1,
            },
            TaxBracket {
                min: 24800.0,
                max: Some(100800.0),
                rate: 0.12,
            },
            TaxBracket {
                min: 100800.0,
                max: Some(211400.0),
                rate: 0.22,
            },
            TaxBracket {
                min: 211400.0,
                max: Some(403550.0),
                rate: 0.24,
            },
            TaxBracket {
                min: 403550.0,
                max: Some(512450.0),
                rate: 0.32,
            },
            TaxBracket {
                min: 512450.0,
                max: Some(768700.0),
                rate: 0.35,
            },
            TaxBracket {
                min: 768700.0,
                max: None,
                rate: 0.37,
            },
        ],
        FilingStatus::MarriedFilingSeparately => vec![
            TaxBracket {
                min: 0.0,
                max: Some(12400.0),
                rate: 0.1,
            },
            TaxBracket {
                min: 12400.0,
                max: Some(50400.0),
                rate: 0.12,
            },
            TaxBracket {
                min: 50400.0,
                max: Some(105700.0),
                rate: 0.22,
            },
            TaxBracket {
                min: 105700.0,
                max: Some(201775.0),
                rate: 0.24,
            },
            TaxBracket {
                min: 201775.0,
                max: Some(256225.0),
                rate: 0.32,
            },
            TaxBracket {
                min: 256225.0,
                max: Some(384350.0),
                rate: 0.35,
            },
            TaxBracket {
                min: 384350.0,
                max: None,
                rate: 0.37,
            },
        ],
        FilingStatus::HeadOfHousehold => vec![
            TaxBracket {
                min: 0.0,
                max: Some(17700.0),
                rate: 0.1,
            },
            TaxBracket {
                min: 17700.0,
                max: Some(67450.0),
                rate: 0.12,
            },
            TaxBracket {
                min: 67450.0,
                max: Some(105700.0),
                rate: 0.22,
            },
            TaxBracket {
                min: 105700.0,
                max: Some(201750.0),
                rate: 0.24,
            },
            TaxBracket {
                min: 201750.0,
                max: Some(256200.0),
                rate: 0.32,
            },
            TaxBracket {
                min: 256200.0,
                max: Some(640600.0),
                rate: 0.35,
            },
            TaxBracket {
                min: 640600.0,
                max: None,
                rate: 0.37,
            },
        ],
        FilingStatus::QualifyingSurvivingSpouse => vec![
            TaxBracket {
                min: 0.0,
                max: Some(24800.0),
                rate: 0.1,
            },
            TaxBracket {
                min: 24800.0,
                max: Some(100800.0),
                rate: 0.12,
            },
            TaxBracket {
                min: 100800.0,
                max: Some(211400.0),
                rate: 0.22,
            },
            TaxBracket {
                min: 211400.0,
                max: Some(403550.0),
                rate: 0.24,
            },
            TaxBracket {
                min: 403550.0,
                max: Some(512450.0),
                rate: 0.32,
            },
            TaxBracket {
                min: 512450.0,
                max: Some(768700.0),
                rate: 0.35,
            },
            TaxBracket {
                min: 768700.0,
                max: None,
                rate: 0.37,
            },
        ],
    }
}

// ---------------------------------------------------------------------------
// Standard deductions (2026, reviewed artifact)
// ---------------------------------------------------------------------------

pub fn standard_deductions(status: FilingStatus) -> f64 {
    match status {
        FilingStatus::Single => 16100.0,
        FilingStatus::MarriedFilingJointly => 32200.0,
        FilingStatus::MarriedFilingSeparately => 16100.0,
        FilingStatus::HeadOfHousehold => 24150.0,
        FilingStatus::QualifyingSurvivingSpouse => 32200.0,
    }
}

// ---------------------------------------------------------------------------
// Capital gains brackets (2026, reviewed artifact)
// ---------------------------------------------------------------------------

pub fn capital_gains_brackets(status: FilingStatus) -> Vec<TaxBracket> {
    match status {
        FilingStatus::Single => vec![
            TaxBracket {
                min: 0.0,
                max: Some(49450.0),
                rate: 0.0,
            },
            TaxBracket {
                min: 49450.0,
                max: Some(545500.0),
                rate: 0.15,
            },
            TaxBracket {
                min: 545500.0,
                max: None,
                rate: 0.2,
            },
        ],
        FilingStatus::MarriedFilingJointly => vec![
            TaxBracket {
                min: 0.0,
                max: Some(98900.0),
                rate: 0.0,
            },
            TaxBracket {
                min: 98900.0,
                max: Some(613700.0),
                rate: 0.15,
            },
            TaxBracket {
                min: 613700.0,
                max: None,
                rate: 0.2,
            },
        ],
        FilingStatus::MarriedFilingSeparately => vec![
            TaxBracket {
                min: 0.0,
                max: Some(49450.0),
                rate: 0.0,
            },
            TaxBracket {
                min: 49450.0,
                max: Some(306850.0),
                rate: 0.15,
            },
            TaxBracket {
                min: 306850.0,
                max: None,
                rate: 0.2,
            },
        ],
        FilingStatus::HeadOfHousehold => vec![
            TaxBracket {
                min: 0.0,
                max: Some(66200.0),
                rate: 0.0,
            },
            TaxBracket {
                min: 66200.0,
                max: Some(579600.0),
                rate: 0.15,
            },
            TaxBracket {
                min: 579600.0,
                max: None,
                rate: 0.2,
            },
        ],
        FilingStatus::QualifyingSurvivingSpouse => vec![
            TaxBracket {
                min: 0.0,
                max: Some(98900.0),
                rate: 0.0,
            },
            TaxBracket {
                min: 98900.0,
                max: Some(613700.0),
                rate: 0.15,
            },
            TaxBracket {
                min: 613700.0,
                max: None,
                rate: 0.2,
            },
        ],
    }
}

// ---------------------------------------------------------------------------
// Net Investment Income Tax (statutory, not indexed)
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// Net Investment Income Tax (2026, reviewed artifact)
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// Net Investment Income Tax (2026, reviewed artifact)
// ---------------------------------------------------------------------------

pub fn niit(status: FilingStatus) -> NiitParams {
    let (rate, threshold) = match status {
        FilingStatus::Single => (0.038, 200000.0),
        FilingStatus::MarriedFilingJointly => (0.038, 250000.0),
        FilingStatus::MarriedFilingSeparately => (0.038, 125000.0),
        FilingStatus::HeadOfHousehold => (0.038, 200000.0),
        FilingStatus::QualifyingSurvivingSpouse => (0.038, 250000.0),
    };
    NiitParams { rate, threshold }
}

pub fn payroll(status: FilingStatus) -> PayrollParams {
    match status {
        FilingStatus::Single => PayrollParams {
            social_security_rate: 0.062,
            social_security_wage_base: 184500.0,
            self_employment_tax_rate: 0.124,
            medicare_rate: 0.0145,
            self_employment_medicare_rate: 0.029,
            additional_medicare_rate: 0.009,
            additional_medicare_threshold: 200000.0,
        },
        FilingStatus::MarriedFilingJointly => PayrollParams {
            social_security_rate: 0.062,
            social_security_wage_base: 184500.0,
            self_employment_tax_rate: 0.124,
            medicare_rate: 0.0145,
            self_employment_medicare_rate: 0.029,
            additional_medicare_rate: 0.009,
            additional_medicare_threshold: 250000.0,
        },
        FilingStatus::MarriedFilingSeparately => PayrollParams {
            social_security_rate: 0.062,
            social_security_wage_base: 184500.0,
            self_employment_tax_rate: 0.124,
            medicare_rate: 0.0145,
            self_employment_medicare_rate: 0.029,
            additional_medicare_rate: 0.009,
            additional_medicare_threshold: 125000.0,
        },
        FilingStatus::HeadOfHousehold => PayrollParams {
            social_security_rate: 0.062,
            social_security_wage_base: 184500.0,
            self_employment_tax_rate: 0.124,
            medicare_rate: 0.0145,
            self_employment_medicare_rate: 0.029,
            additional_medicare_rate: 0.009,
            additional_medicare_threshold: 200000.0,
        },
        FilingStatus::QualifyingSurvivingSpouse => PayrollParams {
            social_security_rate: 0.062,
            social_security_wage_base: 184500.0,
            self_employment_tax_rate: 0.124,
            medicare_rate: 0.0145,
            self_employment_medicare_rate: 0.029,
            additional_medicare_rate: 0.009,
            additional_medicare_threshold: 200000.0,
        },
    }
}

// ---------------------------------------------------------------------------
// Capital loss limit (statutory)
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// Capital loss limit (2026, reviewed artifact)
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// Capital loss limit (2026, reviewed artifact)
// ---------------------------------------------------------------------------

pub fn capital_loss_limit(status: FilingStatus) -> f64 {
    match status {
        FilingStatus::Single => 3000.0,
        FilingStatus::MarriedFilingJointly => 3000.0,
        FilingStatus::MarriedFilingSeparately => 1500.0,
        FilingStatus::HeadOfHousehold => 3000.0,
        FilingStatus::QualifyingSurvivingSpouse => 3000.0,
    }
}

// ---------------------------------------------------------------------------
// SALT deduction parameters (2026, reviewed artifact)
// ---------------------------------------------------------------------------

pub fn salt_deduction_parameters(status: FilingStatus) -> SaltDeductionParams {
    match status {
        FilingStatus::Single => SaltDeductionParams {
            cap_amount: 40400.0,
            phaseout_threshold: 505000.0,
            phaseout_rate: 0.3,
            floor_amount: 10000.0,
        },
        FilingStatus::MarriedFilingJointly => SaltDeductionParams {
            cap_amount: 40400.0,
            phaseout_threshold: 505000.0,
            phaseout_rate: 0.3,
            floor_amount: 10000.0,
        },
        FilingStatus::MarriedFilingSeparately => SaltDeductionParams {
            cap_amount: 20200.0,
            phaseout_threshold: 252500.0,
            phaseout_rate: 0.3,
            floor_amount: 5000.0,
        },
        FilingStatus::HeadOfHousehold => SaltDeductionParams {
            cap_amount: 40400.0,
            phaseout_threshold: 505000.0,
            phaseout_rate: 0.3,
            floor_amount: 10000.0,
        },
        FilingStatus::QualifyingSurvivingSpouse => SaltDeductionParams {
            cap_amount: 40400.0,
            phaseout_threshold: 505000.0,
            phaseout_rate: 0.3,
            floor_amount: 10000.0,
        },
    }
}

// ---------------------------------------------------------------------------
// QBI Deduction parameters (Section 199A, 2026 post-OBBBA)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct QbiDeductionParams {
    pub deduction_rate: f64,
    pub threshold: f64,
    pub phase_in_range_end: f64,
    pub minimum_qbi_deduction: f64,
    pub minimum_qbi_amount: f64,
}

// ---------------------------------------------------------------------------
// QBI Deduction parameters (Section 199A, 2026, reviewed artifact)
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// QBI Deduction parameters (Section 199A, 2026, reviewed artifact)
// ---------------------------------------------------------------------------

pub fn qbi_deduction(status: FilingStatus) -> QbiDeductionParams {
    match status {
        FilingStatus::Single => QbiDeductionParams {
            deduction_rate: 0.2,
            threshold: 201750.0,
            phase_in_range_end: 276750.0,
            minimum_qbi_deduction: 400.0,
            minimum_qbi_amount: 1000.0,
        },
        FilingStatus::MarriedFilingJointly => QbiDeductionParams {
            deduction_rate: 0.2,
            threshold: 403500.0,
            phase_in_range_end: 553500.0,
            minimum_qbi_deduction: 400.0,
            minimum_qbi_amount: 1000.0,
        },
        FilingStatus::MarriedFilingSeparately => QbiDeductionParams {
            deduction_rate: 0.2,
            threshold: 201775.0,
            phase_in_range_end: 276775.0,
            minimum_qbi_deduction: 400.0,
            minimum_qbi_amount: 1000.0,
        },
        FilingStatus::HeadOfHousehold => QbiDeductionParams {
            deduction_rate: 0.2,
            threshold: 201750.0,
            phase_in_range_end: 276750.0,
            minimum_qbi_deduction: 400.0,
            minimum_qbi_amount: 1000.0,
        },
        FilingStatus::QualifyingSurvivingSpouse => QbiDeductionParams {
            deduction_rate: 0.2,
            threshold: 201750.0,
            phase_in_range_end: 276750.0,
            minimum_qbi_deduction: 400.0,
            minimum_qbi_amount: 1000.0,
        },
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_bracket_count_2026() {
        let b = brackets_for_year(2026, FilingStatus::Single).unwrap();
        assert_eq!(b.len(), 7);
        assert_eq!(b[0].rate, 0.10);
        assert_eq!(b[6].rate, 0.37);
        assert_eq!(b[6].max, None);
    }

    #[test]
    fn single_bracket_count_2025() {
        let b = brackets_for_year(2025, FilingStatus::Single).unwrap();
        assert_eq!(b.len(), 7);
        assert_eq!(b[0].max, Some(11_925.0));
        assert_eq!(b[5].max, Some(626_350.0));
        assert_eq!(b[6].rate, 0.37);
    }

    #[test]
    fn brackets_default_to_2026() {
        let default = brackets(FilingStatus::Single);
        let y2026 = brackets_for_year(2026, FilingStatus::Single).unwrap();
        assert_eq!(default.len(), y2026.len());
        assert_eq!(default[0].max, y2026[0].max);
        assert_eq!(default[5].max, y2026[5].max);
    }

    #[test]
    fn mfj_first_bracket_double_single_2026() {
        let s = brackets_for_year(2026, FilingStatus::Single).unwrap();
        let m = brackets_for_year(2026, FilingStatus::MarriedFilingJointly).unwrap();
        assert_eq!(m[0].max.unwrap(), s[0].max.unwrap() * 2.0);
    }

    #[test]
    fn standard_deduction_values() {
        assert_eq!(standard_deductions(FilingStatus::Single), 16_100.0);
        assert_eq!(
            standard_deductions(FilingStatus::MarriedFilingJointly),
            32_200.0
        );
        assert_eq!(standard_deductions(FilingStatus::HeadOfHousehold), 24_150.0);
    }

    #[test]
    fn cg_brackets_single() {
        let b = capital_gains_brackets(FilingStatus::Single);
        assert_eq!(b.len(), 3);
        assert_eq!(b[0].rate, 0.0);
        assert_eq!(b[0].max, Some(49_450.0));
    }

    #[test]
    fn niit_thresholds() {
        assert_eq!(niit(FilingStatus::Single).threshold, 200000.0);
        assert_eq!(niit(FilingStatus::MarriedFilingJointly).threshold, 250000.0);
        assert_eq!(niit(FilingStatus::MarriedFilingSeparately).threshold, 125000.0);
        assert_eq!(niit(FilingStatus::Single).rate, 0.038);
    }

    #[test]
    fn payroll_ss_wage_base_2026() {
        let p = payroll(FilingStatus::Single);
        assert_eq!(p.social_security_wage_base, 184500.0);
        assert_eq!(p.social_security_rate, 0.062);
    }

    #[test]
    fn salt_deduction_parameters_single() {
        let salt = salt_deduction_parameters(FilingStatus::Single);
        assert_eq!(salt.cap_amount, 40400.0);
        assert_eq!(salt.phaseout_threshold, 505000.0);
        assert_eq!(salt.phaseout_rate, 0.3);
        assert_eq!(salt.floor_amount, 10000.0);
    }

    #[test]
    fn salt_deduction_parameters_mfs() {
        let salt = salt_deduction_parameters(FilingStatus::MarriedFilingSeparately);
        assert_eq!(salt.cap_amount, 20200.0);
        assert_eq!(salt.floor_amount, 5000.0);
    }

    #[test]
    fn qbi_deduction_mfj() {
        let q = qbi_deduction(FilingStatus::MarriedFilingJointly);
        assert_eq!(q.threshold, 403500.0);
        assert_eq!(q.phase_in_range_end, 553500.0);
        assert_eq!(q.deduction_rate, 0.2);
        assert_eq!(q.minimum_qbi_deduction, 400.0);
        assert_eq!(q.minimum_qbi_amount, 1000.0);
    }

    #[test]
    fn qbi_deduction_single() {
        let q = qbi_deduction(FilingStatus::Single);
        assert_eq!(q.threshold, 201750.0);
        assert_eq!(q.phase_in_range_end, 276750.0);
    }

    #[test]
    fn qss_equals_mfj() {
        let mfj = brackets(FilingStatus::MarriedFilingJointly);
        let qss = brackets(FilingStatus::QualifyingSurvivingSpouse);
        assert_eq!(mfj.len(), qss.len());
        for (a, b) in mfj.iter().zip(qss.iter()) {
            assert_eq!(a.rate, b.rate);
            assert_eq!(a.min, b.min);
            assert_eq!(a.max, b.max);
        }
    }

    #[test]
    fn hoh_brackets() {
        let b = brackets(FilingStatus::HeadOfHousehold);
        assert_eq!(b[0].max, Some(17_700.0));
        assert_eq!(b[1].max, Some(67_450.0));
    }
}
