use std::str::Lines;

fn main() {
    let mut lines = include_str!("../data/input.txt").lines();
    let sum: u64 = parse(&mut lines)
        .into_iter()
        .filter(|&n| n <= 100_000)
        .sum();
    println!("part one = {:#?}", sum);

    //part2
    let mut lines = include_str!("../data/input.txt").lines();
    let mut parsed = parse(&mut lines);
    let to_be_freed = 30_000_000 - (70000000 - parsed.last().unwrap());
    parsed.sort_unstable();
    let result = parsed.into_iter().find(|&x| x >= to_be_freed).unwrap();
    println!("part two = {}", result)
}

fn parse(input: &mut Lines) -> Vec<u64> {
    let (mut total, mut subdirs) = (0, vec![]);
    loop {
        match input
            .next()
            .map(|s| s.split_whitespace().collect::<Vec<_>>())
            .as_deref()
        {
            Some(["$", "cd", ".."]) | None => break,
            Some(["$", "cd", dir]) if *dir != "/" => {
                subdirs.extend(parse(input));
                total += subdirs.last().unwrap();
            }
            Some([s, _]) if *s != "$" && *s != "dir" => {
                total += s.parse::<u64>().unwrap();
            }
            _ => (),
        }
    }
    // current_dir.size = size;
    subdirs.push(total);
    subdirs
}
