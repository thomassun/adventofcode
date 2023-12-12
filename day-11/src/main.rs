struct Monkey {
    bag: Vec<usize>,
    op: Box<dyn FnOnce(usize) -> usize>,
    div: usize,
    if_to: usize,
    else_to: usize,
    ins: usize,
}
fn main() {
    let lines = include_str!("../data/input.txt")
        .lines()
        .collect::<Vec<_>>();
    // .chunks(7);
    lines.chunks(7).for_each(|ch| {
        // let monkey = ch[0].parse::<i32>().unwrap();
        let items_str = ch[1].split(&[':', ','][..]).collect::<Vec<_>>();
        let items;
        if let [_, end @ ..] = &items_str[..] {
            items = end
                .iter()
                .map(|&x| x.trim().parse::<i32>().unwrap())
                // .map(|&x| x)
                .collect::<Vec<_>>();
        };
        // println!("{:?}", ch[1]);
        let func = op(ch[2].split("new = ").collect::<Vec<_>>()[1]);
        let next_func = next_step(&ch[3..]);
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
            _ => panic!("Wrong op of the expression"),
        },
        [_, op, right] => {
            let right_number = right.parse::<i32>().unwrap();
            match op {
                "+" => Box::new(move |a| a + right_number),
                "-" => Box::new(move |a| a - right_number),
                "*" => Box::new(move |a| a * right_number),
                "/" => Box::new(move |a| a / right_number),
                _ => panic!("Wrong op of the expression"),
            }
        }
        _ => panic!("Wrong expression"),
    }
}

fn next_step(expr: &[&str]) -> Box<dyn FnOnce(i32) -> i32> {
    let number: i32 = expr[0].split("by ").collect::<Vec<_>>()[1].parse().unwrap();
    let true_to: i32 = expr[1].split("monkey ").collect::<Vec<_>>()[1]
        .parse()
        .unwrap();
    let else_to: i32 = expr[2].split("monkey ").collect::<Vec<_>>()[1]
        .parse()
        .unwrap();
    Box::new(move |d| if d % number == 0 { true_to } else { else_to })
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_macro() {
        assert_eq!(op("old * 3")(3), 9);
        assert_eq!(op("old * old")(3), 9);
        assert_eq!(op("old + 3")(3), 6);
        assert_eq!(op("old - 2")(3), 1);
        assert_eq!(op("old / 2")(3), 1);
        assert_eq!(op("old / 2")(11), 5);
        assert_eq!(op("old / 2")(10), 5);
    }
}
