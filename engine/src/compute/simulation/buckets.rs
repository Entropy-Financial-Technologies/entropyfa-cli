use super::normalized::BucketType;

#[derive(Debug, Clone)]
pub struct BucketState {
    pub id: String,
    pub bucket_type: BucketType,
    pub balance: f64,
    pub realized_gain_ratio: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct HouseholdBucketState {
    pub buckets: Vec<BucketState>,
    pub household_cash: f64,
}

impl HouseholdBucketState {
    pub fn from_balances<I, S>(balances: I) -> Self
    where
        I: IntoIterator<Item = (S, f64)>,
        S: Into<String>,
    {
        Self {
            buckets: balances
                .into_iter()
                .map(|(id, balance)| {
                    let id = id.into();
                    BucketState {
                        bucket_type: infer_bucket_type(&id),
                        id,
                        balance,
                        realized_gain_ratio: None,
                    }
                })
                .collect(),
            household_cash: 0.0,
        }
    }

    pub fn balance(&self, id: &str) -> f64 {
        self.buckets
            .iter()
            .find(|bucket| bucket.id == id)
            .map(|bucket| bucket.balance)
            .unwrap_or(0.0)
    }

    pub fn bucket_mut(&mut self, id: &str) -> Option<&mut BucketState> {
        self.buckets.iter_mut().find(|bucket| bucket.id == id)
    }
}

pub fn apply_monthly_returns(state: &mut HouseholdBucketState, monthly_returns: &[f64]) {
    for (bucket, monthly_return) in state.buckets.iter_mut().zip(monthly_returns.iter()) {
        bucket.balance = (bucket.balance * (1.0 + monthly_return)).max(0.0);
    }
}

fn infer_bucket_type(id: &str) -> BucketType {
    let lower = id.to_ascii_lowercase();
    if lower.contains("cash") {
        BucketType::Cash
    } else if lower.contains("roth") || lower.contains("tax_free") || lower.contains("taxfree") {
        BucketType::TaxFree
    } else if lower.contains("taxable") {
        BucketType::Taxable
    } else {
        BucketType::TaxDeferred
    }
}

#[cfg(test)]
mod tests {
    use super::{apply_monthly_returns, HouseholdBucketState};

    #[test]
    fn test_apply_monthly_returns_per_bucket() {
        let mut state = HouseholdBucketState::from_balances(vec![
            ("taxable", 100_000.0),
            ("ira", 200_000.0),
        ]);
        let returns = vec![0.01, 0.02];

        apply_monthly_returns(&mut state, &returns);

        assert_eq!(state.balance("taxable"), 101_000.0);
        assert_eq!(state.balance("ira"), 204_000.0);
    }
}
