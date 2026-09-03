use chrono::{Datelike, NaiveDate, Weekday};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell {
    pub week: usize,
    pub day: usize,
    pub date: NaiveDate,
    pub in_target_year: bool,
}

impl Cell {
    pub fn weekday(&self) -> Weekday {
        self.date.weekday()
    }
}

#[derive(Debug, Clone)]
pub struct YearGrid {
    cells: Vec<Cell>,
    week_count: usize,
}

impl YearGrid {
    pub fn build(year: i32) -> YearGrid {
        let jan_1 = NaiveDate::from_ymd_opt(year, 1, 1).expect("valid date");
        let dec_31 = NaiveDate::from_ymd_opt(year, 12, 31).expect("valid date");

        let first_day = weekday_index(jan_1.weekday());
        let last_day = weekday_index(dec_31.weekday());

        let start = jan_1 - chrono::Duration::days(first_day as i64);
        let end = dec_31 + chrono::Duration::days((6 - last_day) as i64);

        let total_days = ((end - start).num_days() + 1) as usize;
        let week_count = total_days / 7;

        let mut cells = Vec::with_capacity(total_days);
        let mut current = start;
        for week in 0..week_count {
            for day in 0..7 {
                let in_target_year = current.year() == year;
                cells.push(Cell {
                    week,
                    day,
                    date: current,
                    in_target_year,
                });
                current += chrono::Duration::days(1);
            }
        }

        YearGrid { cells, week_count }
    }

    pub fn week_count(&self) -> usize {
        self.week_count
    }

    pub fn date_at(&self, week: usize, day: usize) -> Option<NaiveDate> {
        if week >= self.week_count || day > 6 {
            return None;
        }
        self.cells.get(week * 7 + day).map(|c| c.date)
    }

    pub fn drawable_cells(&self) -> impl Iterator<Item = &Cell> {
        self.cells.iter().filter(|c| c.in_target_year)
    }
}

fn weekday_index(weekday: Weekday) -> usize {
    match weekday {
        Weekday::Sun => 0,
        Weekday::Mon => 1,
        Weekday::Tue => 2,
        Weekday::Wed => 3,
        Weekday::Thu => 4,
        Weekday::Fri => 5,
        Weekday::Sat => 6,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_cell_is_sunday_last_cell_is_saturday() {
        for year in 2020..2030 {
            let grid = YearGrid::build(year);
            let first = &grid.cells[0];
            let last = &grid.cells[grid.cells.len() - 1];
            assert_eq!(first.day, 0, "first cell day for {year}");
            assert_eq!(first.weekday(), Weekday::Sun, "first weekday for {year}");
            assert_eq!(last.day, 6, "last cell day for {year}");
            assert_eq!(last.weekday(), Weekday::Sat, "last weekday for {year}");
        }
    }

    #[test]
    fn jan_1_and_dec_31_present_and_in_target_year() {
        for year in 2020..2030 {
            let grid = YearGrid::build(year);
            let jan_1 = NaiveDate::from_ymd_opt(year, 1, 1).unwrap();
            let dec_31 = NaiveDate::from_ymd_opt(year, 12, 31).unwrap();
            assert!(
                grid.cells.iter().any(|c| c.date == jan_1 && c.in_target_year),
                "Jan 1 missing for {year}"
            );
            assert!(
                grid.cells.iter().any(|c| c.date == dec_31 && c.in_target_year),
                "Dec 31 missing for {year}"
            );
        }
    }

    #[test]
    fn week_count_between_52_and_54() {
        for year in 2020..2030 {
            let grid = YearGrid::build(year);
            assert!(
                (52..=54).contains(&grid.week_count),
                "week count {} out of range for {year}",
                grid.week_count
            );
        }
    }

    #[test]
    fn date_at_matches_cell_data() {
        let grid = YearGrid::build(2026);
        for week in 0..grid.week_count {
            for day in 0..7 {
                let cell = &grid.cells[week * 7 + day];
                assert_eq!(grid.date_at(week, day), Some(cell.date));
                assert_eq!(grid.date_at(week, day).map(|d| d.weekday()), Some(cell.date.weekday()));
            }
        }
    }

    #[test]
    fn first_cell_weekday_always_sunday() {
        for year in 2020..2030 {
            let grid = YearGrid::build(year);
            let first = grid.date_at(0, 0).unwrap();
            assert_eq!(first.weekday(), Weekday::Sun, "year {year}");
        }
    }

    #[test]
    fn drawable_cells_only_in_target_year() {
        let grid = YearGrid::build(2026);
        for cell in grid.drawable_cells() {
            assert!(cell.in_target_year, "cell {:?} should be drawable", cell.date);
            assert_eq!(cell.date.year(), 2026, "cell {:?} should be in 2026", cell.date);
        }
    }

    #[test]
    fn drawable_cells_cover_full_year() {
        let grid = YearGrid::build(2026);
        let count = grid.drawable_cells().count();
        assert!((365..=366).contains(&count), "unexpected drawable count {count}");
    }

    #[test]
    fn date_at_out_of_bounds_returns_none() {
        let grid = YearGrid::build(2026);
        assert!(grid.date_at(grid.week_count, 0).is_none());
        assert!(grid.date_at(0, 7).is_none());
    }
}
