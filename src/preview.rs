use colored::Colorize;

use crate::calendar::YearGrid;
use crate::layout::Placement;

fn level_color(level: u8) -> (u8, u8, u8) {
    match level {
        0 => (235, 237, 240),
        1 => (155, 233, 168),
        2 => (64, 196, 99),
        3 => (48, 161, 78),
        4 => (33, 110, 57),
        _ => (255, 255, 255),
    }
}

fn block(r: u8, g: u8, b: u8) -> String {
    "  ".on_truecolor(r, g, b).to_string()
}

pub fn render_terminal(grid: &YearGrid, placements: &[Placement]) {
    use std::collections::HashMap;

    let mut intensity_by_date: HashMap<chrono::NaiveDate, u8> = HashMap::new();
    for p in placements {
        intensity_by_date.insert(p.date, p.intensity);
    }

    let week_count = grid.week_count();

    for day in 0..7 {
        let mut line = String::new();
        for week in 0..week_count {
            let date = match grid.date_at(week, day) {
                Some(d) => d,
                None => {
                    line.push_str(&block(level_color(0).0, level_color(0).1, level_color(0).2));
                    continue;
                }
            };
            let level = intensity_by_date.get(&date).copied().unwrap_or(0);
            let (r, g, b) = level_color(level);
            line.push_str(&block(r, g, b));
        }
        println!("{line}");
    }
}
