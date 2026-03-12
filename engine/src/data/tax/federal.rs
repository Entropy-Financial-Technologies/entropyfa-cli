use crate::data::types::FilingStatus;
use crate::models::tax_request::{NiitParams, PayrollParams, TaxBracket};

// ---------------------------------------------------------------------------
// Federal income tax brackets (2026, IRS Rev. Proc. 2025-32)
// ---------------------------------------------------------------------------

pub fn brackets(status: FilingStatus) -> Vec<TaxBracket> {
    match status {
        FilingStatus::Single => vec![
            TaxBracket {
                min: 0.0,
                max: Some(12_400.0),
                rate: 0.10,
            },
            TaxBracket {
                min: 12_400.0,
                max: Some(50_400.0),
                rate: 0.12,
            },
            TaxBracket {
                min: 50_400.0,
                max: Some(105_700.0),
                rate: 0.22,
            },
            TaxBracket {
                min: 105_700.0,
                max: Some(201_775.0),
                rate: 0.24,
            },
            TaxBracket {
                min: 201_775.0,
                max: Some(256_225.0),
                rate: 0.32,
            },
            TaxBracket {
                min: 256_225.0,
                max: Some(640_600.0),
                rate: 0.35,
            },
            TaxBracket {
                min: 640_600.0,
                max: None,
                rate: 0.37,
            },
        ],
        FilingStatus::MarriedFilingJointly | FilingStatus::QualifyingSurvivingSpouse => vec![
            TaxBracket {
                min: 0.0,
                max: Some(24_800.0),
                rate: 0.10,
            },
            TaxBracket {
                min: 24_800.0,
                max: Some(100_800.0),
                rate: 0.12,
            },
            TaxBracket {
                min: 100_800.0,
                max: Some(211_400.0),
                rate: 0.22,
            },
            TaxBracket {
                min: 211_400.0,
                max: Some(403_550.0),
                rate: 0.24,
            },
            TaxBracket {
                min: 403_550.0,
                max: Some(512_450.0),
                rate: 0.32,
            },
            TaxBracket {
                min: 512_450.0,
                max: Some(768_700.0),
                rate: 0.35,
            },
            TaxBracket {
                min: 768_700.0,
                max: None,
                rate: 0.37,
            },
        ],
        FilingStatus::MarriedFilingSeparately => vec![
            TaxBracket {
                min: 0.0,
                max: Some(12_400.0),
                rate: 0.10,
            },
            TaxBracket {
                min: 12_400.0,
                max: Some(50_400.0),
                rate: 0.12,
            },
            TaxBracket {
                min: 50_400.0,
                max: Some(105_700.0),
                rate: 0.22,
            },
            TaxBracket {
                min: 105_700.0,
                max: Some(201_775.0),
                rate: 0.24,
            },
            TaxBracket {
                min: 201_775.0,
                max: Some(256_225.0),
                rate: 0.32,
            },
            TaxBracket {
                min: 256_225.0,
                max: Some(384_350.0),
                rate: 0.35,
            },
            TaxBracket {
                min: 384_350.0,
                max: None,
                rate: 0.37,
            },
        ],
        FilingStatus::HeadOfHousehold => vec![
            TaxBracket {
                min: 0.0,
                max: Some(17_700.0),
                rate: 0.10,
            },
            TaxBracket {
                min: 17_700.0,
                max: Some(67_450.0),
                rate: 0.12,
            },
            TaxBracket {
                min: 67_450.0,
                max: Some(105_700.0),
                rate: 0.22,
            },
            TaxBracket {
                min: 105_700.0,
                max: Some(201_750.0),
                rate: 0.24,
            },
            TaxBracket {
                min: 201_750.0,
                max: Some(256_200.0),
                rate: 0.32,
            },
            TaxBracket {
                min: 256_200.0,
                max: Some(640_600.0),
                rate: 0.35,
            },
            TaxBracket {
                min: 640_600.0,
                max: None,
                rate: 0.37,
            },
        ],
    }
}

// ---------------------------------------------------------------------------
// Standard deductions (2026)
// ---------------------------------------------------------------------------

pub fn standard_deductions(status: FilingStatus) -> f64 {
    match status {
        FilingStatus::Single | FilingStatus::MarriedFilingSeparately => 16_100.0,
        FilingStatus::MarriedFilingJointly | FilingStatus::QualifyingSurvivingSpouse => 32_200.0,
        FilingStatus::HeadOfHousehold => 24_150.0,
    }
}

// ---------------------------------------------------------------------------
// Capital gains brackets (2026)
// ---------------------------------------------------------------------------

