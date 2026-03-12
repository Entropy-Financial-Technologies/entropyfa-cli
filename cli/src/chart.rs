use entropyfa_engine::models::simulation_request::{CashFlow, SimulationRequest};
use entropyfa_engine::models::simulation_response::{
    BalanceHistogram, LinearResult, MonteCarloResult,
};
use ratatui::{
    backend::CrosstermBackend,
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    symbols::Marker,
    text::{Line, Span},
    widgets::{Axis, Block, Chart, Dataset, GraphType, Paragraph, Widget},
};
use std::collections::HashSet;
use std::io::{self, IsTerminal, Write};

const CHART_LINE_COLOR: Color = Color::Blue;
const LINEAR_COLOR: Color = Color::Cyan;
const TITLE_COLOR: Color = Color::Reset;
const AXIS_COLOR: Color = Color::Gray;
const LABEL_COLOR: Color = Color::Gray;
const DIM: Color = Color::DarkGray;
const PANEL_BORDER_COLOR: Color = Color::DarkGray;
const SUCCESS_HIGH_COLOR: Color = Color::Green;
const SUCCESS_WARN_COLOR: Color = Color::Yellow;
const SUCCESS_LOW_COLOR: Color = Color::Red;

const DASHBOARD_HEIGHT: u16 = 57;
const MIN_WIDTH: u16 = 72;
const RIGHT_LABEL_WIDTH: u16 = 16;
const BRAILLE_X_RESOLUTION: usize = 2;

fn interpolate_series(months: &[u32], values: &[f64], num_points: usize) -> Vec<f64> {
    if values.is_empty() || num_points == 0 {
        return vec![];
    }
    if num_points == 1 || values.len() == 1 {
        return vec![values[0]];
    }

    let max_month = *months.last().unwrap_or(&0) as f64;
    let mut result = Vec::with_capacity(num_points);
    for i in 0..num_points {
        let t = max_month * (i as f64 / (num_points - 1) as f64);
        let mut idx = 0;
        while idx + 1 < months.len() && (months[idx + 1] as f64) < t {
            idx += 1;
        }
        if idx + 1 >= months.len() {
            result.push(*values.last().unwrap_or(&0.0));
        } else {
            let m0 = months[idx] as f64;
            let m1 = months[idx + 1] as f64;
            let span = m1 - m0;
            if span <= 0.0 {
                result.push(values[idx]);
            } else {
                let frac = (t - m0) / span;
                result.push(values[idx] + (values[idx + 1] - values[idx]) * frac);
            }
        }
    }
    result
}

fn interpolate_points(months: &[u32], values: &[f64], num_points: usize) -> Vec<(f64, f64)> {
    let series = interpolate_series(months, values, num_points);
    let total_years = months.last().copied().unwrap_or(0) as f64 / 12.0;

    if series.len() <= 1 {
        return series.into_iter().map(|y| (0.0, y)).collect();
    }

    series
        .into_iter()
        .enumerate()
        .map(|(i, y)| (total_years * (i as f64 / (num_points - 1) as f64), y))
        .collect()
}

fn format_dollars(v: f64) -> String {
    let sign = if v < 0.0 { "-" } else { "" };
    let abs = v.abs();

    if abs >= 1_000_000.0 {
        format!("{sign}${:.1}M", abs / 1_000_000.0)
    } else if abs >= 1_000.0 {
        format!("{sign}${:.0}K", abs / 1_000.0)
    } else {
        format!("{sign}${:.0}", abs)
    }
}

fn format_percent(v: f64) -> String {
    format!("{:.0}%", v * 100.0)
}

fn format_rate(v: f64) -> String {
    format!("{:.1}%", v * 100.0)
}

fn fmt_count(n: u32) -> String {
    if n >= 1000 {
        format!("{},{:03}", n / 1000, n % 1000)
    } else {
        n.to_string()
    }
}

