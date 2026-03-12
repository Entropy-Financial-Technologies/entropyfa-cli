use std::collections::BTreeMap;

use crate::models::simulation_response::{
    BalanceHistogram, MonteCarloDetailRow, MonteCarloResult, Percentiles, TimeSeries,
};

/// Compute a percentile from a sorted slice. Uses linear interpolation.
pub fn percentile(sorted: &[f64], p: f64) -> f64 {
    if sorted.is_empty() {
        return 0.0;
    }
    if sorted.len() == 1 {
        return sorted[0];
    }
    let idx = p / 100.0 * (sorted.len() - 1) as f64;
    let lower = idx.floor() as usize;
    let upper = idx.ceil() as usize;
    if lower == upper {
        sorted[lower]
    } else {
        let frac = idx - lower as f64;
        sorted[lower] * (1.0 - frac) + sorted[upper] * frac
    }
}

pub fn mean(values: &[f64]) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    values.iter().sum::<f64>() / values.len() as f64
}

pub fn std_dev(values: &[f64]) -> f64 {
    if values.len() < 2 {
        return 0.0;
    }
    let m = mean(values);
    let variance = values.iter().map(|v| (v - m).powi(2)).sum::<f64>() / (values.len() - 1) as f64;
    variance.sqrt()
}

/// Build the depletion histogram: categorize paths by when (if ever) their balance hit 0.
pub fn depletion_histogram(paths: &[Vec<f64>], total_months: u32) -> BTreeMap<String, u32> {
    let mut buckets: BTreeMap<String, u32> = BTreeMap::new();

    for path in paths {
        // Find first month where balance reached 0
        let depleted_month = path.iter().skip(1).position(|&b| b <= 0.0);

        let bucket = match depleted_month {
            None => "never".to_string(),
            Some(m) => {
                let month = m as u32;
                // Bucket into 60-month (5-year) ranges
                let bucket_start = (month / 60) * 60;
                let bucket_end = bucket_start + 60;
                if bucket_start == 0 {
                    format!("0_{}", bucket_end)
                } else {
                    format!("{}_{}", bucket_start + 1, bucket_end.min(total_months))
                }
            }
        };

        *buckets.entry(bucket).or_insert(0) += 1;
    }

    buckets
}

/// Compute year-by-year survival rates: fraction of paths with balance > 0 at each year-end.
pub fn survival_by_year(paths: &[Vec<f64>], total_months: u32) -> Vec<f64> {
    let num_years = (total_months / 12) as usize;
    let num_paths = paths.len() as f64;
    (1..=num_years)
        .map(|yr| {
            let month_idx = yr * 12;
            let alive = paths.iter().filter(|p| p[month_idx] > 0.0).count() as f64;
            round4(alive / num_paths)
        })
        .collect()
}

/// Build a histogram of terminal balances for the distribution bell curve.
/// Uses log-spaced bins for positive balances (makes log-normal look bell-shaped).
/// Depleted paths ($0) are reported separately as `depleted_pct`.
pub fn balance_histogram(sorted_balances: &[f64]) -> BalanceHistogram {
    let num_bins = 60;
    let n = sorted_balances.len() as f64;
    let empty = BalanceHistogram {
        centers: vec![],
        percentages: vec![],
        depleted_pct: 0.0,
    };

    if sorted_balances.is_empty() {
        return empty;
    }

    // Separate depleted ($0) from surviving paths
    let depleted = sorted_balances.iter().filter(|&&b| b <= 0.0).count();
    let depleted_pct = round4(depleted as f64 / n);
    let positive: Vec<f64> = sorted_balances
        .iter()
        .copied()
        .filter(|&b| b > 0.0)
        .collect();

    if positive.is_empty() {
        return BalanceHistogram {
            centers: vec![],
            percentages: vec![],
            depleted_pct,
        };
    }

    let log_min = positive.first().unwrap().ln();
    let log_max = positive.last().unwrap().ln();

    if (log_max - log_min).abs() < 1e-6 {
        // All positive balances are essentially the same value
        return BalanceHistogram {
            centers: vec![round2(positive[0])],
            percentages: vec![round4(positive.len() as f64 / n)],
            depleted_pct,
        };
    }

    let log_width = (log_max - log_min) / num_bins as f64;
    let mut counts = vec![0u32; num_bins];

    for &val in &positive {
        let idx = ((val.ln() - log_min) / log_width).floor() as usize;
        counts[idx.min(num_bins - 1)] += 1;
    }

    // Bin centers in dollar-space (exp of log-space midpoint)
    let centers: Vec<f64> = (0..num_bins)
        .map(|i| round2((log_min + log_width * (i as f64 + 0.5)).exp()))
        .collect();
    let percentages: Vec<f64> = counts.iter().map(|&c| round4(c as f64 / n)).collect();

    BalanceHistogram {
        centers,
        percentages,
        depleted_pct,
    }
}

