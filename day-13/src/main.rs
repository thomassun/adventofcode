use std::cmp::Ordering;

fn main() {
    let lines = include_str!("../data/input.txt").split("\n\n").enumerate();
    println!(
        "Part One:{}",
        lines.fold(0, |accu, (idx, block)| {
            accu + if let [left, right] = block.split('\n').collect::<Vec<_>>()[..] {
                if comp(Some(left), Some(right)) == Ordering::Less {
                    idx + 1
                } else {
                    0
                }
            } else {
                0
            }
        })
    );
    let packets = include_str!("../data/input.txt")
        .split('\n')
        .filter(|l| !l.is_empty())
        .filter(|l| comp(Some(l), Some("[[6]]")) == Ordering::Less)
        .collect::<Vec<_>>();
    let pos2 = packets.len() + 2;
    let pos1 = packets
        .iter()
        .filter(|l| comp(Some(l), Some("[[2]]")) == Ordering::Less)
        .count()
        + 1;

    println!("Part Two:{}", pos1 * pos2);
}

//        [[1],[2,3,4]], [[1],4]
fn comp(left: Option<&str>, right: Option<&str>) -> Ordering {
    let result;
    if left.is_none() {
        return if right.is_some() {
            Ordering::Less
        } else {
            Ordering::Equal
        };
    }

    if left.is_some() && right.is_none() {
        return Ordering::Greater;
    }
    if left.unwrap() == "[]" {
        if right.unwrap() == "[]" {
            return Ordering::Equal;
        } else {
            return Ordering::Less;
        }
    }
    if right.unwrap() == "[]" {
        return Ordering::Greater;
    }
    let ((l_h, l_t), (mut r_h, r_t)) = (extract(left), extract(right));
    let l_h_str = l_h.unwrap();
    let r_h_str = r_h.unwrap();
    if &l_h_str[0..1] != "[" {
        // it's a number
        if &r_h_str[0..1] == "[" {
            (r_h, _) = extract(r_h);
            result = comp(l_h, r_h);
        } else {
            //number vs number
            result = l_h_str
                .parse::<i32>()
                .unwrap()
                .cmp(&r_h_str.parse::<i32>().unwrap())
        }
    } else {
        // left is an list
        if &r_h_str[0..1] == "[" {
            // the right one is also a list
            result = comp(l_h, r_h);
        } else {
            // list vs number
            result = comp(r_h, l_h).reverse();
        }
    }

    if result != Ordering::Equal {
        result
    } else {
        comp(l_t, r_t)
    }
}
fn extract(expr: Option<&str>) -> (Option<&str>, Option<&str>) {
    match expr {
        Some(expr) => {
            let (mut h_s, mut h_e, t_s, t_e);
            if expr == "[]" {
                return (None, None);
            }
            if &expr[0..1] != "[" {
                h_s = 0;
                t_e = expr.len();
                // it a digit like 1,[2,3] or just 1 OR 12,2
                h_e = expr
                    .bytes()
                    .position(|p| p as char == ',')
                    .unwrap_or(expr.len());
                t_s = h_e + 1;
            } else {
                //[[1,2,[[3,4,5]]],6]
                //[1,2,3,4]
                // [1,[2,3]]
                // [1],[2,[3,[4,[5]]]]
                // [[1],[2,[3,[4,[5]]]]]
                h_s = 1;
                h_e = close_bracket_pos(expr, 0);
                if h_e == expr.len() - 1 {
                    //covers all
                    t_e = expr.len() - 1;
                    if &expr[h_s..h_s + 1] == "[" {
                        h_e = close_bracket_pos(expr, h_s) + 1;
                        t_s = h_e + 1;
                    } else {
                        h_e = expr
                            .bytes()
                            .position(|p| p as char == ',')
                            .unwrap_or(expr[h_s..].len());
                        t_s = h_e + 1;
                    }
                } else {
                    h_s = 0;
                    h_e += 1;
                    t_s = h_e + 1;
                    t_e = expr.len();
                }
            }
            (
                Some(&expr[h_s..h_e]),
                if t_s > expr.len() - 1 {
                    None
                } else {
                    Some(&expr[t_s..t_e])
                },
            )
        }
        None => (None, None),
    }
}

fn close_bracket_pos(expr: &str, open_pos: usize) -> usize {
    //[[11,2,2]],[333]
    let mut cnt = 1;
    let mut pos = open_pos;
    while cnt > 0 {
        pos += 1;
        if &expr[pos..pos + 1] == "[" {
            cnt += 1;
        }
        if &expr[pos..pos + 1] == "]" {
            cnt -= 1;
        }
    }
    pos
}
#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use crate::{comp, extract};

    #[test]
    fn test_cases() {
        assert_eq!(
            comp(
                Some("[[6]]"),
                Some("[[6,[],3],[[],[[6],[4,9,10]],[[],3,[],8,[1,8]],[[10,2,10]],9]]"),
            ),
            Ordering::Less
        );
        assert_eq!(
            comp(
                Some("[[[6],[2,2,3,[5,8,9,0,0],2],3,[]],[5,[1]],[],[9,[[0],[6,3]],[2],3,8]]"),
                Some("[[6,[],3],[[],[[6],[4,9,10]],[[],3,[],8,[1,8]],[[10,2,10]],9]]"),
            ),
            Ordering::Greater
        );
        assert_eq!(
            comp(
                Some("[[[6],[2,2,3,[5,8,9,0,0],2],3,[]],[5,[1]],[],[9,[[0],[6,3]],[2],3,8]]"),
                Some("[[6]]"),
            ),
            Ordering::Greater
        );
    }
    #[test]
    fn test_extract() {
        assert_eq!(extract(Some("[[]]")), (Some("[]"), None));
        assert_eq!(
            extract(Some("[[1],[2,3,4]]")),
            (Some("[1]"), Some("[2,3,4]"))
        );
        assert_eq!(extract(Some("[[1],4]")), (Some("[1]"), Some("4")));
        assert_eq!(extract(Some("[1,[]]")), (Some("1"), Some("[]")));
        assert_eq!(extract(Some("1,2,3")), (Some("1"), Some("2,3")));
        assert_eq!(extract(Some("[10,2,3]")), (Some("10"), Some("2,3")));
        assert_eq!(extract(Some("[1,2,[3]]")), (Some("1"), Some("2,[3]")));
        assert_eq!(
            extract(Some("[[[2,3,4]],1,2,[3]]")),
            (Some("[[2,3,4]]"), Some("1,2,[3]"))
        );
        assert_eq!(
            extract(Some("[1,[2,[3,[4,[5,6,7]]]],8,[9]]")),
            (Some("1"), Some("[2,[3,[4,[5,6,7]]]],8,[9]"))
        );
        assert_eq!(
            extract(Some("[2,[3,[4,[5,6,7]]]],8,[9]")),
            (Some("[2,[3,[4,[5,6,7]]]]"), Some("8,[9]"))
        );
        assert_eq!(
            extract(Some("[2,[3,[4,[5,6,7]]]]")),
            (Some("2"), Some("[3,[4,[5,6,7]]]"))
        );
    }
}