fn format_horizon(months: u32) -> String {
    let years = months / 12;
    let rem_months = months % 12;
    if rem_months == 0 {
        format!("{years}y")
    } else if years == 0 {
        format!("{rem_months}m")
    } else {
        format!("{years}y {rem_months}m")
    }
}

fn format_cash_flow(cf: &CashFlow, horizon_months: u32) -> String {
    let freq = match cf.frequency.as_str() {
        "monthly" => "mo",
        "annually" => "yr",
        other => other,
    };
    let mut summary = format!("{}/{}", format_dollars(cf.amount), freq);
    let start = cf.start_month.unwrap_or(0);
    let end = cf.end_month.unwrap_or(horizon_months);
    if start > 0 || end < horizon_months {
        summary.push_str(&format!(" @m{}-{}", start + 1, end));
    }
    summary
}

fn request_summary_lines(req: &SimulationRequest) -> [Line<'static>; 2] {
    let mode = req.mode.as_deref().unwrap_or("both");
    let mode_label = match mode {
        "monte_carlo" => "MC",
        "linear" => "Linear",
        _ => "Both",
    };

    let mut top = vec![
        Span::styled("Inputs: ", Style::default().fg(LABEL_COLOR)),
        Span::styled(
            format!("start {}", format_dollars(req.starting_balance)),
            Style::default().fg(TITLE_COLOR),
        ),
        Span::styled("  |  ", Style::default().fg(DIM)),
        Span::styled(
            format!("horizon {}", format_horizon(req.time_horizon_months)),
            Style::default().fg(TITLE_COLOR),
        ),
        Span::styled("  |  ", Style::default().fg(DIM)),
        Span::styled(
            format!(
                "return {} / vol {}",
                format_rate(req.return_assumption.annual_mean),
                format_rate(req.return_assumption.annual_std_dev)
            ),
            Style::default().fg(TITLE_COLOR),
        ),
        Span::styled("  |  ", Style::default().fg(DIM)),
        Span::styled(
            format!("mode {mode_label}"),
            Style::default().fg(TITLE_COLOR),
        ),
    ];
    if let Some(seed) = req.seed {
        top.push(Span::styled("  |  ", Style::default().fg(DIM)));
        top.push(Span::styled(
            format!("seed {seed}"),
            Style::default().fg(TITLE_COLOR),
        ));
    }

    let flows = if req.cash_flows.is_empty() {
        "none".to_string()
    } else if req.cash_flows.len() <= 2 {
        req.cash_flows
            .iter()
            .map(|cf| format_cash_flow(cf, req.time_horizon_months))
            .collect::<Vec<_>>()
            .join("  |  ")
    } else {
        let preview = req
            .cash_flows
            .iter()
            .take(2)
            .map(|cf| format_cash_flow(cf, req.time_horizon_months))
            .collect::<Vec<_>>()
            .join("  |  ");
        format!("{preview}  |  +{} more", req.cash_flows.len() - 2)
    };

    [
        Line::from(top),
        Line::from(vec![
            Span::styled("Cash Flows: ", Style::default().fg(LABEL_COLOR)),
            Span::styled(flows, Style::default().fg(TITLE_COLOR)),
        ]),
    ]
}

fn chart_x_step(total_years: u32) -> u32 {
    if total_years <= 10 {
        1
    } else if total_years <= 20 {
        2
    } else if total_years <= 40 {
        5
    } else {
        10
    }
}

fn histogram_points(histogram: &BalanceHistogram, max_bins: usize) -> Vec<(f64, f64)> {
    if histogram.centers.is_empty() || histogram.percentages.is_empty() {
        return vec![];
    }
    if histogram.centers.len() <= max_bins || max_bins == 0 {
        return histogram
            .centers
            .iter()
            .zip(&histogram.percentages)
            .map(|(&x, &y)| (x, y * 100.0))
            .collect();
    }

    let chunk_size = histogram.centers.len().div_ceil(max_bins);
    let mut result = Vec::new();
    for idx in (0..histogram.centers.len()).step_by(chunk_size) {
        let end = (idx + chunk_size).min(histogram.centers.len());
        let centers = &histogram.centers[idx..end];
        let pct = histogram.percentages[idx..end].iter().sum::<f64>() * 100.0;
        let center = centers.iter().sum::<f64>() / centers.len() as f64;
        result.push((center, pct));
    }
    result
}

