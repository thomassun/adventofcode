use std::fs;

use day_2::Shape;

fn main() {
    let file_name = "/Users/guoqingsun/code/adventofcode/day-2/data/input.txt";
    let mut score = 0;
    match fs::read_to_string(file_name) {
        Ok(content) => {
            for line in content.lines() {
                let splitted = line.split(' ').collect::<Vec<&str>>();
                let other = Shape::from(splitted[0]);
                let me = Shape::from(splitted[1]);
                score += me.score_of_the_shape() + me.match_score(&other);
            }
        }
        Err(e) => {
            panic!("{:#?}", e)
        }
    }
    println!("Part One:{score}");

    // PART TWO
    let mut score = 0;
    match fs::read_to_string(file_name) {
        Ok(content) => {
            for line in content.lines() {
                let splitted = line.split(' ').collect::<Vec<&str>>();
                let other = Shape::from(splitted[0]);
                let instruction = splitted[1];
                let me = match instruction {
                    "X" => other.to_loss(),
                    "Y" => other.clone(),
                    "Z" => other.to_win(),
                    _ => panic!("Unimplemented!"),
                };
                score += me.score_of_the_shape() + me.match_score(&other);
            }
        }
        Err(e) => {
            panic!("{:#?}", e)
        }
    }
    println!("Part Two:{score}");
}
