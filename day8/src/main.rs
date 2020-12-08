use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

#[derive(PartialEq)]
enum OpType {
    Acc,
    Jmp,
    Nop,
}

struct Op {
    op_type: OpType,
    arg: i32,
}

fn main() {
    let content = read_file();
    let mut ops: Vec<Op> = content
        .lines()
        .map(|line| {
            let words: Vec<&str> = line.split(' ').collect();
            let op = words[0];
            let arg = words[1];
            Op {
                op_type: match op {
                    "acc" => OpType::Acc,
                    "jmp" => OpType::Jmp,
                    "nop" => OpType::Nop,
                    _ => panic!("Unknown op type: {}", op),
                },
                arg: arg.parse().unwrap(),
            }
        })
        .collect();

    println!("Part 1 result: {}", execute(&mut ops, false));
    println!("Part 2 result: {}", execute(&mut ops, true));
}

fn execute(ops: &mut Vec<Op>, part2: bool) -> i32 {
    let mut accumulator = 0;
    let mut index = 0;
    let mut last = (0, 0);
    let mut seen = HashSet::new();
    seen.reserve(ops.len());

    while (index as usize) < ops.len() && index >= 0 {
        if let Some(_) = seen.get(&index) {
            if !part2 {
                return last.1;
            }

            // repair instruction
            let op = &mut ops[last.0 as usize];
            if op.op_type == OpType::Jmp {
                op.op_type = OpType::Nop;
            } else {
                op.op_type = OpType::Jmp;
            }

            accumulator = last.1;
            index = last.0;
        }

        seen.insert(index);
        let op = &ops[index as usize];
        match op.op_type {
            OpType::Acc => {
                accumulator += op.arg;
                index += 1;
            }
            OpType::Jmp => {
                last = (index, accumulator);
                index += op.arg;
            }
            OpType::Nop => {
                index += 1;
            }
        }
    }

    accumulator
}

fn read_file() -> String {
    let mut s = String::new();
    File::open("input.txt")
        .unwrap()
        .read_to_string(&mut s)
        .unwrap();
    s
}
