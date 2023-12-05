use std::fs;

use day_3::{find_intersection, priority_of_item, split_by_half};

fn main() {
    let file_name = "/Users/thomas/code/adventofcode/day-3/data/input.txt";
    // Part ONE
    let mut priority = 0;
    match fs::read_to_string(file_name) {
        Ok(contents) => {
            for line in contents.lines() {
                // let first = &line[..line.len()];
                let intersections = find_intersection(split_by_half(line));
                priority += intersections
                    .chars()
                    .fold(0, |acc, c| acc + priority_of_item(&c))
            }
        }
        Err(e) => panic!("{:#?}", e),
    }
    println!("Part One: {priority}");
    // Part TWO
    let mut priority = 0;
    match fs::read_to_string(file_name) {
        Ok(contents) => {
            for group in contents.lines().collect::<Vec<_>>().chunks(3) {
                // let first = &line[..line.len()];
                let intersections =
                    find_intersection((group[0], &find_intersection((group[1], group[2]))));
                priority += intersections
                    .chars()
                    .fold(0, |acc, c| acc + priority_of_item(&c))
            }
        }
        Err(e) => panic!("{:#?}", e),
    }
    eprintln!("Part Two = {:#?}", priority);
}
