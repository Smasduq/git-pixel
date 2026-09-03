use chrono::NaiveDate;

use crate::calendar::YearGrid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Placement {
    pub date: NaiveDate,
    pub intensity: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SkipWarning {
    OutsideGrid { week: usize, row: usize },
    PaddingDay { week: usize, row: usize, date: NaiveDate },
}

#[derive(Debug, Clone, Default)]
pub struct LayoutResult {
    pub placements: Vec<Placement>,
    pub warnings: Vec<SkipWarning>,
}

pub fn place_on_grid(
    grid: &YearGrid,
    intensity_matrix: &[Vec<u8>],
    start_week: usize,
) -> LayoutResult {
    let mut result = LayoutResult::default();

    for (row, cols) in intensity_matrix.iter().enumerate() {
        for (col, &intensity) in cols.iter().enumerate() {
            if intensity == 0 {
                continue;
            }
            let week = start_week + col;
            let day = row;

            let date = match grid.date_at(week, day) {
                Some(d) => d,
                None => {
                    result
                        .warnings
                        .push(SkipWarning::OutsideGrid { week, row });
                    continue;
                }
            };

            if !grid.cell_in_target_year(week, day) {
                result
                    .warnings
                    .push(SkipWarning::PaddingDay { week, row, date });
                continue;
            }

            result.placements.push(Placement { date, intensity });
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::calendar::YearGrid;

    #[test]
    fn simple_matrix_places_correct_dates() {
        let grid = YearGrid::build(2026);
        let matrix = vec![
            vec![4u8, 0],
            vec![0, 3],
            vec![2, 0],
            vec![0, 1],
            vec![4, 0],
            vec![0, 3],
            vec![0, 0],
        ];
        let result = place_on_grid(&grid, &matrix, 0);

        // Cross-check every placement against grid.date_at; keep only
        // in-target-year cells with intensity > 0.
        let mut expected: Vec<(usize, usize, u8)> = Vec::new();
        for (row, cols) in matrix.iter().enumerate() {
            for (col, &intensity) in cols.iter().enumerate() {
                if intensity > 0
                    && grid.date_at(col, row).is_some()
                    && grid.cell_in_target_year(col, row)
                {
                    expected.push((col, row, intensity));
                }
            }
        }

        assert_eq!(result.placements.len(), expected.len());
        for (placement, (week, day, intensity)) in
            result.placements.iter().zip(expected.iter())
        {
            assert_eq!(placement.date, grid.date_at(*week, *day).unwrap());
            assert_eq!(placement.intensity, *intensity);
        }

        // The (0,0) cell in this matrix falls on a January padding day, so
        // it must be skipped via a warning, not placed.
        assert!(
            result
                .warnings
                .iter()
                .any(|w| matches!(w, SkipWarning::PaddingDay { week: 0, row: 0, .. }))
        );
    }

    #[test]
    fn matrix_longer_than_grid_produces_warnings_not_panic() {
        let grid = YearGrid::build(2026);
        let width = grid.week_count() + 5;
        let matrix = vec![vec![4u8; width]; 7];
        let result = place_on_grid(&grid, &matrix, 0);
        assert!(!result.warnings.is_empty());
        assert!(result.warnings.iter().any(|w| matches!(
            w,
            SkipWarning::OutsideGrid { .. }
        )));
        // Exactly one placement per in-target-year cell.
        let expected = grid.drawable_cells().count();
        assert_eq!(result.placements.len(), expected);
    }

    #[test]
    fn all_zero_matrix_produces_zero_placements() {
        let grid = YearGrid::build(2026);
        let matrix = vec![vec![0u8; 5]; 7];
        let result = place_on_grid(&grid, &matrix, 0);
        assert!(result.placements.is_empty());
        assert!(result.warnings.is_empty());
    }
}
