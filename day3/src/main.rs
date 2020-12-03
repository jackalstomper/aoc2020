use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let forest: Vec<Vec<bool>> = {
        let file = File::open("input.txt").unwrap();
        io::BufReader::new(file)
            .lines()
            .map(|line| line.unwrap())
            .filter(|line| !line.is_empty())
            .map(|line| line.chars().map(|c| c == '#').collect())
            .collect()
    };

    //part 1
    println!("Part1 hit count: {}", get_hits(&forest, 3, 1));

    // part 2
    println!(
        "Part2 hit count: {}",
        get_hits(&forest, 1, 1)
            * get_hits(&forest, 3, 1)
            * get_hits(&forest, 5, 1)
            * get_hits(&forest, 7, 1)
            * get_hits(&forest, 1, 2)
    );
}

fn get_hits(forest: &Vec<Vec<bool>>, right_step: usize, down_step: usize) -> usize {
    let mut hit_count = 0;
    let mut x = 0usize;
    let mut y = 0usize;
    loop {
        if y >= forest.len() {
            // complete
            break;
        }

        if x >= forest[y].len() {
            // wrap around
            x = x - forest[y].len();
        }

        if forest[y][x] {
            hit_count += 1;
        }

        y += down_step;
        x += right_step;
    }

    hit_count
}
