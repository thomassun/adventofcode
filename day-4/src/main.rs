use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let path = "/Users/guoqingsun/code/adventofcode/day-4/data/input.txt";
    let mut cnt_part1 = 0;
    let mut cnt_part2 = 0;
    let content = fs::read_to_string(path)?;
    for pair in content.lines() {
        let numbers = pair
            .split(',')
            .flat_map(|n| n.split('-'))
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        if (numbers[0] <= numbers[2] && numbers[1] >= numbers[3])
            || (numbers[0] >= numbers[2] && numbers[1] <= numbers[3])
        {
            cnt_part1 += 1;
        }
        //x1(0) <= y2(3) && y1(2) <= x2(1)
        if numbers[0] <= numbers[3] && numbers[2] <= numbers[1] {
            cnt_part2 += 1;
        }
    }
    println!("Part One:{cnt_part1}");
    println!("Part Two:{cnt_part2}");
    Ok(())
}