pub fn capital_gains_brackets(status: FilingStatus) -> Vec<TaxBracket> {
    match status {
        FilingStatus::Single => vec![
            TaxBracket {
                min: 0.0,
                max: Some(49_450.0),
                rate: 0.0,
            },
            TaxBracket {
                min: 49_450.0,
                max: Some(545_500.0),
                rate: 0.15,
            },
            TaxBracket {
                min: 545_500.0,
                max: None,
                rate: 0.20,
            },
        ],
        FilingStatus::MarriedFilingJointly | FilingStatus::QualifyingSurvivingSpouse => vec![
            TaxBracket {
                min: 0.0,
                max: Some(98_900.0),
                rate: 0.0,
            },
            TaxBracket {
                min: 98_900.0,
                max: Some(613_700.0),
                rate: 0.15,
            },
            TaxBracket {
                min: 613_700.0,
                max: None,
                rate: 0.20,
            },
        ],
        FilingStatus::MarriedFilingSeparately => vec![
            TaxBracket {
                min: 0.0,
                max: Some(49_450.0),
                rate: 0.0,
            },
            TaxBracket {
                min: 49_450.0,
                max: Some(306_850.0),
                rate: 0.15,
            },
            TaxBracket {
                min: 306_850.0,
                max: None,
                rate: 0.20,
            },
        ],
        FilingStatus::HeadOfHousehold => vec![
            TaxBracket {
                min: 0.0,
                max: Some(66_200.0),
                rate: 0.0,
            },
            TaxBracket {
                min: 66_200.0,
                max: Some(579_600.0),
                rate: 0.15,
            },
            TaxBracket {
                min: 579_600.0,
                max: None,
                rate: 0.20,
            },
        ],
    }
}

// ---------------------------------------------------------------------------
// Net Investment Income Tax (statutory, not indexed)
// ---------------------------------------------------------------------------

pub fn niit(status: FilingStatus) -> NiitParams {
    let threshold = match status {
        FilingStatus::Single | FilingStatus::HeadOfHousehold => 200_000.0,
        FilingStatus::MarriedFilingJointly | FilingStatus::QualifyingSurvivingSpouse => 250_000.0,
        FilingStatus::MarriedFilingSeparately => 125_000.0,
    };
    NiitParams {
        rate: 0.038,
        threshold,
    }
}

// ---------------------------------------------------------------------------
// Payroll tax parameters (2026)
// ---------------------------------------------------------------------------

pub fn payroll(status: FilingStatus) -> PayrollParams {
    let additional_medicare_threshold = match status {
        FilingStatus::Single | FilingStatus::HeadOfHousehold => 200_000.0,
        FilingStatus::MarriedFilingJointly | FilingStatus::QualifyingSurvivingSpouse => 250_000.0,
        FilingStatus::MarriedFilingSeparately => 125_000.0,
    };
    PayrollParams {
        social_security_rate: 0.062,
        social_security_wage_base: 184_500.0,
        self_employment_tax_rate: 0.124,
        medicare_rate: 0.0145,
        self_employment_medicare_rate: 0.029,
        additional_medicare_rate: 0.009,
        additional_medicare_threshold,
    }
}

// ---------------------------------------------------------------------------
// Capital loss limit (statutory)
// ---------------------------------------------------------------------------

pub fn capital_loss_limit(status: FilingStatus) -> f64 {
    match status {
        FilingStatus::MarriedFilingSeparately => 1_500.0,
        _ => 3_000.0,
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

pub fn qbi_deduction(status: FilingStatus) -> QbiDeductionParams {
    let (threshold, phase_in_end) = match status {
        FilingStatus::MarriedFilingJointly => (403_500.0, 553_500.0),
        FilingStatus::MarriedFilingSeparately => (201_750.0, 276_750.0),
        FilingStatus::Single
        | FilingStatus::HeadOfHousehold
        | FilingStatus::QualifyingSurvivingSpouse => (201_750.0, 276_750.0),
    };
    QbiDeductionParams {
        deduction_rate: 0.20,
        threshold,
        phase_in_range_end: phase_in_end,
        minimum_qbi_deduction: 400.0,
        minimum_qbi_amount: 1_000.0,
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_bracket_count() {
        let b = brackets(FilingStatus::Single);
        assert_eq!(b.len(), 7);
        assert_eq!(b[0].rate, 0.10);
        assert_eq!(b[6].rate, 0.37);
        assert_eq!(b[6].max, None);
    }

    #[test]
    fn mfj_first_bracket_double_single() {
        let s = brackets(FilingStatus::Single);
        let m = brackets(FilingStatus::MarriedFilingJointly);
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
        assert_eq!(niit(FilingStatus::Single).threshold, 200_000.0);
        assert_eq!(
            niit(FilingStatus::MarriedFilingJointly).threshold,
            250_000.0
        );
        assert_eq!(
            niit(FilingStatus::MarriedFilingSeparately).threshold,
            125_000.0
        );
    }

    #[test]
    fn payroll_ss_wage_base_2026() {
        let p = payroll(FilingStatus::Single);
        assert_eq!(p.social_security_wage_base, 184_500.0);
        assert_eq!(p.social_security_rate, 0.062);
    }

    #[test]
    fn capital_loss_limit_mfs() {
        assert_eq!(
            capital_loss_limit(FilingStatus::MarriedFilingSeparately),
            1_500.0
        );
        assert_eq!(capital_loss_limit(FilingStatus::Single), 3_000.0);
    }

    #[test]
    fn qbi_deduction_mfj() {
        let q = qbi_deduction(FilingStatus::MarriedFilingJointly);
        assert_eq!(q.threshold, 403_500.0);
        assert_eq!(q.phase_in_range_end, 553_500.0);
        assert_eq!(q.deduction_rate, 0.20);
        assert_eq!(q.minimum_qbi_deduction, 400.0);
        assert_eq!(q.minimum_qbi_amount, 1_000.0);
    }

    #[test]
    fn qbi_deduction_single() {
        let q = qbi_deduction(FilingStatus::Single);
        assert_eq!(q.threshold, 201_750.0);
        assert_eq!(q.phase_in_range_end, 276_750.0);
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
