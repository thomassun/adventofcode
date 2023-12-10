use std::collections::HashSet;

type Point = (i32, i32);

pub fn lead(direction: &str, head: &mut Point) {
    match direction {
        "R" => head.0 += 1,
        "L" => head.0 -= 1,
        "D" => head.1 += 1,
        "U" => head.1 -= 1,
        _ => panic!("{direction}"),
    };
}

pub fn follow(leader: Point, follower: &mut Point) {
    let delta = (leader.0 - follower.0, leader.1 - follower.1);
    if delta.0.abs() == 2 || delta.1.abs() == 2 {
        follower.0 += delta.0.signum();
        follower.1 += delta.1.signum();
    }
}

pub fn visit(knot_count: usize) -> usize {
    let mut rope = vec![(0, 0); knot_count];
    let mut visited = HashSet::from([(0, 0)]);

    for line in include_str!("../data/input.txt").lines() {
        let mut split = line.split(' ');
        let direction = split.next().unwrap();
        let count: i32 = split.next().unwrap().parse().unwrap();

        for _ in 0..count {
            lead(direction, &mut rope[0]);

            for i in 1..rope.len() {
                follow(rope[i - 1], &mut rope[i]);
            }

            visited.insert(*rope.last().unwrap());
        }
    }
    visited.len()
}

fn main() {
    println!("Part one: {}\nPart two:{}",visit(2),visit(10));
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(visit(2), 13);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(visit(10), 1);
    }
}
