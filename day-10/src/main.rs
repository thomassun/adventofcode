use std::{
    collections::{HashMap, HashSet},
    iter,
};

const KEY_CYCLE: [i32; 6] = [20, 60, 100, 140, 180, 220];
fn main() {
    //Part one
    let mut lines = include_str!("../data/input.txt").lines();
    let mut cycle = 0_i32;
    let mut value = 1;
    let mut value_at = HashMap::new();
    while let Some(instr) = lines
        .next()
        .map(|l| l.split_whitespace().collect::<Vec<_>>())
        .as_deref()
    {
        match instr {
            ["noop"] => {
                cycle += 1;
                if KEY_CYCLE.contains(&cycle) {
                    println!("{value},{:?}", value_at.insert(cycle, value));
                }
            }
            ["addx", number] => {
                cycle += 1;
                if KEY_CYCLE.contains(&cycle) {
                    value_at.insert(cycle, value);
                }
                cycle += 1;
                if KEY_CYCLE.contains(&cycle) {
                    value_at.insert(cycle, value);
                }

                value += number.parse::<i32>().unwrap();
            }
            _ => (),
        }
    }
    // println!("{:?}", value_at);
    println!(
        "Part One:{:?}",
        value_at
            .into_iter()
            .fold(0, |acc, kv| { acc + kv.0 * kv.1 })
    );
    let part_two = part2();
    part_two.chunks(40).for_each(|chunk| {
        println!("{}", chunk.iter().collect::<String>());
    })
    // println!("Part Two:{}", &part2()[..8]);
}
//Part TWO
fn part2() -> Vec<char> {
    let (mut c, mut s, mut screen) = (0_i32, 1, Vec::with_capacity(40 * 6));
    for instr in include_str!("../data/input.txt").lines() {
        let dot = ((s - 1 <= c % 40 && s + 1 >= c % 40) as u8 * 3 + 32) as char;
        // ' '(space) = 32; '#' = 35
        screen.push(dot);
        c += 1;
        if &instr[..4] == "addx" {
            let dot = ((s - 1 <= c % 40 && s + 1 >= c % 40) as u8 * 3 + 32) as char;
            screen.push(dot);
            c += 1;
            s += instr[5..].parse::<i32>().unwrap();
        }
    }
    // println!("Part Two:{:?}", screen);

    screen
}
