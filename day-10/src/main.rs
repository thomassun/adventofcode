use std::collections::{HashMap, HashSet};

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
    part_two.chars().enumerate().for_each(|(i, c)| {
        if i % 40 == 39 {
            println!("{c}");
        } else {
            print!("{c}");
        }
    });
    // println!("Part Two:{}", &part2()[..8]);
}
//Part TWO
fn part2() -> String {
    let mut lines = include_str!("../data/input.txt").lines();
    let mut cycle = 0_i32;
    let mut sprite = 1;
    let mut screen = HashSet::new();
    // let mut row = 0;
    // screen.fill(b'.');
    while let Some(instr) = lines
        .next()
        .map(|l| l.split_whitespace().collect::<Vec<_>>())
        .as_deref()
    {
        match instr {
            ["noop"] => {
                cycle += 1;
                // row = cycle % 40;
                if (sprite - 1..=sprite + 1).contains(&((cycle - 1) % 40)) {
                    // if (sprite - 1..=sprite + 1).contains(&(cycle - 1)) {
                    screen.insert(cycle - 1);
                }
            }
            ["addx", number] => {
                cycle += 1;
                // row = cycle % 40;
                if (sprite - 1..=sprite + 1).contains(&((cycle - 1) % 40)) {
                    // if (sprite - 1..=sprite + 1).contains(&(cycle - 1)) {
                    screen.insert(cycle - 1);
                    // screen.insert(cycle - 1);
                }
                cycle += 1;
                // row = cycle % 40;
                if (sprite - 1..=sprite + 1).contains(&((cycle - 1) % 40)) {
                    screen.insert(cycle - 1);
                    // screen.insert(cycle - 1);
                }

                sprite += number.parse::<i32>().unwrap();
            }
            _ => (),
        }
    }
    // println!("Part Two:{:?}", screen);

    let mut output = std::iter::repeat('.').take(40 * 6).collect::<Vec<char>>();
    screen.into_iter().for_each(|n| output[n as usize] = '#');
    output.into_iter().collect::<String>()
}
