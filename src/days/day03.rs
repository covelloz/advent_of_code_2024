use regex::Regex;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::{env, io};

pub fn solve() {
    println!("\nDAY 03");
    println!("------");

    let memory: String = match read_input() {
        Ok(content) => {
            content
        }
        Err(e) => {
            eprintln!("Error reading file:\n  {}", e);
            return;
        }
    };

    let pattern: &str = r"mul\((\d+),(\d+)\)";
    let re: Regex = Regex::new(pattern).unwrap();

    let num_pairs: Vec<(i32, i32)> = capture_pairs(&memory, &re);

    // === Part 1 ===
    let products: Vec<i32> = num_pairs.iter()
        .map(|&(first, second)| first * second)
        .collect();
    let total: i32 = products.iter().sum();

    println!("Part1::Answer: {}", total);

    // === Part 2 ===
    let init_keyword: &str = "don't()";
    let mut mem_parts_to_process: Vec<String> = Vec::new();

    recursive_search(&memory, init_keyword, &mut mem_parts_to_process);

    let num_pairs2: Vec<(i32, i32)> = mem_parts_to_process.iter()
        .flat_map(|mem_part| {
            capture_pairs(mem_part, &re)
        })
        .collect();

    let products2: Vec<i32> = num_pairs2.iter()
        .map(|&(first, second)| first * second)
        .collect();
    let total2: i32 = products2.iter().sum();

    println!("Part2::Answer: {}", total2);
}

fn recursive_search(
    text: &str,
    keyword: &str,
    accumulator: &mut Vec<String>,
) -> String {
    let position: Option<usize> = text.find(keyword);

    if let Some(pos) = position {
        let (p1 , p2): (&str, &str) = text.split_at(pos);
        let p2: &str = &p2[keyword.len()..]; // remove keyword

        if keyword == "don't()" {
            accumulator.push(p1.to_string())
        }

        let next_keyword: &str = if keyword == "don't()" { "do()" } else { "don't()" };

        return recursive_search(p2, next_keyword, accumulator);
    }

    // keyword not found
    if keyword == "don't()" {
        accumulator.push(text.to_string());
    }
    text.to_string()
}

fn capture_pairs(memory: &String, re: &Regex) -> Vec<(i32, i32)> {
    re.captures_iter(memory)
        .filter_map(|caps| {
            let first: i32 = caps.get(1)?.as_str().parse().ok()?;
            let second: i32 = caps.get(2)?.as_str().parse().ok()?;
            Some((first, second))
        })
        .collect()
}

fn read_input() -> Result<String, io::Error> {
    let current_dir: PathBuf = env::current_dir().unwrap();
    let root: &Path = Path::new(current_dir.to_str().unwrap());
    let path: PathBuf = root.join("src/resources/day03/day03.txt");

    let mut file = File::open(path)?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    Ok(contents)
}