use csv::{Reader, ReaderBuilder};
use std::env;
use std::error::Error;
use std::fs::File;
use std::path::{Path, PathBuf};

pub fn solve() -> Result<(), Box<dyn Error>> {
    println!("\nDAY 01");
    println!("------");
    let (mut list_1, mut list_2) = match read_input() {
        Ok((list_1, list_2)) => (list_1, list_2),
        Err(e) => {
            eprintln!("Error reading TSV input:\n  {}", e);
            return Err(e.into());
        }
    };

    // === Part 1 ===
    // sort input
    list_1.sort();
    list_2.sort();

    // calculate distances
    let len = list_1.len().min(list_2.len());
    let mut list_diff: Vec<isize> = Vec::new();

    for i in 0..len {
        let diff = (list_1[i] - list_2[i]).abs();
        list_diff.push(diff);
    }

    // sum of distances
    let sum_diff: isize = list_diff.iter().sum();
    println!("Part1::Answer: {}", sum_diff);

    // === Part 2 ===
    // calculate similarity score
    let mut list_sim: Vec<isize> = Vec::new();

    for i in 0..len {
        let mut sim_count: usize = 0;

        for j in 0..len {
            if list_1[i] == list_2[j] {
                sim_count += 1;
            }
        }

        list_sim.push(list_1[i] * (sim_count as isize))
    }

    // sum of similarity scores
    let sum_sim: isize = list_sim.iter().sum();
    println!("Part2::Answer: {}", sum_sim);

    Ok(())
}

fn read_input() -> Result<(Vec<isize>, Vec<isize>), Box<dyn Error>> {
    let current_dir: PathBuf = env::current_dir().unwrap();
    let root: &Path = Path::new(current_dir.to_str().unwrap());
    let path: PathBuf = root.join("src/resources/day01/day01.tsv");

    let mut reader: Reader<File> = ReaderBuilder::new()
        .delimiter(b'\t')
        .has_headers(false)
        .from_path(path)?;

    let mut column_1: Vec<isize> = Vec::new();
    let mut column_2: Vec<isize> = Vec::new();

    for result in reader.records() {
        let record = result?;

        record
            .iter()
            .filter_map(|field| field.trim().parse::<i32>().ok())
            .enumerate()
            .for_each(|(idx, num)| match idx {
                0 => column_1.push(num as isize),
                1 => column_2.push(num as isize),
                _ => {}
            })
    }

    Ok((column_1, column_2))
}
