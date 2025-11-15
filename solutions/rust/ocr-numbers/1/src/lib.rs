extern crate core;

use crate::Error::{InvalidColumnCount, InvalidRowCount};

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

const GLYPH_HEIGHT: usize = 4;
const GLYPH_WIDTH: usize = 3;
const UNKNOWN_CHAR: char = '?';

#[rustfmt::skip]
const DIGIT_GLYPHS: [[&str; GLYPH_HEIGHT]; 10] = [
    [" _ ",
     "| |",
     "|_|",
     "   "],
    ["   ",
     "  |",
     "  |",
     "   "],
    [" _ ",
     " _|",
     "|_ ",
     "   "],
    [" _ ",
     " _|",
     " _|",
     "   "],
    ["   ",
     "|_|",
     "  |",
     "   "],
    [" _ ",
     "|_ ",
     " _|",
     "   "],
    [" _ ",
     "|_ ",
     "|_|",
     "   "],
    [" _ ",
     "  |",
     "  |",
     "   "],
    [" _ ",
     "|_|",
     "|_|",
     "   "],
    [" _ ",
     "|_|",
     " _|",
     "   "]
];

pub struct Glyph {
    key: char,
    raw: Vec<String>,
    value: String,
}

pub struct GlyphSet {
    glyphs: Vec<Glyph>,
}

impl GlyphSet {
    pub fn new(glyphs: &[(char, &[&str])]) -> Self {
        GlyphSet {
            glyphs: glyphs
                .iter()
                .map(|g| Glyph {
                    key: g.0,
                    raw: g.1.iter().map(|row| row.to_string()).collect(),
                    value: g.1.join("\n"),
                })
                .collect(),
        }
    }

    pub fn recognize(&self, glyph_value: &str) -> Option<char> {
        for glyph in self.glyphs.iter() {
            if glyph.value == glyph_value {
                return Some(glyph.key);
            }
        }
        None
    }

    pub fn recognize_raw(&self, glyph_raw: &[&str]) -> Option<char> {
        for glyph in self.glyphs.iter() {
            if glyph.raw == glyph_raw {
                return Some(glyph.key);
            }
        }
        None
    }
}

fn create_digit_glyphset() -> GlyphSet {
    let glyphs: Vec<(char, &[&str])> = DIGIT_GLYPHS
        .iter()
        .enumerate()
        .map(|(i, rows)| (char::from_digit(i as u32, 10).unwrap(), rows.as_slice()))
        .collect();

    GlyphSet::new(&glyphs)
}

pub fn convert(input: &str) -> Result<String, Error> {
    let digit_glyphset = create_digit_glyphset();

    let rows: Vec<&str> = input.split('\n').collect();
    let rows_count = rows.len();

    if rows_count % GLYPH_HEIGHT > 0 || rows_count == 0 {
        return Err(InvalidRowCount(rows.len()));
    }

    let row_len = rows[0].len();
    let glyphs_in_row_count = row_len / GLYPH_WIDTH;
    let glyphs_rows_count = rows_count / GLYPH_HEIGHT;

    for row in &rows {
        if row.len() % GLYPH_WIDTH > 0 || row.len() != row_len {
            return Err(InvalidColumnCount(row.len()));
        }
    }

    let mut result: Vec<char> = Vec::new();

    for ridx in 0..glyphs_rows_count {
        for gidx in 0..glyphs_in_row_count {
            let grow = &rows[ridx * GLYPH_HEIGHT..(ridx + 1) * GLYPH_HEIGHT];
            let glyph_raw = grow
                .iter()
                .map(|r| &r[gidx * GLYPH_WIDTH..(gidx + 1) * GLYPH_WIDTH])
                .collect::<Vec<&str>>();
            if let Some(ch) = digit_glyphset.recognize_raw(&glyph_raw) {
                result.push(ch);
            } else {
                result.push(UNKNOWN_CHAR);
            }
        }
        if ridx < glyphs_rows_count - 1 {
            result.push(',');
        }
    }

    if result.is_empty() {
        result.push(UNKNOWN_CHAR);
    }

    Ok(String::from_iter(result))
}
