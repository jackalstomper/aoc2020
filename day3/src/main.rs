#![feature(test)]

use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufRead};

extern crate test;
use test::Bencher;

fn main() {
    do_stuff(read_file().as_bytes());
}

fn read_file() -> String {
    let mut s = String::new();
    File::open("input.txt")
        .unwrap()
        .read_to_string(&mut s)
        .unwrap();
    s
}

fn do_stuff(file_content: &[u8]) {
    //part 1
    println!("Part1 hit count: {}", get_hits(file_content, 3, 1));

    // part 2
    println!(
        "Part2 hit count: {}",
        get_hits(file_content, 1, 1)
            * get_hits(file_content, 3, 1)
            * get_hits(file_content, 5, 1)
            * get_hits(file_content, 7, 1)
            * get_hits(file_content, 1, 2)
    );
}

fn get_hits(forest: &[u8], right_step: usize, down_step: usize) -> usize {
    let width = 31;
    let mut hit_count = 0;
    let mut x = 0usize;
    let mut y = 0usize;
    loop {
        if y + x >= forest.len() {
            // complete
            break;
        }

        if x >= width {
            x = x - width;
        }

        if forest[y + x] == 35 {
            hit_count += 1;
        }

        x += right_step;
        y += down_step * (width + 1); // + 1 to skip newline character
    }

    hit_count
}

#[bench]
fn bench(b: &mut Bencher) {
    let content = read_file();
    b.iter(|| {
        do_stuff(content.as_bytes());
    });
}