struct ChartBands<'a> {
    lower_label: &'static str,
    lower_values: &'a [f64],
    median_values: &'a [f64],
    upper_label: &'static str,
    upper_values: &'a [f64],
}

fn chart_bands<'a>(mc: &'a MonteCarloResult) -> ChartBands<'a> {
    if let Some(custom) = mc.custom_percentile_series.as_ref() {
        if let (Some(p20), Some(p80)) = (custom.get("p20"), custom.get("p80")) {
            return ChartBands {
                lower_label: "p20",
                lower_values: p20,
                median_values: &mc.time_series.p50,
                upper_label: "p80",
                upper_values: p80,
            };
        }
    }

    ChartBands {
        lower_label: "p10",
        lower_values: &mc.time_series.p10,
        median_values: &mc.time_series.p50,
        upper_label: "p90",
        upper_values: &mc.time_series.p90,
    }
}

pub fn render_projection_dashboard(
    req: &SimulationRequest,
    mc: &MonteCarloResult,
    linear: Option<&LinearResult>,
    computation_time_ms: f64,
) {
    let stderr = io::stderr();
    if !stderr.is_terminal() {
        return;
    }

    let ts = &mc.time_series;
    if ts.months.len() < 2 {
        return;
    }

    let term_width = crossterm::terminal::size().map(|(w, _)| w).unwrap_or(100);
    if term_width < MIN_WIDTH {
        return;
    }

    let bands = chart_bands(mc);
    let total_years = ts.months.last().copied().unwrap_or(0) as f64 / 12.0;
    let display_years = total_years.ceil() as u32;

    let mut min_y = bands
        .lower_values
        .iter()
        .copied()
        .fold(f64::INFINITY, f64::min);
    let mut max_y = bands
        .upper_values
        .iter()
        .copied()
        .fold(f64::NEG_INFINITY, f64::max);

    if let Some(linear) = linear {
        if let Some(&min_linear) = linear
            .time_series
            .balance
            .iter()
            .min_by(|a, b| a.total_cmp(b))
        {
            min_y = min_y.min(min_linear);
        }
        if let Some(&max_linear) = linear
            .time_series
            .balance
            .iter()
            .max_by(|a, b| a.total_cmp(b))
        {
            max_y = max_y.max(max_linear);
        }
    }

    let range = max_y - min_y;
    if range <= 0.0 {
        return;
    }

    let padding = range * 0.08;
    let y_min = if min_y >= 0.0 { 0.0 } else { min_y - padding };
    let y_max = max_y + padding;

    let plot_cols = term_width.saturating_sub(RIGHT_LABEL_WIDTH + 12) as usize;
    if plot_cols < 12 {
        return;
    }

    let plot_points = plot_cols.saturating_mul(BRAILLE_X_RESOLUTION);
    let lower_band = interpolate_points(&ts.months, bands.lower_values, plot_points);
    let median_band = interpolate_points(&ts.months, bands.median_values, plot_points);
    let upper_band = interpolate_points(&ts.months, bands.upper_values, plot_points);
    let linear_band = linear.map(|result| {
        interpolate_points(
            &result.time_series.months,
            &result.time_series.balance,
            plot_points,
        )
    });

    let area = Rect::new(0, 0, term_width, DASHBOARD_HEIGHT);
    let mut buffer = Buffer::empty(area);

    render_dashboard(
        &mut buffer,
        area,
        req,
        mc,
        linear,
        computation_time_ms,
        total_years,
        display_years,
        y_min,
        y_max,
        &lower_band,
        &median_band,
        &upper_band,
        linear_band.as_deref(),
        bands.lower_label,
        bands.upper_label,
        *bands.lower_values.last().unwrap_or(&0.0),
        *bands.median_values.last().unwrap_or(&0.0),
        *bands.upper_values.last().unwrap_or(&0.0),
    );

    let mut stderr_lock = stderr.lock();
    let mut backend = CrosstermBackend::new(&mut stderr_lock);
    let _ = write_buffer(&mut backend, &buffer);
    let _ = writeln!(stderr_lock);
}

