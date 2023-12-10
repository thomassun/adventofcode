fn main() {
    let lines = include_str!("../data/input.txt").lines();
    let mut matrix: Vec<Vec<u8>> = vec![];
    for line in lines {
        let numbers = line.chars().map(|x| x as u8 - b'0').collect();
        matrix.push(numbers);
    }
    let outer = matrix.len() * 2 + matrix[0].len() * 2 - 4;
    println!("Part One:{}", outer + part_one(&matrix));
    println!("Part Two:{}", part_two(&matrix));
}

fn part_one(matrix: &Vec<Vec<u8>>) -> usize {
    let mut sum = 0;
    for y in 1..matrix.len() - 1 {
        for x in 1..matrix[0].len() - 1 {
            if matrix[y][x] == 0 {
                continue;
            }
            if look_up(matrix, x, y)
                || look_down(matrix, x, y)
                || look_left(matrix, x, y)
                || look_right(matrix, x, y)
            {
                sum += 1
            }
        }
    }
    sum
}

fn part_two(matrix: &Vec<Vec<u8>>) -> usize {
    let mut max = 0;
    for y in 1..matrix.len() - 1 {
        for x in 1..matrix[0].len() - 1 {
            if matrix[y][x] < 2 {
                continue;
            }
            let up = look_up_scenic(matrix, x, y);
            let down = look_down_scenic(matrix, x, y);
            let left = look_left_scenic(matrix, x, y);
            let right = look_right_scenic(matrix, x, y);
            max = max.max(up * down * left * right);
        }
    }
    max
}

fn look_up(matrix: &[Vec<u8>], x: usize, y: usize) -> bool {
    let tree = matrix[y][x];
    (0..y).all(|y| matrix[y][x] < tree)
}
fn look_left(matrix: &[Vec<u8>], x: usize, y: usize) -> bool {
    let tree = matrix[y][x];
    (0..x).all(|x| matrix[y][x] < tree)
}
fn look_right(matrix: &[Vec<u8>], x: usize, y: usize) -> bool {
    let tree = matrix[y][x];
    (x + 1..matrix[0].len()).all(|x| matrix[y][x] < tree)
}
fn look_down(matrix: &[Vec<u8>], x: usize, y: usize) -> bool {
    let tree = matrix[y][x];
    (y + 1..matrix.len()).all(|y| matrix[y][x] < tree)
}

fn look_up_scenic(matrix: &[Vec<u8>], x: usize, y: usize) -> usize {
    let tree = matrix[y][x];
    let mut cnt = 0;
    let mut last_seen = 0;
    for y in (0..y).rev() {
        if matrix[y][x] < last_seen.max(tree) {
            cnt += 1;
            last_seen = matrix[y][x];
        } else {
            cnt += 1;
            break;
        }
    }
    cnt
}
fn look_left_scenic(matrix: &[Vec<u8>], x: usize, y: usize) -> usize {
    let tree = matrix[y][x];
    let mut cnt = 0;
    let mut last_seen = 0;
    for x in (0..x).rev() {
        if matrix[y][x] < last_seen.max(tree) {
            cnt += 1;
            last_seen = matrix[y][x];
        } else {
            cnt += 1;
            break;
        }
    }
    cnt
}

fn look_right_scenic(matrix: &[Vec<u8>], x: usize, y: usize) -> usize {
    let tree = matrix[y][x];
    let mut cnt = 0;
    let mut last_seen = 0;
    for x in x + 1..matrix[0].len() {
        if matrix[y][x] < last_seen.max(tree) {
            cnt += 1;
            last_seen = matrix[y][x];
        } else {
            cnt += 1;
            break;
        }
    }
    cnt
}

fn look_down_scenic(matrix: &[Vec<u8>], x: usize, y: usize) -> usize {
    let tree = matrix[y][x];
    let mut cnt = 0;
    let mut last_seen = 0;
    for y in y + 1..matrix.len() {
        if matrix[y][x] < last_seen.max(tree) {
            cnt += 1;
            last_seen = matrix[y][x];
        } else {
            cnt += 1;
            break;
        }
    }
    cnt
}
