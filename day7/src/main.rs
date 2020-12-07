use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

struct Bag {
    contains: Vec<(String, usize)>,
    is_gold: bool,
}

impl Bag {
    fn new(color: &str) -> Bag {
        Bag {
            contains: Vec::new(),
            is_gold: color == "shiny gold",
        }
    }
}

fn main() {
    let content = read_file();
    let parse_start = std::time::Instant::now();
    let bags = parse(&content);
    let parse_duration = parse_start.elapsed().as_nanos();
    println!("Parse duration: {}ns", parse_duration);

    // Part 1
    {
        let now = std::time::Instant::now();
        let mut count = 0;
        for bag in bags.values() {
            if part1_check(bag, &bags) {
                count += 1;
            }
        }

        let duration = now.elapsed().as_nanos();
        println!("Part1 count: {}. Time taken: {}ns", count, duration);
    }

    // Part 2
    {
        let now = std::time::Instant::now();
        let count = part2_check(bags.get("shiny gold").unwrap(), &bags);
        let duration = now.elapsed().as_nanos();
        println!("Part2 count: {}. Time taken: {}ns", count, duration);
    }
}

fn parse(content: &String) -> HashMap<String, Bag> {
    let line_regex = Regex::new(r"^(\w+? \w+?) bags contain ([\w\s,]+?).$").unwrap();
    let children_regex = Regex::new(r"(?:, )?(\d+?) (\w+? \w+)").unwrap();
    let mut bags: HashMap<String, Bag> = HashMap::new();
    for line in content.lines() {
        let line_cap = match line_regex.captures(line) {
            None => continue,
            Some(e) => e,
        };

        let key = &line_cap[1];
        let bag = match bags.get_mut(key) {
            None => {
                bags.insert(String::from(key), Bag::new(key));
                bags.get_mut(key).unwrap()
            }
            Some(bag) => bag,
        };

        for child_cap in children_regex.captures_iter(&line_cap[2]) {
            bag.contains.push((
                String::from(&child_cap[2]),
                child_cap[1].parse::<usize>().unwrap(),
            ));
        }
    }

    bags
}

fn part1_check(bag: &Bag, bags: &HashMap<String, Bag>) -> bool {
    for (child_color, _count) in bag.contains.iter() {
        let child_bag = bags.get(child_color).unwrap();
        if child_bag.is_gold {
            return true;
        }

        if part1_check(child_bag, bags) {
            return true;
        }
    }

    false
}

fn part2_check(bag: &Bag, bags: &HashMap<String, Bag>) -> usize {
    let mut count = 0;
    for (child_color, child_count) in bag.contains.iter() {
        count += child_count;
        let child_bag = bags.get(child_color).unwrap();
        let nested_counts = part2_check(child_bag, bags);
        count += nested_counts * child_count;
    }

    count
}

fn read_file() -> String {
    let mut s = String::new();
    File::open("input.txt")
        .unwrap()
        .read_to_string(&mut s)
        .unwrap();
    s
}
