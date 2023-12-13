struct Monkey {
    bag: Vec<usize>,
    op: Box<dyn Fn(usize) -> usize>,
    div: usize,
    if_to: usize,
    else_to: usize,
    ins: usize,
}
fn main() {
    let mut monkies = include_str!("../data/input.txt")
        .split("\n\n")
        .map(|m| {
            let l: Vec<_> = m.lines().map(|l| l.split(": ").last().unwrap()).collect();
            Monkey {
                bag: l[1]
                    .split(", ")
                    .map(|n| n.parse::<usize>().unwrap())
                    .collect::<Vec<_>>(),
                op: {
                    let expr = l[2].rsplit_once("= ").unwrap().1;
                    op(expr)
                },
                div: l[3].rsplit_once(' ').unwrap().1.parse().unwrap(),
                if_to: l[4].rsplit_once(' ').unwrap().1.parse().unwrap(),
                else_to: l[5].rsplit_once(' ').unwrap().1.parse().unwrap(),
                ins: 0,
            }
        })
        .collect::<Vec<_>>();

    let mut bags = vec![vec![]; monkies.len()];
    let mo: usize = monkies.iter().map(|m| m.div).product();
    (0..10_000).for_each(|_| {
        monkies.iter_mut().enumerate().for_each(|(i, m)| {
            m.bag.append(&mut bags[i]);
            m.bag.drain(0..).for_each(|mut n| {
                n = (m.op)(n) % mo;
                bags[if n % m.div == 0 { m.if_to } else { m.else_to }].push(n);
                m.ins += 1;
            });
        });
    });
    monkies.sort_by(|a, b| b.ins.cmp(&a.ins));
    println!(
        "{}",
        monkies.iter().take(2).map(|m| m.ins).product::<usize>()
    );
}

fn op(expr: &str) -> Box<dyn Fn(usize) -> usize> {
    let x = expr.split(' ').collect::<Vec<_>>();
    match x[..] {
        [_, op, "old"] => match op {
            "+" => Box::new(|a| a * 2),
            "-" => Box::new(|_a| 0),
            "*" => Box::new(|a| a * a),
            "/" => Box::new(|_a| 1),
            _ => panic!("Wrong op of the expression"),
        },
        [_, op, right] => {
            let right_number = right.parse::<usize>().unwrap();
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_macro() {
        assert_eq!(op("old * 3")(3), 9);
        assert_eq!(op("old * old")(3), 9);

        assert_eq!(op("old - 2")(3), 1);
        assert_eq!(op("old / 2")(3), 1);
        assert_eq!(op("old / 2")(11), 5);
        assert_eq!(op("old / 2")(10), 5);
    }
}
