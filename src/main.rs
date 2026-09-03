use std::path::PathBuf;

use chrono::Datelike;
use clap::{Parser, Subcommand};

use gitpixel::calendar::YearGrid;
use gitpixel::commit::generate_commits;
use gitpixel::font::{matrix_to_intensity, text_to_matrix};
use gitpixel::history::{HistoryEntry, HistoryLog};
use gitpixel::layout::{place_on_grid, Placement};
use gitpixel::preview::render_terminal;

#[derive(Parser)]
#[command(name = "gitpixel", about = "Draw text on a GitHub contribution graph")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Render text and (optionally) write backdated commits
    Draw(DrawArgs),
    /// Undo commits created by a previous run, cleanly
    Revert(RevertArgs),
    /// List recorded command history
    History,
}

#[derive(clap::Args)]
struct DrawArgs {
    /// The text to render, e.g. "SADIQU"
    #[arg(long)]
    text: String,

    /// Which calendar year's grid to draw on (default: current year)
    #[arg(long)]
    year: Option<i32>,

    /// Which week column to start placing the text at
    #[arg(long, default_value_t = 10)]
    start_week: usize,

    /// Path to the git repo to write commits into
    #[arg(long)]
    repo: PathBuf,

    /// Actually write the backdated commits (dry run otherwise)
    #[arg(long)]
    confirm: bool,
}

#[derive(clap::Args)]
struct RevertArgs {
    /// Revert the N-th recorded run (1 = oldest). Defaults to the most recent.
    #[arg(long)]
    id: Option<u64>,

    /// Revert the most recent recorded run (default)
    #[arg(long)]
    last: bool,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Draw(args) => run_draw(args),
        Command::Revert(args) => run_revert(args),
        Command::History => run_history(),
    }
}

fn build_pipeline(args: &DrawArgs) -> (YearGrid, Vec<Placement>, i32) {
    let year = args.year.unwrap_or_else(|| chrono::Local::now().date_naive().year());
    let grid = YearGrid::build(year);

    let bool_matrix = text_to_matrix(&args.text, 1);
    let matrix_width = bool_matrix.first().map(|r| r.len()).unwrap_or(0);

    let last_week = grid.week_count();
    if args.start_week + matrix_width > last_week {
        eprintln!(
            "error: \"{}\" (width {}) doesn't fit in year {year} starting at week {} \
             (grid has {} weeks). Try a shorter word or an earlier start week.",
            args.text, matrix_width, args.start_week, last_week
        );
        std::process::exit(1);
    }

    let intensity = matrix_to_intensity(&bool_matrix);
    let result = place_on_grid(&grid, &intensity, args.start_week);
    (grid, result.placements, year)
}

fn run_draw(args: DrawArgs) {
    let (grid, placements, year) = build_pipeline(&args);

    println!(
        "GitPixel preview — year {year}, text \"{}\" at week {}",
        args.text, args.start_week
    );
    println!("  placements: {}", placements.len());
    println!();

    render_terminal(&grid, &placements);

    if args.confirm {
        println!();
        match generate_commits(&args.repo, &placements) {
            Ok(outcome) => {
                println!(
                    "Created {} backdated commits in {}",
                    outcome.count,
                    args.repo.display()
                );
                record_history(&args, &outcome, year);
            }
            Err(e) => {
                eprintln!("error: failed to generate commits: {e}");
                std::process::exit(1);
            }
        }
    } else {
        println!();
        println!(
            "Dry run: {} commits would be written to {} (re-run with --confirm to write)",
            placements.len(),
            args.repo.display()
        );
    }
}

fn record_history(
    args: &DrawArgs,
    outcome: &gitpixel::commit::CommitOutcome,
    year: i32,
) {
    let after_head = match git2::Repository::open(&args.repo) {
        Ok(r) => match r.head().and_then(|h| h.peel_to_commit()) {
            Ok(c) => c.id().to_string(),
            Err(_) => String::new(),
        },
        Err(_) => String::new(),
    };

    let mut log = HistoryLog::open();
    let entry = HistoryEntry {
        id: log.entries().iter().map(|e| e.id).max().unwrap_or(0) + 1,
        when: chrono::Local::now().to_rfc3339(),
        repo: args.repo.display().to_string(),
        text: args.text.clone(),
        year,
        start_week: args.start_week,
        commits: outcome.count,
        before_head: outcome.before_head.clone(),
        after_head,
    };
    if let Err(e) = log.record(entry) {
        eprintln!("warning: could not record history: {e}");
    }
}

fn run_revert(args: RevertArgs) {
    let mut log = HistoryLog::open();

    let entry = if args.id.is_some() {
        let id = args.id.unwrap();
        log.entries().iter().find(|e| e.id == id).cloned()
    } else {
        log.entries().last().cloned()
    };

    let Some(entry) = entry else {
        eprintln!("error: no history to revert. Run `gitpixel draw --confirm` first.");
        std::process::exit(1);
    };

    let repo_path = PathBuf::from(&entry.repo);

    let repo = match git2::Repository::open(&repo_path) {
        Ok(r) => r,
        Err(e) => {
            eprintln!(
                "error: cannot open repo {} for {} to revert: {e}",
                repo_path.display(),
                entry.id
            );
            std::process::exit(1);
        }
    };

    match gitpixel::commit::head_is(&repo, &entry.after_head) {
        Ok(true) => {}
        Ok(false) => {
            eprintln!(
                "error: repo HEAD is not at the recorded state for run {} (expected {}). \
                 Refusing to revert — new commits are on top.",
                entry.id, entry.after_head
            );
            std::process::exit(1);
        }
        Err(e) => {
            eprintln!("error: {e}");
            std::process::exit(1);
        }
    }

    println!(
        "Reverting run #{} ({} commits, \"{}\" year {}) in {} → {}",
        entry.id,
        entry.commits,
        entry.text,
        entry.year,
        repo_path.display(),
        entry.before_head
    );

    if let Err(e) = gitpixel::commit::reset_to_oid(&repo_path, &entry.before_head) {
        eprintln!("error: revert failed: {e}");
        std::process::exit(1);
    }

    if let Some(_removed) = log.remove_id(entry.id) {
        println!("Reverted {} commits. History entry removed.", entry.commits);
    }
}

fn run_history() {
    let log = HistoryLog::open();
    let entries = log.entries();
    if entries.is_empty() {
        println!("No recorded runs yet.");
        return;
    }
    println!("{:<4} {:<24} {:<40} {:<8} {:>6} {:>6}", "ID", "WHEN", "REPO", "TEXT", "YEAR", "COMMITS");
    for e in entries {
        println!(
            "{:<4} {:<24} {:<40} {:<8} {:>6} {:>6}",
            e.id, e.when, e.repo, e.text, e.year, e.commits
        );
    }
    println!();
    println!("Revert the most recent run with: gitpixel revert");
    println!("Revert a specific run with:       gitpixel revert --id <ID>");
}
