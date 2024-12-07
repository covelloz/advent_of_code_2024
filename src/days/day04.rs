use csv::{Reader, ReaderBuilder, StringRecord};
use std::env;
use std::error::Error;
use std::fs::File;
use std::path::{Path, PathBuf};

pub fn solve() {
    println!("\nDAY 04");
    println!("------");

    let (rows, cols): (Vec<Vec<char>>, Vec<Vec<char>>) = match read_input() {
        Ok(txt) => txt,
        Err(e) => {
            eprintln!("Error reading input:\n  {}", e);
            return;
        }
    };
    let word_grid = CharGrid { rows, cols };

    let mut count: usize = 0;
    let word_match: String = String::from("XMAS");

    count = orthogonal_search(&word_grid, &word_match, 4, CardinalDirection::East, count);
    count = orthogonal_search(&word_grid, &word_match, 4, CardinalDirection::West, count);
    count = orthogonal_search(&word_grid, &word_match, 4, CardinalDirection::North, count);
    count = orthogonal_search(&word_grid, &word_match, 4, CardinalDirection::South, count);

    println!("Part1::Answer: {}", count)
}

fn orthogonal_search(
    grid: &CharGrid,
    word_match: &str,
    window_size: usize,
    direction: CardinalDirection,
    mut count: usize,
) -> usize {
    match direction {
        CardinalDirection::East => {
            // forwards horizontal search
            let text_to_scan: &Vec<Vec<char>> = &grid.rows;

            for row in text_to_scan {
                for i in 0..row.len() {
                    let scan_window: String = row.iter().skip(i).take(window_size).collect();

                    if scan_window.contains(word_match) {
                        count += 1;
                    }
                }
            }
        }
        CardinalDirection::West => {
            // backwards horizontal search
            let text_to_scan: &Vec<Vec<char>> = &grid.rows;

            for row in text_to_scan {
                for i in 0..row.len() {
                    let scan_window: String = row.iter().rev().skip(i).take(window_size).collect();

                    if scan_window.contains(word_match) {
                        count += 1;
                    }
                }
            }
        }
        CardinalDirection::North => {
            let _text_to_scan: &Vec<Vec<char>> = &grid.cols;
        }
        CardinalDirection::South => {
            let _text_to_scan: &Vec<Vec<char>> = &grid.cols;
        }
    }

    count
}

/// Creates a "CharGrid" of characters using vectors.
/// **Assumption**: all rows have the same number of characters.
/// i.e:
///     XMASAMX
///     XMASAMX
/// parses as:
/// rows: [['X', 'M', 'A', 'S', 'A', 'M', 'X'], ['X', 'M', 'A', 'S', 'A', 'M', 'X']]
/// cols: [['X', 'X'], ['M', 'M'], ['A', 'A'], ['S', 'S'], ['A', 'A'], ['M', 'M'], ['X', 'X']]
fn read_input() -> Result<(Vec<Vec<char>>, Vec<Vec<char>>), Box<dyn Error>> {
    let current_dir: PathBuf = env::current_dir().unwrap();
    let root: &Path = Path::new(current_dir.to_str().unwrap());
    let path: PathBuf = root.join("src/resources/day04/day04_ex.tsv");

    let mut reader: Reader<File> = ReaderBuilder::new()
        .delimiter(b'\n')
        .has_headers(false)
        .from_path(path)?;

    let mut rows: Vec<Vec<char>> = Vec::new();
    let mut cols: Vec<Vec<char>> = Vec::new();

    for result in reader.records() {
        let record: StringRecord = result?;
        // deconstruct into chars
        rows = record.iter().map(|field| field.chars().collect()).collect();
    }

    let max_cols: usize = rows[0].len(); // note: ref assumption in docstring
    for i in 0..max_cols {
        let mut col: Vec<char> = vec![' '; rows.len()];

        for j in 0..rows.len() {
            col[j] = rows[j][i];
        }

        cols.push(col);
    }

    Ok((rows, cols))
}

#[derive(Debug)]
struct CharGrid {
    rows: Vec<Vec<char>>,
    cols: Vec<Vec<char>>,
}

#[derive(Debug)]
enum CardinalDirection {
    North,
    East,
    South,
    West,
}

// #[derive(Debug)]
// enum InterCardinalDirection {
//     NorthEast,
//     SouthEast,
//     SouthWest,
//     NorthWest
// }
