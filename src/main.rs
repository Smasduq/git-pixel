use chrono::Datelike;
use clap::Parser;

use gitpixel::calendar::YearGrid;
use gitpixel::font::{matrix_to_intensity, text_to_matrix};
use gitpixel::layout::place_on_grid;
use gitpixel::preview::render_terminal;

#[derive(Parser)]
#[command(name = "gitpixel", about = "Draw text on a GitHub contribution graph")]
struct Cli {
    /// The text to render, e.g. "SADIQU"
    #[arg(long)]
    text: String,

    /// Which calendar year's grid to draw on (default: current year)
    #[arg(long)]
    year: Option<i32>,

    /// Which week column to start placing the text at
    #[arg(long, default_value_t = 10)]
    start_week: usize,
}

fn main() {
    let cli = Cli::parse();

    let year = cli.year.unwrap_or_else(|| {
        chrono::Local::now().date_naive().year()
    });

    let grid = YearGrid::build(year);

    let bool_matrix = text_to_matrix(&cli.text, 1);
    let matrix_width = bool_matrix.first().map(|r| r.len()).unwrap_or(0);

    let last_week = grid.week_count();
    if cli.start_week + matrix_width > last_week {
        eprintln!(
            "error: \"{}\" (width {}) doesn't fit in year {year} starting at week {} \
             (grid has {} weeks). Try a shorter word or an earlier start week.",
            cli.text, matrix_width, cli.start_week, last_week
        );
        std::process::exit(1);
    }

    let intensity = matrix_to_intensity(&bool_matrix);
    let result = place_on_grid(&grid, &intensity, cli.start_week);

    println!(
        "GitPixel preview — year {year}, text \"{}\" at week {}",
        cli.text, cli.start_week
    );
    println!("  placements: {}", result.placements.len());
    if !result.warnings.is_empty() {
        eprintln!("  warnings: {}", result.warnings.len());
    }
    println!();

    render_terminal(&grid, &result.placements);
}
