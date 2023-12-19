use std::collections::HashSet;
use std::usize;

use nom::combinator::map_res;
use nom::multi::separated_list1;
use nom::{bytes::complete::tag, IResult};
use nom::{character::complete::digit1, sequence::separated_pair};
fn main() {
    let input = include_str!("../data/input.txt");
    println!("part one:{}", part1(input));
    // let result = parse(input) ;
}
fn part1(input: &str) -> usize {
    let mut cnt = 0;
    if let Ok(data) = parse(input) {
        let mut board = board(data.1);
        let max_depth = board.iter().fold(0, |max, rock| max.max(rock.1));
        let mut sand = (500, 0);
        loop {
            let down = (sand.0, sand.1 + 1);
            let left = (sand.0 - 1, sand.1 + 1);
            let right = (sand.0 + 1, sand.1 + 1);
            if down.1 > max_depth {
                break;
            }

            sand = match (board.get(&down), board.get(&left), board.get(&right)) {
                (None, _, _) => down,
                (_, None, _) => left,
                (_, _, None) => right,
                (Some(_), Some(_), Some(_)) => {
                    board.insert(sand);
                    cnt += 1;
                    (500, 0)
                }
            }
        }
    }
    cnt
}
fn parse(input: &str) -> IResult<&str, Vec<Vec<(usize, usize)>>> {
    separated_list1(
        tag("\n"),
        separated_list1(
            tag(" -> "),
            separated_pair(
                map_res(digit1, str::parse),
                tag(","),
                map_res(digit1, str::parse),
            ),
        ),
    )(input)
    // let mut parsed: HashSet<(usize, usize)>;
    // let x = nom_iter.collect::<Vec<Vec<_>>>();
    // nom_iter.finish();
    //
}
fn board(data: Vec<Vec<(usize, usize)>>) -> HashSet<(usize, usize)> {
    let mut parsed = HashSet::new();
    for p in data {
        p.into_iter().fold(None, |i, point| {
            if i.is_none() {
                parsed.insert(point);
            } else {
                let prev: (usize, usize) = i.unwrap();
                (prev.0.min(point.0)..=(point.0.max(prev.0))).for_each(|h| {
                    parsed.insert((h, point.1));
                });
                (prev.1.min(point.1)..=(point.1.max(prev.1))).for_each(|l| {
                    parsed.insert((point.0, l));
                });
            }
            Some(point)
        });
    }
    parsed
}
#[cfg(test)]
mod tests {

    use super::*;
    const SAMPLE: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn test_parse() {
        let expected_1 = vec![
            vec![(498, 4), (498, 6), (496, 6)],
            vec![(503, 4), (502, 4), (502, 9), (494, 9)],
        ];
        assert_eq!(parse(SAMPLE), Ok(("", expected_1)))
    }
    #[test]
    fn test_board() {
        let input = vec![
            vec![(498, 4), (498, 6), (496, 6)],
            vec![(503, 4), (502, 4), (502, 9), (494, 9)],
        ];
        assert_eq!(
            board(input),
            HashSet::from([
                (498, 4),
                (498, 5),
                (498, 6),
                (497, 6),
                (496, 6),
                (503, 4),
                (502, 4),
                (502, 5),
                (502, 6),
                (502, 7),
                (502, 8),
                (502, 9),
                (501, 9),
                (500, 9),
                (499, 9),
                (498, 9),
                (497, 9),
                (496, 9),
                (495, 9),
                (494, 9)
            ])
        )
    }
}
