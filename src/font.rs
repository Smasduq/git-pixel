pub const GLYPH_WIDTH: usize = 5;
pub const GLYPH_HEIGHT: usize = 7;

type Glyph = [[bool; 5]; 7];

use std::collections::HashMap;
use std::sync::OnceLock;

fn font_table() -> &'static HashMap<char, Glyph> {
    static TABLE: OnceLock<HashMap<char, Glyph>> = OnceLock::new();
    TABLE.get_or_init(|| {
        let mut m = HashMap::new();
        for (c, g) in FONT {
            let c = c.chars().next().unwrap();
            m.insert(c, g);
        }
        m
    })
}

pub fn glyph(c: char) -> Option<&'static Glyph> {
    let upper = c.to_ascii_uppercase();
    font_table().get(&upper)
}

pub fn text_to_matrix(text: &str, spacing: usize) -> Vec<Vec<bool>> {
    let mut all: Vec<Glyph> = Vec::new();
    for c in text.chars() {
        match glyph(c) {
            Some(g) => all.push(*g),
            None => {
                let mut blank = [[false; 5]; 7];
                for row in blank.iter_mut() {
                    *row = [false; 5];
                }
                all.push(blank);
            }
        }
    }

    if all.is_empty() {
        return vec![vec![false; 0]; GLYPH_HEIGHT];
    }

    let total_cols = all.len() * GLYPH_WIDTH + spacing * (all.len() - 1);
    let mut matrix = vec![vec![false; total_cols]; GLYPH_HEIGHT];

    let mut col = 0;
    for (i, g) in all.iter().enumerate() {
        for r in 0..GLYPH_HEIGHT {
            for c in 0..GLYPH_WIDTH {
                matrix[r][col + c] = g[r][c];
            }
        }
        col += GLYPH_WIDTH;
        if i + 1 < all.len() {
            col += spacing;
        }
    }

    matrix
}

pub fn matrix_to_intensity(matrix: &[Vec<bool>]) -> Vec<Vec<u8>> {
    matrix
        .iter()
        .map(|row| row.iter().map(|&b| if b { 4 } else { 0 }).collect())
        .collect()
}

