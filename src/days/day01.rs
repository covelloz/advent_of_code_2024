use std::env;
use csv::{Reader, ReaderBuilder};
use std::error::Error;
use std::fs::File;
use std::path::{Path, PathBuf};

pub fn read_input() -> Result<(Vec<i32>, Vec<i32>), Box<dyn Error>> {
    let current_dir: PathBuf = env::current_dir().unwrap();
    let root: &Path = Path::new(current_dir.to_str().unwrap());
    let path: PathBuf = root.join("src/resources/day01/day01.tsv");

    let mut reader: Reader<File> = ReaderBuilder::new()
        .delimiter(b'\t')
        .has_headers(false)
        .from_path(path)?;

    let mut column_1: Vec<i32> = Vec::new();
    let mut column_2: Vec<i32> = Vec::new();

    for result in reader.records() {
        let record = result?;

        if let Some(field_1) = record.get(0) {
            if let Ok(num) = field_1.parse::<i32>() {
                column_1.push(num)
            }
        }

        if let Some(field_2) = record.get(1) {
            if let Ok(num) = field_2.parse::<i32>() {
                column_2.push(num);
            }
        }
    }

    Ok((column_1, column_2))
}

pub fn solve() -> Result<(), Box<dyn Error>> {
    println!("\nDAY 01");
    println!("------");
    let (mut list_1, mut list_2) = match read_input() {
        Ok((list_1, list_2)) => (list_1, list_2),
        Err(e) => {
            println!("Error reading CSV input:\n  {}", e);
            return Err(e.into());
        }
    };

    // === Part 1 ===
    // sort input
    list_1.sort();
    list_2.sort();

    // calculate distances
    let len = list_1.len().min(list_2.len());
    let mut list_diff: Vec<i32> = Vec::new();

    for i in 0..len {
        let diff = (list_1[i] - list_2[i]).abs();
        list_diff.push(diff);
    }

    // sum of distances
    let sum_diff: i32 = list_diff.iter().sum();
    println!("Part 1 answer: {}", sum_diff);

    // === Part 2 ===
    // calculate similarity score
    let mut list_sim: Vec<i32> = Vec::new();

    for i in 0..len {
        let mut sim_count = 0;

        for j in 0..len {
            if list_1[i] == list_2[j] {
                sim_count += 1;
            }
        }

        list_sim.push(list_1[i]*sim_count)
    }

    // sum of similarity scores
    let sum_sim: i32 = list_sim.iter().sum();
    println!("Part 2 answer: {}", sum_sim);

    Ok(())
}
