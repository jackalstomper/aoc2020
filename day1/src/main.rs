use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut input: Vec<i32> = contents
        .split('\n')
        .filter(|e| !e.is_empty())
        .map(|e| e.parse::<i32>().unwrap())
        .collect();

    input.sort();

    // part 1
    for i in 0..input.len() {
        if i + 1 >= input.len() {
            println!("No solution found");
            return Ok(());
        }

        let a = input[i];
        let b = input[i + 1];
        if a + b == 2020 {
            println!("The answer for part1 is {}", a * b);
            break;
        }
    }

    // part 2
    for i in 0..input.len() {
        if i + 2 >= input.len() {
            println!("No solution found");
            return Ok(());
        }

        let a = input[i];
        let b = input[i + 1];
        let c = input[i + 2];
        if a + b + c == 2020 {
            println!("The answer for part1 is {}", a * b * c);
            break;
        }
    }

    Ok(())
}
