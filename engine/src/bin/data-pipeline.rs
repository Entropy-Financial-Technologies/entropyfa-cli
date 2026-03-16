use std::collections::BTreeMap;
use std::env;
use std::fmt::Display;
use std::io::{self, IsTerminal, Write};
use std::path::PathBuf;
use std::process::Command;

use crossterm::{
    queue,
    style::{Attribute as CAttribute, Colors, SetAttribute, SetColors},
};
use entropyfa_engine::data_pipeline;
use ratatui::{
    backend::CrosstermBackend,
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Paragraph, Widget, Wrap},
};

fn main() {
    if let Err(error) = run() {
        eprintln!("error: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args().skip(1);
    let Some(command) = args.next() else {
        print_usage();
        return Err("missing command".into());
    };

    match command.as_str() {
        "prepare" => {
            let mut year = 2026_u32;
            let mut category = None;
            let mut key = None;

            while let Some(arg) = args.next() {
                match arg.as_str() {
                    "--year" => {
                        year = args
                            .next()
                            .ok_or("--year requires a value")?
                            .parse()
                            .map_err(|_| "--year must be a valid integer")?;
                    }
                    "--category" => {
                        category = Some(args.next().ok_or("--category requires a value")?)
                    }
                    "--key" => key = Some(args.next().ok_or("--key requires a value")?),
                    _ => return Err(format!("unknown flag: {arg}").into()),
                }
            }

            let category = category.ok_or("--category is required")?;
            let key = key.ok_or("--key is required")?;
            let prepared = data_pipeline::prepare_run(year, &category, &key)?;

            println!("Prepared run {}", prepared.run_id);
            println!("  run_dir: {}", prepared.run_dir.display());
            println!(
                "  primary_prompt: {}",
                prepared.run_dir.join("primary_prompt.md").display()
            );
            println!(
                "  verifier_prompt: {}",
                prepared.run_dir.join("verifier_prompt.md").display()
            );
            Ok(())
        }
        "status" => {
            let mut year = 2026_u32;
            let mut plain = false;
            let mut force_tui = false;

            while let Some(arg) = args.next() {
                match arg.as_str() {
                    "--year" => {
                        year = args
                            .next()
                            .ok_or("--year requires a value")?
                            .parse()
                            .map_err(|_| "--year must be a valid integer")?;
                    }
                    "--plain" => plain = true,
                    "--tui" => force_tui = true,
                    _ => return Err(format!("unknown flag: {arg}").into()),
                }
            }

            if plain && force_tui {
                return Err("--plain and --tui are mutually exclusive".into());
            }

            let report = data_pipeline::status_report(year)?;
            let use_tui = force_tui || (!plain && io::stdout().is_terminal());
            if use_tui {
                render_status_dashboard(&report)?;
            } else {
                print_status_report_plain(&report);
            }
            Ok(())
        }
        "run-agents" | "run_agents" => {
            let mut year = 2026_u32;
            let mut category = None;
            let mut key = None;
            let mut primary_provider = data_pipeline::AgentProvider::Claude;
            let mut primary_model = String::from("claude-opus-4-6");
            let mut verifier_provider = data_pipeline::AgentProvider::Codex;
            let mut verifier_model = String::from("gpt-5.4");
            let mut primary_binary = None;
            let mut verifier_binary = None;

            while let Some(arg) = args.next() {
                match arg.as_str() {
                    "--year" => {
                        year = args
                            .next()
                            .ok_or("--year requires a value")?
                            .parse()
                            .map_err(|_| "--year must be a valid integer")?;
                    }
                    "--category" => {
                        category = Some(args.next().ok_or("--category requires a value")?)
                    }
                    "--key" => key = Some(args.next().ok_or("--key requires a value")?),
                    "--primary-provider" => {
                        primary_provider = parse_agent_provider(
                            &args.next().ok_or("--primary-provider requires a value")?,
                        )?
                    }
                    "--primary-model" => {
                        primary_model = args.next().ok_or("--primary-model requires a value")?
                    }
                    "--primary-binary" => {
                        primary_binary = Some(PathBuf::from(
                            args.next().ok_or("--primary-binary requires a value")?,
                        ))
                    }
                    "--verifier-provider" => {
                        verifier_provider = parse_agent_provider(
                            &args.next().ok_or("--verifier-provider requires a value")?,
                        )?
                    }
                    "--verifier-model" => {
                        verifier_model = args.next().ok_or("--verifier-model requires a value")?
                    }
                    "--verifier-binary" => {
                        verifier_binary = Some(PathBuf::from(
                            args.next().ok_or("--verifier-binary requires a value")?,
                        ))
                    }
                    _ => return Err(format!("unknown flag: {arg}").into()),
                }
            }

            let category = category.ok_or("--category is required")?;
            let key = key.ok_or("--key is required")?;
            let config = data_pipeline::RunAgentsConfig {
                year,
                category,
                key,
                primary: data_pipeline::AgentInvocationConfig {
                    provider: primary_provider,
                    model: primary_model,
                    binary: primary_binary,
                },
                verifier: data_pipeline::AgentInvocationConfig {
                    provider: verifier_provider,
                    model: verifier_model,
                    binary: verifier_binary,
                },
            };
            let outcome = data_pipeline::run_agents(&config)?;

            println!("Run complete for {}", outcome.prepared.run_id);
            println!("  run_dir: {}", outcome.prepared.run_dir.display());
            println!(
                "  primary: {} {}",
                display_agent_provider(outcome.primary.provider),
                outcome.primary.model
            );
            println!("    stdout: {}", outcome.primary.stdout_log_path.display());
            println!("    stderr: {}", outcome.primary.stderr_log_path.display());
            println!(
                "  verifier: {} {}",
                display_agent_provider(outcome.verifier.provider),
                outcome.verifier.model
            );
            println!("    stdout: {}", outcome.verifier.stdout_log_path.display());
            println!("    stderr: {}", outcome.verifier.stderr_log_path.display());
            println!("  review: {}", outcome.review.review_path.display());
            println!(
                "  review_md: {}",
                outcome.review.review_markdown_path.display()
            );
            println!("  status: {}", outcome.review.status_decision);
            println!(
                "  recommended_action: {}",
                outcome.review.recommended_action
            );
            println!("  approved: {}", outcome.review.approved);
            println!("  warnings: {}", outcome.review.warnings.len());
            println!(
                "  blocking_issues: {}",
                outcome.review.blocking_issues.len()
            );
            print_next_commands(
                &outcome.prepared.run_id,
                &outcome.review.recommended_action,
                outcome.review.approved,
                outcome.review.blocking_issues.is_empty(),
            );
            Ok(())
        }
        "review" => {
            let mut run_ref = None;

            while let Some(arg) = args.next() {
                match arg.as_str() {
                    "--run" => run_ref = Some(args.next().ok_or("--run requires a value")?),
                    _ => return Err(format!("unknown flag: {arg}").into()),
                }
            }

            let run_ref = run_ref.ok_or("--run is required")?;
            let outcome = data_pipeline::review_run(&run_ref)?;
            println!("Review complete for {}", outcome.run_id);
            println!("  review: {}", outcome.review_path.display());
            println!("  review_md: {}", outcome.review_markdown_path.display());
            println!("  status: {}", outcome.status_decision);
            println!("  recommended_action: {}", outcome.recommended_action);
            println!("  approved: {}", outcome.approved);
            println!("  warnings: {}", outcome.warnings.len());
            println!("  blocking_issues: {}", outcome.blocking_issues.len());
            print_next_commands(
                &outcome.run_id,
                &outcome.recommended_action,
                outcome.approved,
                outcome.blocking_issues.is_empty(),
            );
            Ok(())
        }
        "apply" => {
            let mut run_ref = None;

            while let Some(arg) = args.next() {
                match arg.as_str() {
                    "--run" => run_ref = Some(args.next().ok_or("--run requires a value")?),
                    _ => return Err(format!("unknown flag: {arg}").into()),
                }
            }

            let run_ref = run_ref.ok_or("--run is required")?;
            let outcome = data_pipeline::apply_run(&run_ref)?;
            println!("Applied run {}", outcome.run_id);
            println!(
                "  reviewed_artifact: {}",
                outcome.reviewed_artifact_path.display()
            );
            println!(
                "  generated_source: {}",
                outcome.generated_source_path.display()
            );
            println!("  metadata: {}", outcome.metadata_path.display());

            run_post_apply_checks(&outcome)?;
            Ok(())
        }
        "validate" => {
            let mut strict = false;
            let mut metadata_path = data_pipeline::default_metadata_path();
            let mut snapshot_path = data_pipeline::default_snapshot_path();

            while let Some(arg) = args.next() {
                match arg.as_str() {
                    "--strict" => strict = true,
                    "--metadata" => {
                        metadata_path =
                            PathBuf::from(args.next().ok_or("--metadata requires a path")?)
                    }
                    "--snapshot" => {
                        snapshot_path =
                            PathBuf::from(args.next().ok_or("--snapshot requires a path")?)
                    }
                    _ => return Err(format!("unknown flag: {arg}").into()),
                }
            }

            let registry = data_pipeline::load_registry(&metadata_path)?;
            let snapshot = data_pipeline::load_snapshot(&snapshot_path)?;
            let report = data_pipeline::validate_registry(&registry, &snapshot, strict)?;

            println!("Data pipeline validation");
            println!("  metadata: {}", metadata_path.display());
            println!("  snapshot: {}", snapshot_path.display());
            println!("  coverage entries: {}", report.coverage_entries);
            println!("  metadata entries: {}", report.metadata_entries);
            println!("  snapshot entries: {}", report.snapshot_entries);
            println!("  variants checked: {}", report.variants_checked);
            println!("  warnings: {}", report.warnings.len());
            println!("  errors: {}", report.errors.len());

            if !report.warnings.is_empty() {
                println!("\nWarnings:");
                for warning in &report.warnings {
                    println!("  - {warning}");
                }
            }

            if !report.errors.is_empty() {
                println!("\nErrors:");
                for error in &report.errors {
                    println!("  - {error}");
                }
            }

            if !report.is_success(strict) {
                return Err("validation failed".into());
            }

            println!("\nValidation passed");
            Ok(())
        }
        "snapshot" => {
            let mut metadata_path = data_pipeline::default_metadata_path();
            let mut output_path = None;

            while let Some(arg) = args.next() {
                match arg.as_str() {
                    "--metadata" => {
                        metadata_path =
                            PathBuf::from(args.next().ok_or("--metadata requires a path")?)
                    }
                    "--output" => {
                        output_path = Some(PathBuf::from(
                            args.next().ok_or("--output requires a path")?,
                        ))
                    }
                    _ => return Err(format!("unknown flag: {arg}").into()),
                }
            }

            let registry = data_pipeline::load_registry(&metadata_path)?;
            let snapshot = data_pipeline::generate_snapshot(&registry)?;

            if let Some(output_path) = output_path {
                data_pipeline::write_snapshot(&output_path, &snapshot)?;
                println!("Wrote snapshot to {}", output_path.display());
            } else {
                println!("{}", serde_json::to_string_pretty(&snapshot)?);
            }

            Ok(())
        }
        "--help" | "-h" | "help" => {
            print_usage();
            Ok(())
        }
        _ => {
            print_usage();
            Err(format!("unknown command: {command}").into())
        }
    }
}

fn print_usage() {
    eprintln!(
        "usage:\n  cargo run -p entropyfa-engine --bin data-pipeline -- prepare --year 2026 --category insurance --key irmaa_brackets\n  cargo run -p entropyfa-engine --bin data-pipeline -- status [--year 2026] [--plain|--tui]\n  cargo run -p entropyfa-engine --bin data-pipeline -- run-agents --year 2026 --category insurance --key irmaa_brackets [--primary-provider claude] [--primary-model claude-opus-4-6] [--verifier-provider codex] [--verifier-model gpt-5.4]\n  cargo run -p entropyfa-engine --bin data-pipeline -- review --run RUN_ID_OR_PATH\n  cargo run -p entropyfa-engine --bin data-pipeline -- apply --run RUN_ID_OR_PATH\n  cargo run -p entropyfa-engine --bin data-pipeline -- validate [--strict] [--metadata PATH] [--snapshot PATH]\n  cargo run -p entropyfa-engine --bin data-pipeline -- snapshot [--metadata PATH] [--output PATH]"
    );
}

fn print_next_commands(
    run_id: &str,
    recommended_action: &impl Display,
    approved: bool,
    blocking_issues_cleared: bool,
) {
    let action = recommended_action.to_string();
    match (action.as_str(), approved, blocking_issues_cleared) {
        ("apply_approved_result", false, true) => {
            println!("\nNext:");
            println!(
                "  approve: cargo run -p entropyfa-engine --bin data-pipeline -- review --run {}",
                run_id
            );
            println!(
                "  apply:   cargo run -p entropyfa-engine --bin data-pipeline -- apply --run {}",
                run_id
            );
        }
        ("apply_approved_result", true, true) => {
            println!("\nNext:");
            println!(
                "  apply: cargo run -p entropyfa-engine --bin data-pipeline -- apply --run {}",
                run_id
            );
        }
        ("address_verifier_feedback_and_rerun_review", _, _) => {
            println!("\nNext:");
            println!("  inspect: review_md for the blocking issues");
            println!(
                "  rerun:   cargo run -p entropyfa-engine --bin data-pipeline -- review --run {}",
                run_id
            );
        }
        ("update_contract_then_rerun_pipeline", _, _) => {
            println!("\nNext:");
            println!("  inspect: review_md for the suggested contract changes");
            println!(
                "  rerun:   cargo run -p entropyfa-engine --bin data-pipeline -- run-agents --year 2026 --category <category> --key <key>"
            );
        }
        ("investigate_sources_manually", _, _) => {
            println!("\nNext:");
            println!("  inspect: review_md for the source conflicts");
        }
        _ => {}
    }
}

fn parse_agent_provider(
    value: &str,
) -> Result<data_pipeline::AgentProvider, Box<dyn std::error::Error>> {
    match value {
        "claude" => Ok(data_pipeline::AgentProvider::Claude),
        "codex" => Ok(data_pipeline::AgentProvider::Codex),
        _ => Err(format!("unsupported agent provider: {value}").into()),
    }
}

fn display_agent_provider(provider: data_pipeline::AgentProvider) -> &'static str {
    match provider {
        data_pipeline::AgentProvider::Claude => "claude",
        data_pipeline::AgentProvider::Codex => "codex",
    }
}

fn run_post_apply_checks(
    outcome: &data_pipeline::ApplyOutcome,
) -> Result<(), Box<dyn std::error::Error>> {
    let workspace_root = data_pipeline::workspace_root();
    let snapshot_output = format!("engine/data_registry/{}/snapshot.json", outcome.year);

    run_command(
        Command::new("cargo")
            .current_dir(&workspace_root)
            .arg("run")
            .arg("-p")
            .arg("entropyfa-engine")
            .arg("--bin")
            .arg("data-pipeline")
            .arg("--")
            .arg("snapshot")
            .arg("--output")
            .arg(snapshot_output),
        "regenerating snapshot",
    )?;
    run_command(
        Command::new("cargo")
            .current_dir(&workspace_root)
            .arg("run")
            .arg("-p")
            .arg("entropyfa-engine")
            .arg("--bin")
            .arg("data-pipeline")
            .arg("--")
            .arg("validate"),
        "validating reviewed data",
    )?;
    if outcome.category == "insurance" && outcome.key == "irmaa_brackets" {
        run_command(
            Command::new("cargo")
                .current_dir(&workspace_root)
                .arg("test")
                .arg("-p")
                .arg("entropyfa-engine")
                .arg("irmaa"),
            "running IRMAA tests",
        )?;
    }

    Ok(())
}

fn run_command(command: &mut Command, description: &str) -> Result<(), Box<dyn std::error::Error>> {
    let status = command.status()?;
    if !status.success() {
        return Err(format!("{description} failed with status {status}").into());
    }
    Ok(())
}

const TITLE_COLOR: Color = Color::Reset;
const LABEL_COLOR: Color = Color::Gray;
const DIM_COLOR: Color = Color::DarkGray;
const BORDER_COLOR: Color = Color::DarkGray;
const ACCENT_COLOR: Color = Color::Cyan;
const INFO_COLOR: Color = Color::Blue;
const SUCCESS_COLOR: Color = Color::Green;
const WARN_COLOR: Color = Color::Yellow;
const DANGER_COLOR: Color = Color::Red;

fn print_status_report_plain(report: &data_pipeline::PipelineStatusReport) {
    println!("Data pipeline status");
    println!("  year: {}", report.year);
    println!("  registry entries: {}", report.registry_entries);
    println!("  pipeline definitions: {}", report.pipeline_definitions);
    println!("  reviewed artifacts: {}", report.reviewed_artifacts);
    println!(
        "  verification: authoritative {} | corroborated {} | derived {} | placeholder {}",
        report.authoritative_entries,
        report.corroborated_entries,
        report.derived_entries,
        report.placeholder_entries
    );

    println!("\nPipeline-backed entries:");
    for entry in report.entries.iter().filter(|entry| entry.pipeline_defined) {
        println!("  - {}/{}", entry.category, entry.key);
        println!(
            "    registry: {}/{}",
            entry.verification_status, entry.completeness
        );
        println!(
            "    reviewed_artifact: {}",
            yes_no(entry.reviewed_artifact_exists)
        );
        if let Some(run) = &entry.latest_run {
            println!(
                "    latest_run: {} | approved {} | {}{}",
                run.status,
                display_approved(run.approved),
                run.run_id,
                run.created_at
                    .as_ref()
                    .map(|created_at| format!(" | {created_at}"))
                    .unwrap_or_default()
            );
        } else {
            println!("    latest_run: none");
        }
        if let Some(notes) = entry
            .notes
            .as_ref()
            .filter(|notes| !notes.trim().is_empty())
        {
            println!("    note: {}", truncate_text(notes.trim(), 140));
        }
    }

    println!("\nRegistry entries without a pipeline definition:");
    let grouped_unpipelined = grouped_unpipelined_entries(report);
    if grouped_unpipelined.is_empty() {
        println!("  - none");
    } else {
        for (category, entries) in grouped_unpipelined {
            println!("  {category} ({})", entries.len());
            for entry in entries {
                let mut line = format!(
                    "    - {}: {}/{}",
                    entry.key, entry.verification_status, entry.completeness
                );
                if let Some(notes) = entry
                    .notes
                    .as_ref()
                    .filter(|notes| !notes.trim().is_empty())
                {
                    line.push_str(&format!(" | {}", truncate_text(notes.trim(), 100)));
                }
                println!("{line}");
            }
        }
    }
}

fn render_status_dashboard(
    report: &data_pipeline::PipelineStatusReport,
) -> Result<(), Box<dyn std::error::Error>> {
    let width = crossterm::terminal::size()
        .map(|(width, _)| width.clamp(100, 140))
        .unwrap_or(120);

    let pipeline_lines = build_pipeline_lines(report);
    let unpipelined_lines = build_unpipelined_lines(report);
    let attention_lines = build_attention_lines(report);
    let mix_lines = build_mix_lines(report);
    let coverage_lines = build_coverage_lines(report);

    let pipeline_height = (pipeline_lines.len() as u16 + 2).max(8);
    let unpipelined_height = (unpipelined_lines.len() as u16 + 2).max(12);
    let mix_height = (mix_lines.len() as u16 + 2).max(8);
    let coverage_height = (coverage_lines.len() as u16 + 2).max(8);
    let attention_height = (attention_lines.len() as u16 + 2).max(8);

    let left_total = pipeline_height + unpipelined_height;
    let right_total = mix_height + coverage_height + attention_height;
    let content_height = left_total.max(right_total);
    let total_height = 1 + 5 + content_height + 1;

    let area = Rect::new(0, 0, width, total_height);
    let mut buffer = Buffer::empty(area);

    let layout = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(5),
        Constraint::Fill(1),
        Constraint::Length(1),
    ])
    .split(area);

    render_header(&mut buffer, layout[0], report);
    render_summary_cards(&mut buffer, layout[1], report);
    render_content(
        &mut buffer,
        layout[2],
        report,
        &pipeline_lines,
        pipeline_height,
        &unpipelined_lines,
        &mix_lines,
        mix_height,
        &coverage_lines,
        coverage_height,
        &attention_lines,
    );
    render_footer(&mut buffer, layout[3]);

    let mut stdout = io::stdout();
    let mut backend = CrosstermBackend::new(&mut stdout);
    write_buffer(&mut backend, &buffer)?;
    Ok(())
}

