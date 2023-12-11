use std::collections::HashMap;

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
    println!("{:?}", value_at);
    println!(
        "{:?}",
        value_at
            .into_iter()
            .fold(0, |acc, kv| { acc + kv.0 * kv.1 })
    );

    //Part TWO
    let mut lines = include_str!("../data/input.txt").lines();
    let mut cycle = 0_i32;
    let mut value = 1;
    let mut screen = (0..240).map(|_| '.').collect::<Vec<_>>();
    // screen.fill(b'.');
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
    println!("{:?}", value_at);
    println!(
        "{:?}",
        value_at
            .into_iter()
            .fold(0, |acc, kv| { acc + kv.0 * kv.1 })
    );
}