/// Build time-series percentile bands at annual (every 12 months) intervals.
pub fn time_series_bands(paths: &[Vec<f64>], total_months: u32) -> TimeSeries {
    let mut months = Vec::new();
    let mut p10_series = Vec::new();
    let mut p25_series = Vec::new();
    let mut p50_series = Vec::new();
    let mut p75_series = Vec::new();
    let mut p90_series = Vec::new();

    // Sample at month 0, then every 12 months, always include final month
    let mut sample_months: Vec<u32> = (0..=total_months).step_by(12).collect();
    if *sample_months.last().unwrap_or(&0) != total_months {
        sample_months.push(total_months);
    }

    for &month in &sample_months {
        let mut values: Vec<f64> = paths.iter().map(|p| p[month as usize]).collect();
        values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

        months.push(month);
        p10_series.push(round2(percentile(&values, 10.0)));
        p25_series.push(round2(percentile(&values, 25.0)));
        p50_series.push(round2(percentile(&values, 50.0)));
        p75_series.push(round2(percentile(&values, 75.0)));
        p90_series.push(round2(percentile(&values, 90.0)));
    }

    TimeSeries {
        months,
        p10: p10_series,
        p25: p25_series,
        p50: p50_series,
        p75: p75_series,
        p90: p90_series,
    }
}

/// Compute MC detail rows with percentile bands at each sample period.
pub fn compute_mc_detail(
    paths: &[Vec<f64>],
    monthly_cash_flows: &[f64],
    survival: &[f64],
    granularity: &str,
    total_months: u32,
) -> Vec<MonteCarloDetailRow> {
    let step: usize = if granularity == "monthly" { 1 } else { 12 };
    let mut details = Vec::new();
    let mut cum_cash_flow = 0.0;

    let mut period_start = 0usize;
    while period_start < total_months as usize {
        let period_end = (period_start + step).min(total_months as usize);

        // Net cash flow for this period (deterministic)
        let net_cf: f64 = monthly_cash_flows[period_start..period_end].iter().sum();
        cum_cash_flow += net_cf;

        // Gather balances at period_end from all paths
        let mut balances: Vec<f64> = paths.iter().map(|p| p[period_end]).collect();
        balances.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

        // Survival rate: use precomputed year-end rates when aligned, else compute
        let year_idx = period_end / 12;
        let survival_rate =
            if period_end.is_multiple_of(12) && year_idx > 0 && year_idx <= survival.len() {
                survival[year_idx - 1]
            } else {
                let alive = paths.iter().filter(|p| p[period_end] > 0.0).count() as f64;
                round4(alive / paths.len() as f64)
            };

        details.push(MonteCarloDetailRow {
            period: period_start as u32,
            year: period_start as f64 / 12.0,
            balance_p10: round2(percentile(&balances, 10.0)),
            balance_p25: round2(percentile(&balances, 25.0)),
            balance_p50: round2(percentile(&balances, 50.0)),
            balance_p75: round2(percentile(&balances, 75.0)),
            balance_p90: round2(percentile(&balances, 90.0)),
            net_cash_flow: round2(net_cf),
            cumulative_cash_flow: round2(cum_cash_flow),
            survival_rate,
        });

        period_start = period_end;
    }

    details
}

/// Compute custom percentile time series at annual sample points.
pub fn custom_percentile_series(
    paths: &[Vec<f64>],
    total_months: u32,
    percentiles_list: &[u32],
) -> BTreeMap<String, Vec<f64>> {
    let mut sample_months: Vec<u32> = (0..=total_months).step_by(12).collect();
    if *sample_months.last().unwrap_or(&0) != total_months {
        sample_months.push(total_months);
    }

    let mut result: BTreeMap<String, Vec<f64>> = BTreeMap::new();
    for &p in percentiles_list {
        result.insert(format!("p{}", p), Vec::with_capacity(sample_months.len()));
    }

    for &month in &sample_months {
        let mut values: Vec<f64> = paths.iter().map(|path| path[month as usize]).collect();
        values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        for &p in percentiles_list {
            let val = round2(percentile(&values, p as f64));
            result.get_mut(&format!("p{}", p)).unwrap().push(val);
        }
    }

    result
}