#[allow(clippy::too_many_arguments)]
fn render_dashboard(
    buffer: &mut Buffer,
    area: Rect,
    req: &SimulationRequest,
    mc: &MonteCarloResult,
    linear: Option<&LinearResult>,
    computation_time_ms: f64,
    total_years: f64,
    display_years: u32,
    y_min: f64,
    y_max: f64,
    lower_band: &[(f64, f64)],
    median_band: &[(f64, f64)],
    upper_band: &[(f64, f64)],
    linear_band: Option<&[(f64, f64)]>,
    lower_label: &'static str,
    upper_label: &'static str,
    lower_final: f64,
    median_final: f64,
    upper_final: f64,
) {
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4),
            Constraint::Length(5),
            Constraint::Length(30),
            Constraint::Length(15),
            Constraint::Length(2),
            Constraint::Length(1),
        ])
        .split(area);

    render_header(buffer, rows[0], req, mc, computation_time_ms, display_years);
    render_stat_cards(buffer, rows[1], mc, linear);
    render_projection_chart(
        buffer,
        rows[2],
        total_years,
        y_min,
        y_max,
        lower_band,
        median_band,
        upper_band,
        linear_band,
        lower_label,
        upper_label,
        lower_final,
        median_final,
        upper_final,
        linear.map(|result| result.final_balance),
    );

    let bottom = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(rows[3]);
    render_survival_panel(buffer, bottom[0], mc, total_years);
    render_distribution_panel(buffer, bottom[1], mc);

    render_linear_summary(buffer, rows[4], mc, linear);
    render_footer(buffer, rows[5], mc, computation_time_ms);
}

fn render_header(
    buffer: &mut Buffer,
    area: Rect,
    req: &SimulationRequest,
    mc: &MonteCarloResult,
    computation_time_ms: f64,
    display_years: u32,
) {
    let title = Line::from(vec![
        Span::styled(
            "Monte Carlo Projection",
            Style::default().fg(CHART_LINE_COLOR),
        ),
        Span::styled(
            format!(
                "  {} sims | {} years | {:.0}ms",
                fmt_count(mc.num_simulations),
                display_years,
                computation_time_ms
            ),
            Style::default().fg(DIM),
        ),
    ]);
    let [input_line, flow_line] = request_summary_lines(req);
    Paragraph::new(vec![title, input_line, flow_line]).render(area, buffer);
}

fn render_stat_cards(
    buffer: &mut Buffer,
    area: Rect,
    mc: &MonteCarloResult,
    linear: Option<&LinearResult>,
) {
    let bands = chart_bands(mc);
    let success_color = if mc.success_rate >= 0.82 {
        SUCCESS_HIGH_COLOR
    } else if mc.success_rate >= 0.70 {
        SUCCESS_WARN_COLOR
    } else {
        SUCCESS_LOW_COLOR
    };

    let cards = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Fill(1); 5])
        .split(area);

    render_stat_card(
        buffer,
        cards[0],
        "Success Rate",
        &format_percent(mc.success_rate),
        "survival",
        success_color,
    );
    render_stat_card(
        buffer,
        cards[1],
        "Median Final",
        &format_dollars(mc.percentiles.p50),
        "p50",
        CHART_LINE_COLOR,
    );
    render_stat_card(
        buffer,
        cards[2],
        "Downside",
        &format_dollars(*bands.lower_values.last().unwrap_or(&0.0)),
        bands.lower_label,
        CHART_LINE_COLOR,
    );
    render_stat_card(
        buffer,
        cards[3],
        "Upside",
        &format_dollars(*bands.upper_values.last().unwrap_or(&0.0)),
        bands.upper_label,
        CHART_LINE_COLOR,
    );
    if let Some(linear) = linear {
        render_stat_card(
            buffer,
            cards[4],
            "Linear Final",
            &format_dollars(linear.final_balance),
            "deterministic",
            LINEAR_COLOR,
        );
    } else {
        render_stat_card(
            buffer,
            cards[4],
            "Depleted",
            &format_percent(mc.balance_histogram.depleted_pct),
            "terminal",
            SUCCESS_LOW_COLOR,
        );
    }
}

