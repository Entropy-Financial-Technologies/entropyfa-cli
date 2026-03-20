use std::collections::BTreeMap;

use super::buckets::HouseholdBucketState;

#[derive(Debug, Clone, Default)]
pub struct WithdrawalResult {
    pub funded: bool,
    pub remaining_shortfall: f64,
    pub withdrawals_by_bucket: BTreeMap<String, f64>,
}

impl WithdrawalResult {
    pub fn withdrawn_from(&self, bucket_id: &str) -> f64 {
        self.withdrawals_by_bucket
            .get(bucket_id)
            .copied()
            .unwrap_or(0.0)
    }
}

pub fn fund_household_deficit(
    state: &mut HouseholdBucketState,
    amount_needed: f64,
    withdrawal_order: &[impl AsRef<str>],
) -> WithdrawalResult {
    let mut remaining = amount_needed.max(0.0);
    let mut withdrawals_by_bucket = BTreeMap::new();

    for bucket_id in withdrawal_order {
        let bucket_id = bucket_id.as_ref();
        if remaining <= 0.0 {
            break;
        }

        if let Some(bucket) = state.bucket_mut(bucket_id) {
            let withdrawal = bucket.balance.min(remaining);
            if withdrawal > 0.0 {
                bucket.balance -= withdrawal;
                remaining -= withdrawal;
                *withdrawals_by_bucket
                    .entry(bucket_id.to_string())
                    .or_insert(0.0) += withdrawal;
            }
        }
    }

    WithdrawalResult {
        funded: remaining <= 0.0,
        remaining_shortfall: remaining,
        withdrawals_by_bucket,
    }
}

#[cfg(test)]
mod tests {
    use super::fund_household_deficit;
    use crate::compute::simulation::buckets::HouseholdBucketState;

    #[test]
    fn test_withdrawal_order_uses_taxable_before_tax_deferred() {
        let mut state =
            HouseholdBucketState::from_balances(vec![("taxable", 25_000.0), ("ira", 50_000.0)]);

        let result = fund_household_deficit(&mut state, 30_000.0, &["taxable", "ira"]);

        assert_eq!(result.withdrawn_from("taxable"), 25_000.0);
        assert_eq!(result.withdrawn_from("ira"), 5_000.0);
    }

    #[test]
    fn test_withdrawal_order_marks_unfunded_shortfall_when_all_buckets_exhausted() {
        let mut state = HouseholdBucketState::from_balances(vec![("taxable", 5_000.0)]);

        let result = fund_household_deficit(&mut state, 10_000.0, &["taxable"]);

        assert!(!result.funded);
        assert_eq!(result.remaining_shortfall, 5_000.0);
    }
}
