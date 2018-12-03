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

    let mut comparing: usize = 0;
    let mut current_line: Option<&String> = None;

    let mut found = false;

    for _ in 0..lines.len() {
        for line in lines.iter().skip(comparing) {
            match current_line {
                None => {
                    current_line = Some(line);
                }
                Some(comparing_line) => match compare_strings(&line, &comparing_line) {
                    None => {}
                    Some(same_letters) => {
                        println!("{}", same_letters);
                        found = true;
                        break;
                    }
                },
            }
        }
        current_line = None;
        comparing += 1;

        if found {
            break;
        }
    }

    Ok(())
}

fn compare_strings(s1: &str, s2: &str) -> Option<String> {
    let mut chars_different = 0;
    let mut same_letters = String::new();
    for i in 0..s1.len() {
        if s1.chars().nth(i).unwrap() != s2.chars().nth(i).unwrap() {
            chars_different += 1;
        } else {
            same_letters.push(s1.chars().nth(i).unwrap());
        }
    }

    if chars_different == 1 {
        return Some(same_letters);
    } else {
        return None;
    }
}
