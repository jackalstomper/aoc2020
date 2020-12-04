mod passport;
mod validator;

use passport::Passport;
use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use validator::Validator;

use std::time::Instant;

fn main() {
    let content = read_file();
    let now = Instant::now();
    do_stuff(&content);
    println!("Time taken: {}ns", now.elapsed().as_nanos());
}

fn do_stuff(content: &String) {
    let validator = Validator::new();
    let pass_regex: Regex = Regex::new(r"(\w{3}):(#?\w+)").unwrap();
    let passports: Vec<Passport> = content
        .split("\n\n")
        .map(|pass| Passport::new(pass, &pass_regex))
        .collect();

    // Part1
    let mut valid_count = 0;
    for passport in passports.iter() {
        if validator.part1_validate(passport) {
            valid_count += 1;
        }
    }

    println!("Part1 valid count is {}", valid_count);

    // Part2
    valid_count = 0;
    for passport in passports.iter() {
        if validator.part2_validate(passport) {
            valid_count += 1;
        }
    }

    println!("Part2 valid count is {}", valid_count);
}

fn read_file() -> String {
    let mut s = String::new();
    File::open("input.txt")
        .unwrap()
        .read_to_string(&mut s)
        .unwrap();
    s
}