fn render_stat_card(
    buffer: &mut Buffer,
    area: Rect,
    title: &str,
    value: &str,
    subvalue: &str,
    value_color: Color,
) {
    let block = Block::bordered()
        .border_style(Style::default().fg(PANEL_BORDER_COLOR))
        .title(Line::from(Span::styled(
            format!(" {title} "),
            Style::default().fg(LABEL_COLOR),
        )));
    let inner = block.inner(area);
    block.render(area, buffer);

    Paragraph::new(vec![
        Line::from(Span::styled(
            value.to_string(),
            Style::default().fg(value_color),
        )),
        Line::from(Span::styled(subvalue.to_string(), Style::default().fg(DIM))),
    ])
    .render(inner, buffer);
}

#[allow(clippy::too_many_arguments)]
fn render_projection_chart(
    buffer: &mut Buffer,
    area: Rect,
    total_years: f64,
    y_min: f64,
    y_max: f64,
    lower_band: &[(f64, f64)],
    median_band: &[(f64, f64)],
    upper_band: &[(f64, f64)],
    linear_band: Option<&[(f64, f64)]>,
    lower_label: &'static str,
    upper_label: &'static str,
    lower_final: f64,
    median_final: f64,
    upper_final: f64,
    linear_final: Option<f64>,
) {
    let block = Block::bordered()
        .border_style(Style::default().fg(PANEL_BORDER_COLOR))
        .title(Line::from(Span::styled(
            " Portfolio Projection ",
            Style::default().fg(LABEL_COLOR),
        )));
    let inner = block.inner(area);
    block.render(area, buffer);

    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(10), Constraint::Length(RIGHT_LABEL_WIDTH)])
        .split(inner);

    let years_for_step = total_years.ceil() as u32;
    let x_step = chart_x_step(years_for_step);
    let mut x_labels: Vec<Line> = (0..=years_for_step)
        .step_by(x_step as usize)
        .map(|year| Line::from(year.to_string()))
        .collect();
    if x_labels.len() < 2 {
        x_labels.push(Line::from(years_for_step.to_string()));
    }

    let y_mid = (y_min + y_max) / 2.0;
    let y_labels = vec![
        Line::from(format_dollars(y_min)),
        Line::from(format_dollars(y_mid)),
        Line::from(format_dollars(y_max)),
    ];

    let mut datasets: Vec<Dataset<'_>> = vec![
        Dataset::default()
            .marker(Marker::Braille)
            .graph_type(GraphType::Line)
            .style(Style::default().fg(CHART_LINE_COLOR))
            .data(lower_band),
        Dataset::default()
            .marker(Marker::Braille)
            .graph_type(GraphType::Line)
            .style(Style::default().fg(CHART_LINE_COLOR))
            .data(upper_band),
        Dataset::default()
            .marker(Marker::Braille)
            .graph_type(GraphType::Line)
            .style(Style::default().fg(CHART_LINE_COLOR))
            .data(median_band),
    ];
    if let Some(linear_band) = linear_band {
        datasets.push(
            Dataset::default()
                .marker(Marker::Braille)
                .graph_type(GraphType::Line)
                .style(Style::default().fg(LINEAR_COLOR))
                .data(linear_band),
        );
    }

    Chart::new(datasets)
        .x_axis(
            Axis::default()
                .bounds([0.0, total_years.max(1.0)])
                .labels(x_labels)
                .style(Style::default().fg(AXIS_COLOR))
                .title(Line::from("yrs")),
        )
        .y_axis(
            Axis::default()
                .bounds([y_min, y_max])
                .labels(y_labels)
                .style(Style::default().fg(AXIS_COLOR)),
        )
        .render(columns[0], buffer);

    let right_rows = columns[1].height as usize;
    let val_to_row = |val: f64| -> usize {
        let frac = (y_max - val) / (y_max - y_min);
        (frac * (right_rows.saturating_sub(1)) as f64)
            .round()
            .clamp(0.0, right_rows.saturating_sub(1) as f64) as usize
    };

    let mut labels: Vec<(usize, &str, f64, Color)> = vec![
        (
            val_to_row(upper_final),
            upper_label,
            upper_final,
            CHART_LINE_COLOR,
        ),
        (
            val_to_row(median_final),
            "p50",
            median_final,
            CHART_LINE_COLOR,
        ),
        (
            val_to_row(lower_final),
            lower_label,
            lower_final,
            CHART_LINE_COLOR,
        ),
    ];
    if let Some(linear_final) = linear_final {
        labels.push((val_to_row(linear_final), "lin", linear_final, LINEAR_COLOR));
    }
    labels.sort_by_key(|(row, _, _, _)| *row);

    let mut right_lines: Vec<Line> = vec![Line::from(""); right_rows];
    let mut used_rows = HashSet::new();
    for &(row, name, val, color) in &labels {
        let mut r = row;
        while used_rows.contains(&r) && r + 1 < right_rows {
            r += 1;
        }
        if r < right_rows && !used_rows.contains(&r) {
            used_rows.insert(r);
            right_lines[r] = Line::from(vec![
                Span::styled(format!(" {name}"), Style::default().fg(color)),
                Span::styled(
                    format!(" {}", format_dollars(val)),
                    Style::default().fg(LABEL_COLOR),
                ),
            ]);
        }
    }
    Paragraph::new(right_lines).render(columns[1], buffer);
}

