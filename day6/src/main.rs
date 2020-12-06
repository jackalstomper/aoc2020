use std::fs::File;
use std::io::prelude::*;

fn main() {
    let content = read_file();

    // Part 1
    {
        let now = std::time::Instant::now();
        let mut count = 0;
        for line in content.split("\n\n") {
            let mut counts = [0usize; 26];
            for b in line.bytes() {
                if b == '\n' as u8 {
                    continue;
                }

                counts[(b - 97) as usize] += 1;
            }

            for c in counts.iter() {
                if *c > 0 {
                    count += 1;
                }
            }
        }

        let duration = now.elapsed().as_nanos();
        println!("Part1 result is {}. Time taken: {}ns", count, duration);
    }

    // Part 2
    {
        let now = std::time::Instant::now();
        let mut count = 0;
        for line in content.split("\n\n") {
            let mut group_size = 1;
            let mut counts = [0usize; 26];
            for b in line.bytes() {
                if b == '\n' as u8 {
                    group_size += 1;
                    continue;
                }

                counts[(b - 97) as usize] += 1;
            }

            for c in counts.iter() {
                if *c == group_size {
                    count += 1;
                }
            }
        }

        let duration = now.elapsed().as_nanos();
        println!("Part2 result is {}. Time taken: {}ns", count, duration);
    }
}

fn read_file() -> String {
    let mut s = String::new();
    File::open("input.txt")
        .unwrap()
        .read_to_string(&mut s)
        .unwrap();
    s
}
