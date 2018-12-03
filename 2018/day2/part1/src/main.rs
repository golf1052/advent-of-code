use std::collections::HashMap;
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
    for line in reader.lines() {
        let unwrapped_line = line.unwrap();
        lines.push(unwrapped_line);
    }

    let mut num_2: i32 = 0;
    let mut num_3: i32 = 0;

    for line in &lines {
        let mut seen_letters: HashMap<char, i32> = HashMap::new();
        for letter in line.chars() {
            if !seen_letters.contains_key(&letter) {
                seen_letters.insert(letter, 0);
            }
            *(seen_letters.get_mut(&letter).unwrap()) += 1;
        }

        let mut has_2 = false;
        let mut has_3 = false;

        for (_, count) in seen_letters.iter() {
            if *count == 2 {
                has_2 = true;
            } else if *count == 3 {
                has_3 = true;
            }
        }

        if has_2 {
            num_2 += 1;
        }
        if has_3 {
            num_3 += 1;
        }
    }

    println!("{}", num_2 * num_3);

    Ok(())
}