fn render_survival_panel(buffer: &mut Buffer, area: Rect, mc: &MonteCarloResult, total_years: f64) {
    let block = Block::bordered()
        .border_style(Style::default().fg(PANEL_BORDER_COLOR))
        .title(Line::from(Span::styled(
            " Longevity Risk ",
            Style::default().fg(LABEL_COLOR),
        )));
    let inner = block.inner(area);
    block.render(area, buffer);

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(4), Constraint::Length(1)])
        .split(inner);

    let survival_points: Vec<(f64, f64)> = mc
        .survival_by_year
        .iter()
        .enumerate()
        .map(|(i, rate)| ((i + 1) as f64, rate * 100.0))
        .collect();
    let final_survival = *mc.survival_by_year.last().unwrap_or(&mc.success_rate);
    let confidence_year = mc
        .survival_by_year
        .iter()
        .enumerate()
        .rev()
        .find(|(_, rate)| **rate >= 0.90)
        .map(|(i, _)| i + 1);

    let axis_step = chart_x_step(total_years.ceil() as u32);
    let x_labels: Vec<Line> = (0..=total_years.ceil() as u32)
        .step_by(axis_step as usize)
        .map(|year| Line::from(year.to_string()))
        .collect();
    let y_labels = vec![Line::from("0%"), Line::from("50%"), Line::from("100%")];

    Chart::new(vec![Dataset::default()
        .marker(Marker::HalfBlock)
        .graph_type(GraphType::Bar)
        .style(Style::default().fg(CHART_LINE_COLOR))
        .data(&survival_points)])
    .x_axis(
        Axis::default()
            .bounds([0.0, total_years.max(1.0)])
            .labels(x_labels)
            .style(Style::default().fg(AXIS_COLOR))
            .title(Line::from("yrs")),
    )
    .y_axis(
        Axis::default()
            .bounds([0.0, 100.0])
            .labels(y_labels)
            .style(Style::default().fg(AXIS_COLOR)),
    )
    .render(rows[0], buffer);

    let footer = if let Some(confidence_year) = confidence_year {
        format!(
            "conf yr {} | final {}",
            confidence_year,
            format_percent(final_survival)
        )
    } else {
        format!("final {}", format_percent(final_survival))
    };
    Paragraph::new(Line::from(Span::styled(footer, Style::default().fg(DIM))))
        .render(rows[1], buffer);
}