fn render_header(buffer: &mut Buffer, area: Rect, report: &data_pipeline::PipelineStatusReport) {
    let line = Line::from(vec![
        Span::styled(
            format!("Data Pipeline Status {}", report.year),
            Style::default()
                .fg(TITLE_COLOR)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled("  |  ", Style::default().fg(DIM_COLOR)),
        Span::styled(
            format!("{} registry entries", report.registry_entries),
            Style::default().fg(LABEL_COLOR),
        ),
        Span::styled("  |  ", Style::default().fg(DIM_COLOR)),
        Span::styled(
            format!("{} pipeline definitions", report.pipeline_definitions),
            Style::default().fg(LABEL_COLOR),
        ),
    ]);
    Paragraph::new(vec![line]).render(area, buffer);
}

fn render_summary_cards(
    buffer: &mut Buffer,
    area: Rect,
    report: &data_pipeline::PipelineStatusReport,
) {
    let pipeline_total = report
        .entries
        .iter()
        .filter(|entry| entry.pipeline_defined)
        .count();
    let summary = Layout::horizontal([
        Constraint::Ratio(1, 4),
        Constraint::Ratio(1, 4),
        Constraint::Ratio(1, 4),
        Constraint::Ratio(1, 4),
    ])
    .split(area);

    render_stat_card(
        buffer,
        summary[0],
        "Registry",
        report.registry_entries.to_string(),
        "tracked entries",
        ACCENT_COLOR,
    );
    render_stat_card(
        buffer,
        summary[1],
        "Pipeline Coverage",
        format!("{pipeline_total}/{}", report.registry_entries),
        "definitions present",
        INFO_COLOR,
    );
    render_stat_card(
        buffer,
        summary[2],
        "Reviewed",
        format!("{}/{}", report.reviewed_artifacts, pipeline_total),
        "artifacts written",
        SUCCESS_COLOR,
    );
    render_stat_card(
        buffer,
        summary[3],
        "Verification",
        format!("{} auth", report.authoritative_entries),
        &format!(
            "{} placeholder | {} derived",
            report.placeholder_entries, report.derived_entries
        ),
        if report.placeholder_entries > 0 {
            DANGER_COLOR
        } else if report.derived_entries > 0 {
            WARN_COLOR
        } else {
            SUCCESS_COLOR
        },
    );
}

fn render_stat_card(
    buffer: &mut Buffer,
    area: Rect,
    title: &str,
    value: String,
    subtitle: &str,
    color: Color,
) {
    let lines = vec![
        Line::from(Span::styled(
            title.to_string(),
            Style::default().fg(LABEL_COLOR),
        )),
        Line::from(Span::styled(
            value,
            Style::default().fg(color).add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            subtitle.to_string(),
            Style::default().fg(DIM_COLOR),
        )),
    ];

    Paragraph::new(lines)
        .block(Block::bordered().border_style(Style::default().fg(BORDER_COLOR)))
        .render(area, buffer);
}

#[allow(clippy::too_many_arguments)]
fn render_content(
    buffer: &mut Buffer,
    area: Rect,
    report: &data_pipeline::PipelineStatusReport,
    pipeline_lines: &[Line<'static>],
    pipeline_height: u16,
    unpipelined_lines: &[Line<'static>],
    mix_lines: &[Line<'static>],
    mix_height: u16,
    coverage_lines: &[Line<'static>],
    coverage_height: u16,
    attention_lines: &[Line<'static>],
) {
    let columns = Layout::horizontal([Constraint::Percentage(58), Constraint::Percentage(42)])
        .spacing(1)
        .split(area);
    let left = Layout::vertical([Constraint::Length(pipeline_height), Constraint::Fill(1)])
        .spacing(1)
        .split(columns[0]);
    let right = Layout::vertical([
        Constraint::Length(mix_height),
        Constraint::Length(coverage_height),
        Constraint::Fill(1),
    ])
    .spacing(1)
    .split(columns[1]);

    render_panel(buffer, left[0], "Pipeline-Backed Entries", pipeline_lines);
    render_panel(
        buffer,
        left[1],
        &format!(
            "Registry Entries Without Pipeline Definition ({})",
            report
                .entries
                .iter()
                .filter(|entry| !entry.pipeline_defined)
                .count()
        ),
        unpipelined_lines,
    );
    render_panel(buffer, right[0], "Verification Mix", mix_lines);
    render_panel(buffer, right[1], "Pipeline Progress", coverage_lines);
    render_panel(buffer, right[2], "Attention Queue", attention_lines);
}

fn render_panel(buffer: &mut Buffer, area: Rect, title: &str, lines: &[Line<'static>]) {
    Paragraph::new(lines.to_vec())
        .block(
            Block::bordered()
                .title(Span::styled(
                    title.to_string(),
                    Style::default().fg(LABEL_COLOR),
                ))
                .border_style(Style::default().fg(BORDER_COLOR)),
        )
        .wrap(Wrap { trim: true })
        .render(area, buffer);
}

fn render_footer(buffer: &mut Buffer, area: Rect) {
    let line = Line::from(vec![
        Span::styled("Hint: ", Style::default().fg(LABEL_COLOR)),
        Span::styled(
            "use `data-pipeline status --plain` for plain text output",
            Style::default().fg(DIM_COLOR),
        ),
    ]);
    Paragraph::new(vec![line]).render(area, buffer);
}

fn build_pipeline_lines(report: &data_pipeline::PipelineStatusReport) -> Vec<Line<'static>> {
    let mut lines = Vec::new();

    for entry in report.entries.iter().filter(|entry| entry.pipeline_defined) {
        lines.push(Line::from(Span::styled(
            format!("{}/{}", entry.category, entry.key),
            Style::default()
                .fg(TITLE_COLOR)
                .add_modifier(Modifier::BOLD),
        )));

        let mut detail_spans = vec![
            Span::styled(
                format!("{}/{}", entry.verification_status, entry.completeness),
                Style::default().fg(color_for_status(entry.verification_status)),
            ),
            Span::styled("  |  ", Style::default().fg(DIM_COLOR)),
            Span::styled(
                format!("artifact {}", yes_no(entry.reviewed_artifact_exists)),
                Style::default().fg(LABEL_COLOR),
            ),
            Span::styled("  |  ", Style::default().fg(DIM_COLOR)),
        ];

        if let Some(run) = &entry.latest_run {
            detail_spans.push(Span::styled(
                format!(
                    "{} / approved {}",
                    run.status,
                    display_approved(run.approved)
                ),
                Style::default().fg(LABEL_COLOR),
            ));
            detail_spans.push(Span::styled("  |  ", Style::default().fg(DIM_COLOR)));
            detail_spans.push(Span::styled(
                run.created_at
                    .as_deref()
                    .map(short_timestamp)
                    .unwrap_or_else(|| "time unknown".to_string()),
                Style::default().fg(DIM_COLOR),
            ));
        } else {
            detail_spans.push(Span::styled(
                "no run recorded yet",
                Style::default().fg(DIM_COLOR),
            ));
        }

        lines.push(Line::from(detail_spans));
    }

    if lines.is_empty() {
        lines.push(Line::from(Span::styled(
            "No pipeline definitions found.",
            Style::default().fg(DIM_COLOR),
        )));
    }

    lines
}

fn build_unpipelined_lines(report: &data_pipeline::PipelineStatusReport) -> Vec<Line<'static>> {
    let mut lines = Vec::new();

    for (category, entries) in grouped_unpipelined_entries(report) {
        lines.push(Line::from(Span::styled(
            format!("{category} ({})", entries.len()),
            Style::default()
                .fg(LABEL_COLOR)
                .add_modifier(Modifier::BOLD),
        )));

        for entry in entries {
            lines.push(Line::from(vec![
                Span::styled("  ", Style::default().fg(DIM_COLOR)),
                Span::styled(entry.key.clone(), Style::default().fg(TITLE_COLOR)),
                Span::styled("  ", Style::default().fg(DIM_COLOR)),
                Span::styled(
                    format!("{}/{}", entry.verification_status, entry.completeness),
                    Style::default().fg(color_for_status(entry.verification_status)),
                ),
            ]));
        }
    }

    if lines.is_empty() {
        lines.push(Line::from(Span::styled(
            "All registry entries have pipeline definitions.",
            Style::default().fg(SUCCESS_COLOR),
        )));
    }

    lines
}

fn grouped_unpipelined_entries(
    report: &data_pipeline::PipelineStatusReport,
) -> Vec<(&str, Vec<&data_pipeline::PipelineStatusEntry>)> {
    let mut grouped = BTreeMap::<&str, Vec<&data_pipeline::PipelineStatusEntry>>::new();

    for entry in report
        .entries
        .iter()
        .filter(|entry| !entry.pipeline_defined)
    {
        grouped
            .entry(entry.category.as_str())
            .or_default()
            .push(entry);
    }

    grouped.into_iter().collect()
}

fn build_attention_lines(report: &data_pipeline::PipelineStatusReport) -> Vec<Line<'static>> {
    let mut items = Vec::new();

    for entry in report.entries.iter().filter(|entry| {
        matches!(
            entry.verification_status,
            data_pipeline::VerificationStatus::Placeholder
                | data_pipeline::VerificationStatus::Derived
        )
    }) {
        let summary = entry
            .notes
            .as_deref()
            .map(|notes| truncate_text(notes.trim(), 70))
            .unwrap_or_else(|| "needs review".to_string());
        items.push(Line::from(vec![
            Span::styled(
                format!("{}/{}", entry.category, entry.key),
                Style::default().fg(color_for_status(entry.verification_status)),
            ),
            Span::styled("  ", Style::default().fg(DIM_COLOR)),
            Span::styled(summary, Style::default().fg(DIM_COLOR)),
        ]));
    }

    let uncovered = report
        .entries
        .iter()
        .filter(|entry| !entry.pipeline_defined)
        .count();
    if uncovered > 0 {
        items.push(Line::from(vec![
            Span::styled("pipeline gap", Style::default().fg(WARN_COLOR)),
            Span::styled("  ", Style::default().fg(DIM_COLOR)),
            Span::styled(
                format!("{uncovered} registry entries still have no pipeline definition"),
                Style::default().fg(DIM_COLOR),
            ),
        ]));
    }

    for entry in report
        .entries
        .iter()
        .filter(|entry| entry.pipeline_defined && entry.latest_run.is_none())
    {
        items.push(Line::from(vec![
            Span::styled(
                format!("{}/{}", entry.category, entry.key),
                Style::default().fg(INFO_COLOR),
            ),
            Span::styled("  ", Style::default().fg(DIM_COLOR)),
            Span::styled(
                "pipeline exists but no run has been recorded yet",
                Style::default().fg(DIM_COLOR),
            ),
        ]));
    }

    if items.is_empty() {
        items.push(Line::from(Span::styled(
            "No outstanding pipeline issues.",
            Style::default().fg(SUCCESS_COLOR),
        )));
    }

    items
}

fn build_mix_lines(report: &data_pipeline::PipelineStatusReport) -> Vec<Line<'static>> {
    vec![
        bar_line(
            "authoritative",
            report.authoritative_entries,
            report.registry_entries,
            SUCCESS_COLOR,
        ),
        bar_line(
            "corroborated",
            report.corroborated_entries,
            report.registry_entries,
            INFO_COLOR,
        ),
        bar_line(
            "derived",
            report.derived_entries,
            report.registry_entries,
            WARN_COLOR,
        ),
        bar_line(
            "placeholder",
            report.placeholder_entries,
            report.registry_entries,
            DANGER_COLOR,
        ),
    ]
}

fn build_coverage_lines(report: &data_pipeline::PipelineStatusReport) -> Vec<Line<'static>> {
    let pipeline_total = report
        .entries
        .iter()
        .filter(|entry| entry.pipeline_defined)
        .count();
    let latest_run_count = report
        .entries
        .iter()
        .filter(|entry| entry.pipeline_defined && entry.latest_run.is_some())
        .count();
    let approved_count = report
        .entries
        .iter()
        .filter(|entry| {
            entry.pipeline_defined
                && entry
                    .latest_run
                    .as_ref()
                    .and_then(|run| run.approved)
                    .unwrap_or(false)
        })
        .count();
    let applied_count = report
        .entries
        .iter()
        .filter(|entry| {
            entry.pipeline_defined
                && entry
                    .latest_run
                    .as_ref()
                    .map(|run| run.status.to_string() == "applied")
                    .unwrap_or(false)
        })
        .count();

    vec![
        bar_line(
            "defined",
            pipeline_total,
            report.registry_entries,
            ACCENT_COLOR,
        ),
        bar_line(
            "latest run",
            latest_run_count,
            pipeline_total.max(1),
            INFO_COLOR,
        ),
        bar_line(
            "approved",
            approved_count,
            pipeline_total.max(1),
            SUCCESS_COLOR,
        ),
        bar_line(
            "applied",
            applied_count,
            pipeline_total.max(1),
            SUCCESS_COLOR,
        ),
    ]
}

fn bar_line(label: &str, count: usize, total: usize, color: Color) -> Line<'static> {
    let width = 14;
    let filled = if total == 0 || count == 0 {
        0
    } else {
        (count * width).div_ceil(total)
    }
    .min(width);
    let bar = format!("{}{}", "█".repeat(filled), "░".repeat(width - filled));

    Line::from(vec![
        Span::styled(format!("{label:<13}"), Style::default().fg(LABEL_COLOR)),
        Span::styled(bar, Style::default().fg(color)),
        Span::styled(
            format!(" {:>2}/{}", count, total),
            Style::default().fg(TITLE_COLOR),
        ),
    ])
}

fn short_timestamp(value: &str) -> String {
    value
        .replace('T', " ")
        .trim_end_matches('Z')
        .chars()
        .take(16)
        .collect::<String>()
}

fn color_for_status(status: data_pipeline::VerificationStatus) -> Color {
    match status {
        data_pipeline::VerificationStatus::Authoritative => SUCCESS_COLOR,
        data_pipeline::VerificationStatus::Corroborated => INFO_COLOR,
        data_pipeline::VerificationStatus::Derived => WARN_COLOR,
        data_pipeline::VerificationStatus::Placeholder => DANGER_COLOR,
    }
}

fn write_buffer<W: Write>(backend: &mut CrosstermBackend<W>, buffer: &Buffer) -> io::Result<()> {
    let width = buffer.area.width as usize;
    for row in buffer.content.chunks(width) {
        let mut current_fg = Color::Reset;
        let mut current_bg = Color::Reset;
        let mut current_modifier = Modifier::empty();

        for cell in row {
            if cell.modifier != current_modifier {
                queue!(backend, SetAttribute(CAttribute::Reset))?;
                current_modifier = cell.modifier;
                if cell.modifier.contains(Modifier::BOLD) {
                    queue!(backend, SetAttribute(CAttribute::Bold))?;
                }
                if cell.modifier.contains(Modifier::ITALIC) {
                    queue!(backend, SetAttribute(CAttribute::Italic))?;
                }
                if cell.modifier.contains(Modifier::UNDERLINED) {
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

fn yes_no(value: bool) -> &'static str {
    if value {
        "yes"
    } else {
        "no"
    }
}

fn display_approved(value: Option<bool>) -> &'static str {
    match value {
        Some(true) => "yes",
        Some(false) => "no",
        None => "n/a",
    }
}

fn truncate_text(text: &str, max_chars: usize) -> String {
    let truncated = text.chars().take(max_chars).collect::<String>();
    if text.chars().count() > max_chars {
        format!("{truncated}...")
    } else {
        truncated
    }
}
