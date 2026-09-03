use gitpixel::calendar::YearGrid;
use gitpixel::font::text_to_matrix;

fn main() {
    let year = 2026;
    let grid = YearGrid::build(year);

    let first = grid.date_at(0, 0).unwrap();
    let last_week = grid.week_count() - 1;
    let last = grid.date_at(last_week, 6).unwrap();

    println!("GitPixel calendar engine (year {year})");
    println!("  week count   : {}", grid.week_count());
    println!("  first cell   : {first} (Sunday)");
    println!("  last cell    : {last} (Saturday)");
    println!("  drawable days: {}", grid.drawable_cells().count());
    println!();

    println!("Bitmap font — \"HI\":");
    let matrix = text_to_matrix("HI", 1);
    for row in &matrix {
        let line: String = row.iter().map(|&on| if on { '#' } else { '.' }).collect();
        println!("  {line}");
    }
}
