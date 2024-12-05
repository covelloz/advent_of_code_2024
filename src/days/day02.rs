use csv::{Reader, ReaderBuilder, StringRecord};
use std::env;
use std::error::Error;
use std::fs::File;
use std::path::{Path, PathBuf};

pub fn solve() -> Result<(), Box<dyn Error>> {
    println!("\nDAY 02");
    println!("------");

    let reports: Vec<Vec<i32>> = match read_input() {
        Ok(rows) => rows,
        Err(e) => {
            eprintln!("Error reading TSV input:\n  {}", e);
            return Err(e.into());
        }
    };

    let mut safe_count: i32 = 0;
    let mut tolerant_count: i32 = 0;

    for i in 0..reports.len() {
        let levels: &Vec<i32> = &reports[i];
        let monotone: bool = monotone_check(levels);
        let bounded: bool = bounded_check(levels);

        // === Part 1 ===
        if monotone && bounded {
            safe_count += 1;
        } else {
            // === Part 2 ===
            // TODO: this algorithm seems extremely brute-force-ish; try to optimize? O(NxM^3)
            // TODO: early break helps optimize it a bit at least
            let mut tolerant_without_level: Vec<bool> = Vec::new();

            for j in 0..levels.len() {
                // quarantine each level; re-check safety without it
                let quarantine: Vec<i32> = levels.iter()
                    .enumerate()
                    .filter_map(|(idx, &num)| {
                        if j == idx { None } else { Some(num) }
                    })
                    .collect();

                let monotone_tolerant: bool = monotone_check(&quarantine);
                let bounded_tolerant: bool = bounded_check(&quarantine);
                tolerant_without_level.push(monotone_tolerant && bounded_tolerant);

                if monotone_tolerant && bounded_tolerant {
                    break;
                }
            }

            // tolerance: safe with at least one level removed
            if tolerant_without_level.iter().any(|&tol| tol) {
                tolerant_count += 1;
            }
        }
    }

    println!("Part1::Answer: {}", safe_count);
    println!("Part2::Answer: {}", safe_count + tolerant_count);

    Ok(())
}

fn monotone_check(levels: &Vec<i32>) -> bool {
    let mut check: bool = false;
    let mut gradients: Vec<Gradient> = Vec::new();

    for i in 1..levels.len() {
        if levels[i] < levels[i - 1] {
            gradients.push(Gradient::DOWN)
        }
        if levels[i] > levels[i - 1] {
            gradients.push(Gradient::UP)
        }
    }

    // all adjacent pairs are monotone
    if gradients.iter().all(|dir| matches!(dir, Gradient::UP)) ||
        gradients.iter().all(|dir| matches!(dir, Gradient::DOWN)) {
        check = true;
    }

   check
}

fn bounded_check(levels: &Vec<i32>) -> bool {
    let mut check: bool = false;
    let mut distances: Vec<i32> = Vec::new();

    for i in 1..levels.len() {
        let dist = (levels[i] - levels[i-1]).abs();
        distances.push(dist);
    }

    // all adjacent pairs are bounded
    if distances.iter().all(|&dist| dist >= 1 && dist <= 3) {
        check = true;
    }

    check
}

fn read_input() -> Result<Vec<Vec<i32>>, Box<dyn Error>> {
    let current_dir: PathBuf = env::current_dir().unwrap();
    let root: &Path = Path::new(current_dir.to_str().unwrap());
    let path: PathBuf = root.join("src/resources/day02/day02.tsv");

    let mut reader: Reader<File> = ReaderBuilder::new()
        .delimiter(b'\t')
        .has_headers(false)
        .flexible(true)
        .from_path(path)?;

    let mut rows: Vec<Vec<i32>> = Vec::new();

    for result in reader.records() {
        let record: StringRecord = result?;

        let row: Vec<i32> = record.iter()
            .filter_map(|field| field.trim().parse::<i32>().ok())
            .collect();

        if !row.is_empty() {
            rows.push(row)
        }
    }

    Ok(rows)
}

#[derive(Debug)]
enum Gradient {
    UP,
    DOWN,
}
