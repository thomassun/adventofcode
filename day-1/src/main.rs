use std::fs;

fn main() {
    let file_name = "/Users/guoqingsun/code/adventofcode/day-1/data/input.txt";
    let mut vec: Vec<usize> = Vec::new();
    match fs::read_to_string(file_name) {
        Ok(content) => {
            let mut count: usize = 0;
            for line in content.lines() {
                if line.is_empty() {
                    vec.push(count);
                    count = 0;
                } else if let Ok(calorie) = line.parse::<usize>() {
                    count += calorie;
                }
            }
            vec.push(count);
        }
        Err(e) => {
            panic!("{:#?}", e)
        }
    }
    vec.sort_by(|a, b| b.cmp(a));
    print!("{:#?}", vec.iter().take(3).fold(0, |acc, f| f + acc));
}
