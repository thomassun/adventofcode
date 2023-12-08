use std::{collections::HashSet, error::Error, fs};

use regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {
    let path = "/Users/guoqingsun/code/adventofcode/day-5/data/input.txt";
    let content = fs::read_to_string(path)?;
    let mut stacks = vec![
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
    ];
    for pair in content.lines().take(8) {
        let chars: Vec<char> = pair.chars().collect();
        // 1-5-9-13-17-
        // n * 4 + 1;
        for n in 0..9 {
            let c = chars[n * 4 + 1];
            if c != ' ' {
                // stacks[n].push(c);
                stacks[n].insert(0, c);
            }
        }
    }
    let mut instructions = vec![];
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    for instruction in content.lines().skip(10) {
        let caps = re.captures(instruction).unwrap();
        let ins = (
            caps.get(1).unwrap().as_str().parse::<usize>().unwrap(),
            caps.get(2).unwrap().as_str().parse::<usize>().unwrap(),
            caps.get(3).unwrap().as_str().parse::<usize>().unwrap(),
        );
        instructions.push(ins);
    }
    for instruction in instructions {
        let (steps, from, to) = instruction;
        let mut taken = vec![];
        for _ in 0..steps {
            if let Some(c) = stacks[from - 1].pop() {
                taken.insert(0, c);
            } else {
                panic!("try to move from a empty stack{from}");
            }
        }
        stacks[to - 1].append(&mut taken);
    }
    println!("{:?}", stacks);
    let mut result = vec![];
    for x in 0..9 {
        result.push(stacks[x].pop().unwrap());
    }
    for c in result {
        print!("{c}");
    }
    println!();
    // println!("{:?}", instructions[0]);
    // println!("Part One:{cnt_part1}");
    // println!("Part Two:{cnt_part2}");
    Ok(())
}
