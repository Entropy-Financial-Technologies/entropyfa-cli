use crate::models::simulation_request::{
    CashFlow, HouseholdConfig, ReturnAssumption, RmdPolicy, SimulationRequest, TaxPolicy,
};
use crate::validation::validate_simulation_request_contract;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BucketType {
    Taxable,
    TaxDeferred,
    TaxFree,
    Cash,
}

#[derive(Debug, Clone)]
pub struct NormalizedBucket {
    pub id: String,
    pub bucket_type: BucketType,
    pub starting_balance: f64,
    pub return_assumption: ReturnAssumption,
    pub realized_gain_ratio: Option<f64>,
    pub withdrawal_priority: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct NormalizedCashFlow {
    pub amount: f64,
    pub frequency: String,
    pub start_month: Option<u32>,
    pub end_month: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct NormalizedHousehold {
    pub birth_years: Option<Vec<u32>>,
    pub retirement_month: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct NormalizedSpendingPolicy {
    pub withdrawal_order: Vec<String>,
    pub rebalance_tax_withholding_from: Option<String>,
}

#[derive(Debug, Clone)]
pub struct NormalizedTaxPolicy {
    pub mode: String,
    pub modeled_tax_inflation_rate: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct NormalizedRmdPolicy {
    pub enabled: bool,
    pub distribution_month: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct NormalizedSimulationRequest {
    pub num_simulations: u32,
    pub seed: Option<u64>,
    pub time_horizon_months: u32,
    pub filing_status: Option<String>,
    pub household: Option<NormalizedHousehold>,
    pub buckets: Vec<NormalizedBucket>,
    pub cash_flows: Vec<NormalizedCashFlow>,
    pub spending_policy: NormalizedSpendingPolicy,
    pub tax_policy: NormalizedTaxPolicy,
    pub rmd_policy: Option<NormalizedRmdPolicy>,
    pub include_detail: bool,
    pub detail_granularity: String,
    pub sample_paths: Option<usize>,
    pub path_indices: Option<Vec<usize>>,
    pub custom_percentiles: Option<Vec<u32>>,
}

pub fn normalize_request(
    req: &SimulationRequest,
) -> Result<NormalizedSimulationRequest, Vec<String>> {
    let mut errors = validate_simulation_request_contract(req);
    if !errors.is_empty() {
        return Err(errors);
    }

    let buckets = if req.buckets.is_empty() {
        vec![NormalizedBucket {
            id: "taxable".into(),
            bucket_type: BucketType::Taxable,
            starting_balance: req.legacy_starting_balance(),
            return_assumption: req.legacy_return_assumption().clone(),
            realized_gain_ratio: None,
            withdrawal_priority: Some(1),
        }]
    } else {
        req.buckets
            .iter()
            .filter_map(|bucket| match parse_bucket_type(&bucket.bucket_type) {
                Some(bucket_type) => Some(NormalizedBucket {
                    id: bucket.id.clone(),
                    bucket_type,
                    starting_balance: bucket.starting_balance,
                    return_assumption: bucket.return_assumption.clone(),
                    realized_gain_ratio: bucket.realized_gain_ratio,
                    withdrawal_priority: bucket.withdrawal_priority,
                }),
                None => {
                    errors.push(format!(
                        "bucket_type must be one of: taxable, tax_deferred, tax_free, cash (got '{}')",
                        bucket.bucket_type
                    ));
                    None
                }
            })
            .collect()
    };

    if !errors.is_empty() {
        return Err(errors);
    }

    Ok(NormalizedSimulationRequest {
        num_simulations: req.num_simulations.unwrap_or(10000),
        seed: req.seed,
        time_horizon_months: req.time_horizon_months,
        filing_status: req.filing_status.clone(),
        household: req.household.as_ref().map(NormalizedHousehold::from),
        buckets,
        cash_flows: req
            .cash_flows
            .iter()
            .map(NormalizedCashFlow::from)
            .collect(),
        spending_policy: normalize_spending_policy(req),
        tax_policy: normalize_tax_policy(req.tax_policy.as_ref()),
        rmd_policy: req.rmd_policy.as_ref().map(NormalizedRmdPolicy::from),
        include_detail: req.include_detail,
        detail_granularity: req.detail_granularity.clone(),
        sample_paths: req.sample_paths,
        path_indices: req.path_indices.clone(),
        custom_percentiles: req.custom_percentiles.clone(),
    })
}

fn parse_bucket_type(bucket_type: &str) -> Option<BucketType> {
    match bucket_type {
        "taxable" => Some(BucketType::Taxable),
        "tax_deferred" => Some(BucketType::TaxDeferred),
        "tax_free" => Some(BucketType::TaxFree),
        "cash" => Some(BucketType::Cash),
        _ => None,
    }
}

fn normalize_spending_policy(req: &SimulationRequest) -> NormalizedSpendingPolicy {
    match req.spending_policy.as_ref() {
        Some(policy) if !policy.withdrawal_order.is_empty() => NormalizedSpendingPolicy {
            withdrawal_order: policy.withdrawal_order.clone(),
            rebalance_tax_withholding_from: policy.rebalance_tax_withholding_from.clone(),
        },
        Some(policy) => NormalizedSpendingPolicy {
            withdrawal_order: req.buckets.iter().map(|bucket| bucket.id.clone()).collect(),
            rebalance_tax_withholding_from: policy.rebalance_tax_withholding_from.clone(),
        },
        None if req.buckets.is_empty() => NormalizedSpendingPolicy {
            withdrawal_order: vec!["taxable".into()],
            rebalance_tax_withholding_from: None,
        },
        None => NormalizedSpendingPolicy {
            withdrawal_order: req.buckets.iter().map(|bucket| bucket.id.clone()).collect(),
            rebalance_tax_withholding_from: None,
        },
    }
}

fn normalize_tax_policy(policy: Option<&TaxPolicy>) -> NormalizedTaxPolicy {
    match policy {
        Some(policy) => NormalizedTaxPolicy {
            mode: policy.mode.clone(),
            modeled_tax_inflation_rate: policy.modeled_tax_inflation_rate,
        },
        None => NormalizedTaxPolicy {
            mode: "none".into(),
            modeled_tax_inflation_rate: None,
        },
    }
}

impl From<&CashFlow> for NormalizedCashFlow {
    fn from(value: &CashFlow) -> Self {
        Self {
            amount: value.amount,
            frequency: value.frequency.clone(),
            start_month: value.start_month,
            end_month: value.end_month,
        }
    }
}

impl From<&HouseholdConfig> for NormalizedHousehold {
    fn from(value: &HouseholdConfig) -> Self {
        Self {
            birth_years: value.birth_years.clone(),
            retirement_month: value.retirement_month,
        }
    }
}

impl From<&RmdPolicy> for NormalizedRmdPolicy {
    fn from(value: &RmdPolicy) -> Self {
        Self {
            enabled: value.enabled,
            distribution_month: value.distribution_month,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{normalize_request, BucketType};
    use crate::models::simulation_request::{
        HouseholdConfig, ReturnAssumption, RmdPolicy, SimulationBucket, SimulationRequest,
        SpendingPolicy, TaxPolicy,
    };

    fn bucketed_request_without_withdrawal_order() -> SimulationRequest {
        SimulationRequest {
            mode: Some("both".into()),
            num_simulations: Some(100),
            seed: Some(1),
            starting_balance: None,
            buckets: vec![
                SimulationBucket {
                    id: "taxable".into(),
                    bucket_type: "taxable".into(),
                    starting_balance: 60_000.0,
                    return_assumption: ReturnAssumption {
                        annual_mean: 0.06,
                        annual_std_dev: 0.10,
                    },
                    realized_gain_ratio: Some(0.25),
                    withdrawal_priority: Some(1),
                },
                SimulationBucket {
                    id: "traditional_ira".into(),
                    bucket_type: "tax_deferred".into(),
                    starting_balance: 40_000.0,
                    return_assumption: ReturnAssumption {
                        annual_mean: 0.05,
                        annual_std_dev: 0.08,
                    },
                    realized_gain_ratio: None,
                    withdrawal_priority: Some(2),
                },
            ],
            time_horizon_months: 12,
            return_assumption: None,
            cash_flows: vec![],
            filing_status: Some("single".into()),
            household: Some(HouseholdConfig {
                birth_years: Some(vec![1980]),
                retirement_month: Some(1),
            }),
            spending_policy: Some(SpendingPolicy {
                withdrawal_order: vec![],
                rebalance_tax_withholding_from: None,
            }),
            tax_policy: Some(TaxPolicy {
                mode: "modeled".into(),
                modeled_tax_inflation_rate: Some(0.02),
            }),
            rmd_policy: Some(RmdPolicy {
                enabled: true,
                distribution_month: Some(12),
            }),
            include_detail: false,
            detail_granularity: "annual".into(),
            sample_paths: None,
            path_indices: None,
            custom_percentiles: None,
        }
    }

    #[test]
    fn test_normalize_legacy_request_into_single_taxable_bucket() {
        let req = SimulationRequest {
            mode: Some("both".into()),
            num_simulations: Some(100),
            seed: Some(1),
            starting_balance: Some(100_000.0),
            buckets: vec![],
            time_horizon_months: 12,
            return_assumption: Some(ReturnAssumption {
                annual_mean: 0.06,
                annual_std_dev: 0.10,
            }),
            cash_flows: vec![],
            filing_status: None,
            household: None,
            spending_policy: None,
            tax_policy: None,
            rmd_policy: None,
            include_detail: false,
            detail_granularity: "annual".into(),
            sample_paths: None,
            path_indices: None,
            custom_percentiles: None,
        };

        let normalized = normalize_request(&req).expect("legacy request should normalize");
        assert_eq!(normalized.buckets.len(), 1);
        assert_eq!(normalized.buckets[0].bucket_type, BucketType::Taxable);
    }

    #[test]
    fn test_normalize_preserves_household_config() {
        let req = SimulationRequest {
            mode: Some("both".into()),
            num_simulations: Some(100),
            seed: Some(1),
            starting_balance: None,
            buckets: vec![SimulationBucket {
                id: "taxable".into(),
                bucket_type: "taxable".into(),
                starting_balance: 100_000.0,
                return_assumption: ReturnAssumption {
                    annual_mean: 0.06,
                    annual_std_dev: 0.10,
                },
                realized_gain_ratio: Some(0.25),
                withdrawal_priority: Some(1),
            }],
            time_horizon_months: 12,
            return_assumption: None,
            cash_flows: vec![],
            filing_status: None,
            household: Some(HouseholdConfig {
                birth_years: Some(vec![1980, 1982]),
                retirement_month: Some(6),
            }),
            spending_policy: None,
            tax_policy: None,
            rmd_policy: None,
            include_detail: false,
            detail_granularity: "annual".into(),
            sample_paths: None,
            path_indices: None,
            custom_percentiles: None,
        };

        let normalized = normalize_request(&req).expect("bucketed request should normalize");
        let household = normalized.household.expect("household should be preserved");
        assert_eq!(household.birth_years, Some(vec![1980, 1982]));
        assert_eq!(household.retirement_month, Some(6));
    }

    #[test]
    fn test_normalize_invalid_legacy_request_missing_starting_balance_returns_error() {
        let req = SimulationRequest {
            mode: Some("both".into()),
            num_simulations: Some(100),
            seed: Some(1),
            starting_balance: None,
            buckets: vec![],
            time_horizon_months: 12,
            return_assumption: Some(ReturnAssumption {
                annual_mean: 0.06,
                annual_std_dev: 0.10,
            }),
            cash_flows: vec![],
            filing_status: None,
            household: None,
            spending_policy: None,
            tax_policy: None,
            rmd_policy: None,
            include_detail: false,
            detail_granularity: "annual".into(),
            sample_paths: None,
            path_indices: None,
            custom_percentiles: None,
        };

        let err = normalize_request(&req).unwrap_err();
        assert!(err.iter().any(|e| e.contains("starting_balance")));
    }

    #[test]
    fn test_normalize_bucketed_request_requires_withdrawal_order_when_multiple_buckets() {
        let req = bucketed_request_without_withdrawal_order();
        let err = normalize_request(&req).unwrap_err();
        assert!(err.iter().any(|e| e.contains("withdrawal_order")));
    }
}
