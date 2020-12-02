use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};

struct CapInfo {
    min: u32,
    max: u32,
    ch: char,
    pass: String,
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let line_regex =
        Regex::new(r"^(?P<min>\d+)-(?P<max>\d+)\s(?P<char>\w):\s(?P<pass>\w+)$").unwrap();

    let line_info: Vec<CapInfo> = io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .filter(|line| !line.is_empty())
        .map(|line| {
            let cap = line_regex.captures_iter(&line).next().unwrap();
            CapInfo {
                min: cap["min"].parse().unwrap(),
                max: cap["max"].parse().unwrap(),
                ch: cap["char"].chars().next().unwrap(),
                pass: String::from(&cap["pass"]),
            }
        })
        .collect();

    // part 1
    let mut valid_count = 0u32;
    line_info.iter().for_each(|line| {
        let count = line
            .pass
            .chars()
            .fold(0u32, |sum, c| if c == line.ch { sum + 1 } else { sum });
        if count >= line.min && count <= line.max {
            valid_count += 1;
        }
    });

    println!("Part 1 valid count is {}", valid_count);

    // part 2
    valid_count = 0;
    line_info.iter().for_each(|line| {
        let mut count = 0u32;
        if line.pass.chars().nth(line.min as usize - 1).unwrap() == line.ch {
            count += 1;
        }
        if line.pass.chars().nth(line.max as usize - 1).unwrap() == line.ch {
            count += 1;
        }
        if count == 1 {
            valid_count += 1;
        }
    });

    println!("Part 2 valid count is {}", valid_count);
}
