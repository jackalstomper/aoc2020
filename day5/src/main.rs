use std::fs::File;
use std::io::prelude::*;

fn main() {
    let content = read_file();
    let mut ids: Vec<i32> = content
        .lines()
        .map(|line| {
            if line.len() != 10 {
                panic!("Invalid line: {}", line);
            }

            let row = parition(0, 128, &line[0..7]).unwrap();
            let column = parition(0, 8, &line[7..]).unwrap();
            row * 8 + column
        })
        .collect();

    // part 1
    ids.sort();
    println!("Part1 result: {}", ids[ids.len() - 1]);

    // part 2
    for (i, val) in ids.iter().enumerate() {
        if i + 1 >= ids.len() {
            println!("Couldn't find solution");
            break;
        }

        let next = ids[i + 1];
        if next - val > 1 {
            println!("Part2 result: {}", val + 1);
            break;
        }
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

fn parition(start_index: i32, range_length: i32, line: &str) -> Result<i32, String> {
    let mut start = start_index;
    let mut len = range_length;
    for ch in line.chars() {
        match ch {
            'F' | 'L' => len /= 2,
            'B' | 'R' => {
                start += len / 2;
                len /= 2;
            }
            _ => return Err(format!("Invalid partition specifier: {}", ch)),
        };
    }

    Ok(start)
}