fn render_distribution_panel(buffer: &mut Buffer, area: Rect, mc: &MonteCarloResult) {
    let block = Block::bordered()
        .border_style(Style::default().fg(PANEL_BORDER_COLOR))
        .title(Line::from(Span::styled(
            " Final Balance Distribution ",
            Style::default().fg(LABEL_COLOR),
        )));
    let inner = block.inner(area);
    block.render(area, buffer);

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(4), Constraint::Length(1)])
        .split(inner);

    let bands = chart_bands(mc);
    let upper_cutoff = *bands.upper_values.last().unwrap_or(&mc.percentiles.p90);
    let depleted_pct = mc.balance_histogram.depleted_pct * 100.0;
    let full_histogram =
        histogram_points(&mc.balance_histogram, mc.balance_histogram.centers.len());
    let positive_histogram: Vec<(f64, f64)> = full_histogram
        .iter()
        .copied()
        .filter(|(x, _)| *x <= upper_cutoff)
        .collect();

    let mut histogram: Vec<(f64, f64)> = Vec::with_capacity(positive_histogram.len() + 1);
    if depleted_pct > 0.0 {
        histogram.push((0.0, depleted_pct));
    }

    if positive_histogram.len() >= 2 {
        histogram.extend(positive_histogram);
    } else {
        histogram.extend(full_histogram);
    }

    if histogram.is_empty() {
        Paragraph::new(Line::from(Span::styled(
            "no positive terminal balances",
            Style::default().fg(DIM),
        )))
        .render(rows[0], buffer);
        return;
    }

    let x_min = 0.0;
    let x_max = upper_cutoff
        .max(histogram.last().map(|(x, _)| *x).unwrap_or(1.0))
        .max(1.0);
    let x_mid = (x_min + x_max) / 2.0;
    let y_max = histogram
        .iter()
        .map(|(_, pct)| *pct)
        .fold(0.0_f64, f64::max)
        .max(1.0);

    Chart::new(vec![Dataset::default()
        .marker(Marker::HalfBlock)
        .graph_type(GraphType::Bar)
        .style(Style::default().fg(CHART_LINE_COLOR))
        .data(&histogram)])
    .x_axis(
        Axis::default()
            .bounds([x_min, x_max])
            .labels(vec![
                Line::from(format_dollars(x_min)),
                Line::from(format_dollars(x_mid)),
                Line::from(format_dollars(x_max)),
            ])
            .style(Style::default().fg(AXIS_COLOR)),
    )
    .y_axis(
        Axis::default()
            .bounds([0.0, y_max])
            .labels(vec![
                Line::from("0%"),
                Line::from(format!("{:.0}%", y_max / 2.0)),
                Line::from(format!("{:.0}%", y_max)),
            ])
            .style(Style::default().fg(AXIS_COLOR)),
    )
    .render(rows[0], buffer);

    let footer = format!(
        "dep {} | cap {} {}",
        format_percent(mc.balance_histogram.depleted_pct),
        bands.upper_label,
        format_dollars(upper_cutoff)
    );
    Paragraph::new(Line::from(Span::styled(footer, Style::default().fg(DIM))))
        .render(rows[1], buffer);
}

