use std::collections::HashMap;

const KEY_CYCLE: [i32; 6] = [20, 60, 100, 140, 180, 220];
fn main() {
    //Part one
    let mut lines = include_str!("../data/input.txt").lines();
    let (mut c, mut v, mut v_at) = (0_i32, 1, HashMap::new());
    while let Some(instr) = lines
        .next()
        .map(|l| l.split_whitespace().collect::<Vec<_>>())
        .as_deref()
    {
        c += 1;
        if KEY_CYCLE.contains(&c) {
            println!("{v},{:?}", v_at.insert(c, v));
        }
        if let ["addx", number] = instr {
            c += 1;
            if KEY_CYCLE.contains(&c) {
                v_at.insert(c, v);
            }

            v += number.parse::<i32>().unwrap();
        }
    }
    println!(
        "Part One:{:?}",
        v_at.into_iter().fold(0, |acc, kv| { acc + kv.0 * kv.1 })
    );
    let part_two = part2();
    part_two.chunks(40).for_each(|chunk| {
        println!("{}", chunk.iter().collect::<String>());
    })
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

    screen
}
