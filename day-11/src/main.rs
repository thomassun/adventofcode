use std::future;

fn main() {
    let lines = include_str!("../data/input.txt")
        .lines()
        .collect::<Vec<_>>();
    // .chunks(7);
    lines.chunks(7).for_each(|ch| {
        // let monkey = ch[0].parse::<i32>().unwrap();
        let items = ch[1].split(&[':', ','][..]).collect::<Vec<_>>();
        if let [_, end @ ..] = &items[..] {
            let items = end
                .iter()
                .map(|&x| x.trim().parse::<i32>().unwrap())
                // .map(|&x| x)
                .collect::<Vec<_>>();
        };
        // println!("{:?}", ch[1]);
        let func = ch[2].split("new = ").collect::<Vec<_>>()[1];
        println!("{:?}", func);
        // println!("{:?}", ch[3]);
        // println!("{:?}", ch[4]);
        // println!("{:?}", ch[5]);
    });
}

fn op(expr: &str) -> Box<dyn FnOnce(i32) -> i32> {
    let x = expr.split(' ').collect::<Vec<_>>();
    match x[..] {
        [_, op, "old"] => match op {
            "+" => Box::new(|a| a + a),
            "-" => Box::new(|_a| 0),
            "*" => Box::new(|a| a * a),
            "/" => Box::new(|_a| 1),
            _ => panic!("CANNOT BE HERE"),
        },
        [_, op, right] => {
            let right_number = right.parse::<i32>().unwrap();
            match op {
                "+" => Box::new(move |a| a + right_number),
                "-" => Box::new(move |a| a - right_number),
                "*" => Box::new(move |a| a * right_number),
                "/" => Box::new(move |a| a / right_number),
                _ => panic!("CANNOT BE HERE"),
            }
        }
        _ => panic!("CANNOT BE HERE"),
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_macro() {
        let op_str = "old * 3";
        assert_eq!(op(op_str)(3), 9);
    }
}