const FONT: [(&str, Glyph); 37] = [
    ("A", [
        [false, true, true, true, false],
        [true, false, false, false, true],
        [true, false, false, false, true],
        [true, true, true, true, true],
        [true, false, false, false, true],
        [true, false, false, false, true],
        [true, false, false, false, true],
    ]),
    ("B", [
        [true, true, true, true, false],
        [true, false, false, false, true],
        [true, false, false, false, true],
        [true, true, true, true, false],
        [true, false, false, false, true],
        [true, false, false, false, true],
        [true, true, true, true, false],
    ]),
    ("C", [
        [false, true, true, true, true],
        [true, false, false, false, false],
        [true, false, false, false, false],
        [true, false, false, false, false],
        [true, false, false, false, false],
        [true, false, false, false, false],
        [false, true, true, true, true],
    ]),
    ("D", [
        [true, true, true, true, false],
        [true, false, false, false, true],
        [true, false, false, false, true],
        [true, false, false, false, true],
        [true, false, false, false, true],
        [true, false, false, false, true],
        [true, true, true, true, false],
    ]),
    ("E", [
        [true, true, true, true, true],
        [true, false, false, false, false],
        [true, false, false, false, false],
        [true, true, true, true, false],
        [true, false, false, false, false],
        [true, false, false, false, false],
        [true, true, true, true, true],
    ]),
    ("F", [
        [true, true, true, true, true],
        [true, false, false, false, false],
        [true, false, false, false, false],
        [true, true, true, true, false],
        [true, false, false, false, false],
        [true, false, false, false, false],
        [true, false, false, false, false],
    ]),
    ("G", [
        [false, true, true, true, true],
        [true, false, false, false, false],
        [true, false, false, false, false],
        [true, false, true, true, true],
        [true, false, false, false, true],
        [true, false, false, false, true],
        [false, true, true, true, true],
    ]),
    ("H", [
        [true, false, false, false, true],
        [true, false, false, false, true],
        [true, false, false, false, true],
        [true, true, true, true, true],
        [true, false, false, false, true],
        [true, false, false, false, true],
        [true, false, false, false, true],
    ]),
    ("I", [
        [false, true, true, true, false],
        [false, false, true, false, false],
        [false, false, true, false, false],
        [false, false, true, false, false],
        [false, false, true, false, false],
        [false, false, true, false, false],
        [false, true, true, true, false],
    ]),
    ("J", [
        [false, false, false, true, true],
        [false, false, false, false, true],
        [false, false, false, false, true],
        [false, false, false, false, true],
        [false, false, false, false, true],
        [true, false, false, false, true],
        [false, true, true, true, false],
    ]),
    ("K", [
        [true, false, false, false, true],
        [true, false, false, true, false],
        [true, false, true, false, false],
        [true, true, false, false, false],
        [true, false, true, false, false],
        [true, false, false, true, false],
        [true, false, false, false, true],
    ]),
    ("L", [
        [true, false, false, false, false],
        [true, false, false, false, false],
        [true, false, false, false, false],
        [true, false, false, false, false],
        [true, false, false, false, false],
        [true, false, false, false, false],
        [true, true, true, true, true],
    ]),
    ("M", [
        [true, false, false, false, true],
        [true, true, false, true, true],
        [true, false, true, false, true],
        [true, false, true, false, true],
        [true, false, false, false, true],
        [true, false, false, false, true],
        [true, false, false, false, true],
    ]),
    ("N", [
        [true, false, false, false, true],
        [true, true, false, false, true],
        [true, false, true, false, true],
        [true, false, false, true, true],
        [true, false, false, false, true],
        [true, false, false, false, true],
        [true, false, false, false, true],
    ]),
    ("O", [
        [false, true, true, true, false],
        [true, false, false, false, true],
        [true, false, false, false, true],
        [true, false, false, false, true],
        [true, false, false, false, true],
        [true, false, false, false, true],
        [false, true, true, true, false],
    ]),
    ("P", [
        [true, true, true, true, false],
        [true, false, false, false, true],
        [true, false, false, false, true],
        [true, true, true, true, false],
        [true, false, false, false, false],
        [true, false, false, false, false],
        [true, false, false, false, false],
    ]),
    ("Q", [
        [false, true, true, true, false],
        [true, false, false, false, true],
        [true, false, false, false, true],
        [true, false, false, false, true],
        [true, false, true, false, true],
        [true, false, false, true, false],
        [false, true, true, false, true],
    ]),
    ("R", [
        [true, true, true, true, false],
        [true, false, false, false, true],
        [true, false, false, false, true],
        [true, true, true, true, false],
        [true, false, true, false, false],
        [true, false, false, true, false],
        [true, false, false, false, true],
    ]),
    ("S", [
        [false, true, true, true, true],
        [true, false, false, false, false],
        [true, false, false, false, false],
        [false, true, true, true, false],
        [false, false, false, false, true],
        [false, false, false, false, true],
        [true, true, true, true, false],
    ]),
    ("T", [
        [true, true, true, true, true],
        [false, false, true, false, false],
        [false, false, true, false, false],
        [false, false, true, false, false],
        [false, false, true, false, false],
        [false, false, true, false, false],
        [false, false, true, false, false],
    ]),
    ("U", [
        [true, false, false, false, true],
        [true, false, false, false, true],
        [true, false, false, false, true],
        [true, false, false, false, true],
        [true, false, false, false, true],
        [true, false, false, false, true],
        [false, true, true, true, false],
    ]),
    ("V", [
        [true, false, false, false, true],
        [true, false, false, false, true],
        [true, false, false, false, true],
        [true, false, false, false, true],
        [true, false, false, false, true],
        [false, true, false, true, false],
        [false, false, true, false, false],
    ]),
    ("W", [
        [true, false, false, false, true],
        [true, false, false, false, true],
        [true, false, false, false, true],
        [true, false, true, false, true],
        [true, false, true, false, true],
        [true, true, false, true, true],
        [true, false, false, false, true],
    ]),
    ("X", [
        [true, false, false, false, true],
        [true, false, false, false, true],
        [false, true, false, true, false],
        [false, false, true, false, false],
        [false, true, false, true, false],
        [true, false, false, false, true],
        [true, false, false, false, true],
    ]),
    ("Y", [
        [true, false, false, false, true],
        [true, false, false, false, true],
        [false, true, false, true, false],
        [false, false, true, false, false],
        [false, false, true, false, false],
        [false, false, true, false, false],
        [false, false, true, false, false],
    ]),
    ("Z", [
        [true, true, true, true, true],
        [false, false, false, false, true],
        [false, false, false, true, false],
        [false, false, true, false, false],
        [false, true, false, false, false],
        [true, false, false, false, false],
        [true, true, true, true, true],
    ]),
    ("0", [
        [false, true, true, true, false],
        [true, false, false, false, true],
        [true, false, false, true, true],
        [true, false, true, false, true],
        [true, true, false, false, true],
        [true, false, false, false, true],
        [false, true, true, true, false],
    ]),
    ("1", [
        [false, false, true, false, false],
        [false, true, true, false, false],
        [false, false, true, false, false],
        [false, false, true, false, false],
        [false, false, true, false, false],
        [false, false, true, false, false],
        [false, true, true, true, false],
    ]),
    ("2", [
        [false, true, true, true, false],
        [true, false, false, false, true],
        [false, false, false, false, true],
        [false, false, false, true, false],
        [false, false, true, false, false],
        [false, true, false, false, false],
        [true, true, true, true, true],
    ]),
    ("3", [
        [true, true, true, true, false],
        [false, false, false, false, true],
        [false, false, false, false, true],
        [false, true, true, true, false],
        [false, false, false, false, true],
        [false, false, false, false, true],
        [true, true, true, true, false],
    ]),
    ("4", [
        [false, false, false, true, false],
        [false, false, true, true, false],
        [false, true, false, true, false],
        [true, false, false, true, false],
        [true, true, true, true, true],
        [false, false, false, true, false],
        [false, false, false, true, false],
    ]),
    ("5", [
        [true, true, true, true, true],
        [true, false, false, false, false],
        [true, true, true, true, false],
        [false, false, false, false, true],
        [false, false, false, false, true],
        [true, false, false, false, true],
        [false, true, true, true, false],
    ]),
    ("6", [
        [false, true, true, true, false],
        [true, false, false, false, false],
        [true, false, false, false, false],
        [true, true, true, true, false],
        [true, false, false, false, true],
        [true, false, false, false, true],
        [false, true, true, true, false],
    ]),
    ("7", [
        [true, true, true, true, true],
        [false, false, false, false, true],
        [false, false, false, false, true],
        [false, false, false, true, false],
        [false, false, true, false, false],
        [false, false, true, false, false],
        [false, false, true, false, false],
    ]),
    ("8", [
        [false, true, true, true, false],
        [true, false, false, false, true],
        [true, false, false, false, true],
        [false, true, true, true, false],
        [true, false, false, false, true],
        [true, false, false, false, true],
        [false, true, true, true, false],
    ]),
    ("9", [
        [false, true, true, true, false],
        [true, false, false, false, true],
        [true, false, false, false, true],
        [false, true, true, true, true],
        [false, false, false, false, true],
        [false, false, false, false, true],
        [false, true, true, true, false],
    ]),
    (" ", [
        [false, false, false, false, false],
        [false, false, false, false, false],
        [false, false, false, false, false],
        [false, false, false, false, false],
        [false, false, false, false, false],
        [false, false, false, false, false],
        [false, false, false, false, false],
    ]),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn glyph_a_matches_expected_pattern() {
        let expected: Glyph = [
            [false, true, true, true, false],
            [true, false, false, false, true],
            [true, false, false, false, true],
            [true, true, true, true, true],
            [true, false, false, false, true],
            [true, false, false, false, true],
            [true, false, false, false, true],
        ];
        assert_eq!(glyph('A'), Some(&expected));
        assert_eq!(glyph('a'), Some(&expected));
    }

    #[test]
    fn text_to_matrix_hi_has_expected_dims() {
        let matrix = text_to_matrix("HI", 1);
        assert_eq!(matrix.len(), 7);
        assert_eq!(matrix[0].len(), 11);
    }

    #[test]
    fn unsupported_char_does_not_panic_and_gives_blank_gap() {
        let matrix = text_to_matrix("H@I", 1);
        assert_eq!(matrix.len(), 7);
        assert_eq!(matrix[0].len(), 17);
        let broad = text_to_matrix("@", 1);
        assert_eq!(broad.len(), 7);
        assert_eq!(broad[0].len(), 5);
        for row in &broad {
            assert!(row.iter().all(|&b| !b));
        }
    }

    #[test]
    fn matrix_to_intensity_maps_tf() {
        let matrix = vec![
            vec![true, false],
            vec![false, true],
        ];
        let expected = vec![vec![4u8, 0u8], vec![0u8, 4u8]];
        assert_eq!(matrix_to_intensity(&matrix), expected);
    }

    #[test]
    fn glyph_case_insensitive_and_none_for_unsupported() {
        assert_eq!(glyph('Z'), glyph('z'));
        assert!(glyph('~').is_none());
        assert!(glyph('?').is_none());
    }

    #[test]
    fn text_to_matrix_all_7_rows_same_width() {
        let matrix = text_to_matrix("HELLO", 1);
        let width = matrix[0].len();
        for row in &matrix {
            assert_eq!(row.len(), width);
        }
    }
}
