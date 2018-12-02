use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> io::Result<()> {
    let args: Vec<_> = env::args().collect();
    let file = File::open(&args[1]).expect("File not found");
    let reader = BufReader::new(file);
    let mut lines: Vec<String> = Vec::new();
    let mut seen: HashSet<i32> = HashSet::new();
    let mut freq: i32 = 0;

    for line in reader.lines() {
        let unwrapped_line = line.unwrap();
        lines.push(unwrapped_line);
    }

    let mut found = false;
    while !found {
        for line in &lines {
            if line.starts_with("-") {
                freq -= parse_line(&line);
            } else if line.starts_with("+") {
                freq += parse_line(&line);
            }

            if seen.contains(&freq) {
                println!("{}", freq);
                found = true;
                break;
            } else {
                seen.insert(freq);
            }
        }
    }

    println!("{}", freq);
    Ok(())
}

fn parse_line(line: &str) -> i32 {
    let rest: String = line.chars().skip(1).take(line.len() - 1).collect();
    return rest.parse::<i32>().unwrap();
}
