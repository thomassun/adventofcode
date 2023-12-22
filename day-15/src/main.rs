use std::collections::{HashMap, HashSet};
use std::ops::RangeInclusive;
use std::{i32, usize, vec};

use regex::Regex;

fn main() {
    let input = include_str!("../data/input.txt");
    let result = part1(&parse(input), 2_000_000);
    println!("part one:{result}");
    part2(&parse(input));
}

fn parse(input: &str) -> HashMap<(i32, i32), (i32, i32)> {
    let re = Regex::new(r"at x=(-?\d+), y=(-?\d+): .+ x=(-?\d+), y=(-?\d+)").unwrap();
    let mut map = HashMap::new();

    for line in input.lines() {
        let caps = re.captures(line).unwrap();
        let numbers = (
            caps.get(1).unwrap().as_str().parse::<i32>().unwrap(),
            caps.get(2).unwrap().as_str().parse::<i32>().unwrap(),
            caps.get(3).unwrap().as_str().parse::<i32>().unwrap(),
            caps.get(4).unwrap().as_str().parse::<i32>().unwrap(),
        );
        map.insert((numbers.0, numbers.1), (numbers.2, numbers.3));
    }
    map
}

fn part1(input: &HashMap<(i32, i32), (i32, i32)>, row: i32) -> usize {
    let mut b_present = HashSet::new();
    let mut result = HashSet::new();
    for (s, b) in input {
        if b.1 == row {
            b_present.insert(b.0);
        }
        let distance = (s.0 - b.0).abs() + (s.1 - b.1).abs();
        // (s.1 - ROW) + (s.0 - ?) <= distance
        // s.0 - ? <= distance - (s.1 -ROW)
        // o= distance - (s.1 -ROW)
        let o = distance - (s.1 - row).abs();
        if o < 0 {
            continue;
        }
        ((s.0 - o)..=(s.0 + o)).for_each(|e| {
            result.insert(e);
        });
    }
    result.iter().filter(|e| !b_present.contains(e)).count()
}
fn part2(input: &HashMap<(i32, i32), (i32, i32)>) {
    let mut result = vec![];
    // for row in 1..=4_000_000 {
    for row in 3_999_990..=4_000_000 {
        let mut row_result = vec![];
        for (s, b) in input {
            let distance = (s.0 - b.0).abs() + (s.1 - b.1).abs();
            // (s.1 - ROW) + (s.0 - ?) <= distance
            // s.0 - ? <= distance - (s.1 -ROW)
            // o= distance - (s.1 -ROW)
            let o = distance - (s.1 - row).abs();
            if o < 0 {
                continue;
            }
            let range = 1.max(s.0 - o) as usize..=4_000_000.min(s.0 + o) as usize;
            row_result.push(range);
        }
        // let x = row_result.into_iter().reduce(|acc, e| acc.chain(e));
        // let _x = (1..3).chain(4..8).chain(9..10);
        //todo early break the for if we find the row  with only one position which dont be covered by
        //any Sensor
        result.push(row_result);
    }
    println!("{:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        let sample = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";
        let expected = HashMap::from([
            ((2, 18), (-2, 15)),
            ((9, 16), (10, 16)),
            ((13, 2), (15, 3)),
            ((12, 14), (10, 16)),
            ((10, 20), (10, 16)),
            ((14, 17), (10, 16)),
            ((8, 7), (2, 10)),
            ((2, 0), (2, 10)),
            ((0, 11), (2, 10)),
            ((20, 14), (25, 17)),
            ((17, 20), (21, 22)),
            ((16, 7), (15, 3)),
            ((14, 3), (15, 3)),
            ((20, 1), (15, 3)),
        ]);
        assert_eq!(parse(sample), expected);
    }
    #[test]
    fn test_part1() {
        let input = HashMap::from([
            ((2, 18), (-2, 15)),
            ((9, 16), (10, 16)),
            ((13, 2), (15, 3)),
            ((12, 14), (10, 16)),
            ((10, 20), (10, 16)),
            ((14, 17), (10, 16)),
            ((8, 7), (2, 10)),
            ((2, 0), (2, 10)),
            ((0, 11), (2, 10)),
            ((20, 14), (25, 17)),
            ((17, 20), (21, 22)),
            ((16, 7), (15, 3)),
            ((14, 3), (15, 3)),
            ((20, 1), (15, 3)),
        ]);
        assert_eq!(part1(&input, 10), 26);
    }
}