fn render_linear_summary(
    buffer: &mut Buffer,
    area: Rect,
    mc: &MonteCarloResult,
    linear: Option<&LinearResult>,
) {
    let line = if let Some(linear) = linear {
        Line::from(vec![
            Span::styled("Linear ", Style::default().fg(LABEL_COLOR)),
            Span::styled(
                format!("final {}", format_dollars(linear.final_balance)),
                Style::default().fg(LINEAR_COLOR),
            ),
            Span::styled("  |  ", Style::default().fg(DIM)),
            Span::styled(
                format!("return {}", format_dollars(linear.total_return_earned)),
                Style::default().fg(LABEL_COLOR),
            ),
            Span::styled("  |  ", Style::default().fg(DIM)),
            Span::styled(
                format!(
                    "withdrawals {}",
                    format_dollars(linear.total_withdrawals.abs())
                ),
                Style::default().fg(LABEL_COLOR),
            ),
        ])
    } else {
        Line::from(vec![
            Span::styled("Mean ", Style::default().fg(LABEL_COLOR)),
            Span::styled(
                format_dollars(mc.mean),
                Style::default().fg(CHART_LINE_COLOR),
            ),
            Span::styled("  |  ", Style::default().fg(DIM)),
            Span::styled(
                format!("Std dev {}", format_dollars(mc.std_dev)),
                Style::default().fg(LABEL_COLOR),
            ),
            Span::styled("  |  ", Style::default().fg(DIM)),
            Span::styled(
                format!(
                    "p5-p95 {} to {}",
                    format_dollars(mc.percentiles.p5),
                    format_dollars(mc.percentiles.p95)
                ),
                Style::default().fg(LABEL_COLOR),
            ),
        ])
    };

    Paragraph::new(vec![line]).render(area, buffer);
}

fn render_footer(buffer: &mut Buffer, area: Rect, mc: &MonteCarloResult, computation_time_ms: f64) {
    let footer = Line::from(vec![Span::styled(
        format!(
            "Computed in {:.0}ms across {} simulations",
            computation_time_ms,
            fmt_count(mc.num_simulations)
        ),
        Style::default().fg(DIM),
    )]);
    Paragraph::new(vec![footer]).render(area, buffer);
}

fn write_buffer<W: Write>(backend: &mut CrosstermBackend<W>, buffer: &Buffer) -> io::Result<()> {
    use crossterm::{
        queue,
        style::{Attribute as CAttribute, Colors, SetAttribute, SetColors},
    };

    let width = buffer.area.width as usize;
    for row in buffer.content.chunks(width) {
        let mut current_fg = Color::Reset;
        let mut current_bg = Color::Reset;
        let mut current_modifier = ratatui::style::Modifier::empty();

        for cell in row {
            if cell.modifier != current_modifier {
                queue!(backend, SetAttribute(CAttribute::Reset))?;
                current_modifier = cell.modifier;
                if cell.modifier.contains(ratatui::style::Modifier::BOLD) {
                    queue!(backend, SetAttribute(CAttribute::Bold))?;
                }
                if cell.modifier.contains(ratatui::style::Modifier::ITALIC) {
                    queue!(backend, SetAttribute(CAttribute::Italic))?;
                }
                if cell.modifier.contains(ratatui::style::Modifier::UNDERLINED) {
                    queue!(backend, SetAttribute(CAttribute::Underlined))?;
                }
                current_fg = Color::Reset;
                current_bg = Color::Reset;
            }

            if cell.fg != current_fg || cell.bg != current_bg {
                queue!(
                    backend,
                    SetColors(Colors::new(cell.fg.into(), cell.bg.into()))
                )?;
                current_fg = cell.fg;
                current_bg = cell.bg;
            }

            write!(backend, "{}", cell.symbol())?;
        }

        queue!(
            backend,
            SetAttribute(CAttribute::Reset),
            SetColors(Colors::new(Color::Reset.into(), Color::Reset.into()))
        )?;
        write!(backend, "\r\n")?;
    }

    backend.flush()
}
