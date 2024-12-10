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
    count = orthogonal_search(&word_grid, &word_match, 4, CardinalDirection::South, count);
    count = orthogonal_search(&word_grid, &word_match, 4, CardinalDirection::North, count);

    let diagonals = get_diagonals(&word_grid, Orientation::LeftRightDiagonal);

    println!("Part1::Answer: {}", count);
    println!("LeftRightDiagonals: {:?}", diagonals);
}

fn orthogonal_search(
    grid: &CharGrid,
    word_match: &str,
    window_size: u32,
    direction: CardinalDirection,
    mut count: usize,
) -> usize {
    match direction {
        CardinalDirection::East => {
            // forwards horizontal search
            count = forward_search(&grid.rows, window_size, word_match, count)
        }
        CardinalDirection::West => {
            // backwards horizontal search
            count = backward_search(&grid.rows, window_size, word_match, count)
        }
        CardinalDirection::South => {
            // forwards vertical search
            count = forward_search(&grid.cols, window_size, word_match, count)
        }
        CardinalDirection::North => {
            // backwards vertical search
            count = backward_search(&grid.cols, window_size, word_match, count)
        }
    }

    count
}

fn forward_search(
    texts_to_scan: &Vec<Vec<char>>,
    window_size: u32,
    word_match: &str,
    mut count: usize,
) -> usize {
    for text in texts_to_scan {
        for i in 0..text.len() {
            let scan_window: String = text.iter().skip(i).take(window_size as usize).collect();

            if scan_window.contains(word_match) {
                count += 1;
            }
        }
    }

    count
}

fn backward_search(
    texts_to_scan: &Vec<Vec<char>>,
    window_size: u32,
    word_match: &str,
    mut count: usize,
) -> usize {
    for text in texts_to_scan {
        for i in 0..text.len() {
            let scan_window: String = text
                .iter()
                .rev()
                .skip(i)
                .take(window_size as usize)
                .collect();

            if scan_window.contains(word_match) {
                count += 1;
            }
        }
    }

    count
}

fn get_diagonals(grid: &CharGrid, orient: Orientation) -> Vec<Vec<char>> {
    let num_of_rows: usize = grid.rows.len();
    let num_of_cols: usize = grid.cols.len();

    let mut diagonal: Vec<char> = Vec::new();
    let mut diagonals: Vec<Vec<char>> = Vec::new();

    match orient {
        Orientation::LeftRightDiagonal => {
            for i in 0..num_of_rows {
                diagonal = Vec::new();

                let mut walk_i: usize = i;
                let mut walk_j: usize = 0;

                while walk_i <= (num_of_rows - 1) && walk_j <= (num_of_cols - 1) {
                    diagonal.push(grid.rows[walk_i][walk_j]);
                    walk_i += 1;
                    walk_j += 1;
                }

                println!("row iter {}, diagonal {:?}", i, diagonal);
                diagonals.push(diagonal)
            }

            for j in 1..num_of_cols {
                // skip 0 because main-diagonal handled by row-loop
                diagonal = Vec::new();

                let mut walk_i: usize = 0;
                let mut walk_j: usize = j;

                while walk_i <= (num_of_rows - 1) && walk_j <= (num_of_cols - 1) {
                    diagonal.push(grid.rows[walk_i][walk_j]);
                    walk_i += 1;
                    walk_j += 1;
                }

                println!("col iter {}, diagonal {:?}", j, diagonal);
                diagonals.push(diagonal)
            }
        }
        Orientation::RightLeftDiagonal => {}
    }

    diagonals
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

#[derive(Debug)]
enum Orientation {
    LeftRightDiagonal,
    RightLeftDiagonal,
}