/// Compute full Monte Carlo statistics from terminal balances and paths.
pub fn compute_mc_stats(
    terminal_balances: &mut [f64],
    paths: &[Vec<f64>],
    total_months: u32,
    num_simulations: u32,
    include_detail: bool,
    detail_granularity: &str,
    monthly_cash_flows: &[f64],
    custom_percentiles: Option<&[u32]>,
) -> MonteCarloResult {
    terminal_balances.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

    let success_count = terminal_balances.iter().filter(|&&b| b > 0.0).count();
    let survival = survival_by_year(paths, total_months);

    let annual_detail = if include_detail {
        Some(compute_mc_detail(
            paths,
            monthly_cash_flows,
            &survival,
            detail_granularity,
            total_months,
        ))
    } else {
        None
    };

    MonteCarloResult {
        num_simulations,
        percentiles: Percentiles {
            p5: round2(percentile(terminal_balances, 5.0)),
            p10: round2(percentile(terminal_balances, 10.0)),
            p25: round2(percentile(terminal_balances, 25.0)),
            p50: round2(percentile(terminal_balances, 50.0)),
            p75: round2(percentile(terminal_balances, 75.0)),
            p90: round2(percentile(terminal_balances, 90.0)),
            p95: round2(percentile(terminal_balances, 95.0)),
        },
        mean: round2(mean(terminal_balances)),
        std_dev: round2(std_dev(terminal_balances)),
        min: round2(terminal_balances.first().copied().unwrap_or(0.0)),
        max: round2(terminal_balances.last().copied().unwrap_or(0.0)),
        success_rate: round4(success_count as f64 / num_simulations as f64),
        paths_depleted_by_month: depletion_histogram(paths, total_months),
        survival_by_year: survival,
        balance_histogram: balance_histogram(terminal_balances),
        time_series: time_series_bands(paths, total_months),
        annual_detail,
        sample_paths: None,
        custom_percentile_series: custom_percentiles
            .map(|pcts| custom_percentile_series(paths, total_months, pcts)),
    }
}

fn round2(v: f64) -> f64 {
    (v * 100.0).round() / 100.0
}

fn round4(v: f64) -> f64 {
    (v * 10_000.0).round() / 10_000.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_percentile_basic() {
        let sorted: Vec<f64> = (1..=100).map(|x| x as f64).collect();
        assert!((percentile(&sorted, 50.0) - 50.5).abs() < 0.01);
        assert!((percentile(&sorted, 0.0) - 1.0).abs() < 0.01);
        assert!((percentile(&sorted, 100.0) - 100.0).abs() < 0.01);
    }

    #[test]
    fn test_mean_basic() {
        assert!((mean(&[1.0, 2.0, 3.0, 4.0, 5.0]) - 3.0).abs() < 1e-10);
    }

    #[test]
    fn test_std_dev_basic() {
        let vals = vec![2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
        let sd = std_dev(&vals);
        // Known std dev for this dataset (sample) ≈ 2.138
        assert!((sd - 2.138).abs() < 0.01);
    }

    #[test]
    fn test_depletion_histogram_no_depletion() {
        let paths = vec![vec![100.0, 110.0, 120.0], vec![100.0, 105.0, 115.0]];
        let hist = depletion_histogram(&paths, 2);
        assert_eq!(hist.get("never"), Some(&2));
    }

    #[test]
    fn test_depletion_histogram_with_depletion() {
        let paths = vec![
            vec![100.0, 50.0, 0.0],    // depleted at month 2 (index 1 after skip)
            vec![100.0, 110.0, 120.0], // never depleted
        ];
        let hist = depletion_histogram(&paths, 2);
        assert_eq!(hist.get("never"), Some(&1));
        // Month index 1 in the path after skip(1) = position 1, which is month 2 in the path
        // But position returns 0-based index of first zero in skip(1) iterator
        // path[2] = 0.0, position in skip(1) = 1, so month=1, bucket = 0_60
        assert_eq!(hist.get("0_60"), Some(&1));
    }

    #[test]
    fn test_time_series_bands() {
        // 3 paths, 24 months
        let paths = vec![
            vec![100.0; 25], // constant 100
            vec![200.0; 25], // constant 200
            vec![300.0; 25], // constant 300
        ];
        let ts = time_series_bands(&paths, 24);
        assert_eq!(ts.months, vec![0, 12, 24]);
        // p50 should be 200 at all points
        assert!(ts.p50.iter().all(|&v| (v - 200.0).abs() < 0.01));
    }
}
