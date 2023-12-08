use std::{error::Error, fs};
const LEN_OF_MARKER: usize = 4;

fn main() -> Result<(), Box<dyn Error>> {
    let path = "/Users/guoqingsun/code/adventofcode/day-6/data/input.txt";
    let stream = fs::read_to_string(path)?;
    // let stream = "bvwbjplbgvbhsrlpgdmjqwftvncz".to_owned();
    let chars = stream.chars().collect::<Vec<char>>();
    let mut n = 0;
    while n <= chars.len() - LEN_OF_MARKER {
        match is_dup(&chars[n..n + LEN_OF_MARKER]) {
            Ok(()) => {
                break;
            }
            Err(pos) => n += pos,
        }
    }
    if n == chars.len() - LEN_OF_MARKER && is_dup(&chars[n..]).is_err() {
        panic!("unable to resolve")
    }
    println!(
        "Part one = {}, token is `{:?}`",
        n + LEN_OF_MARKER,
        &chars[n..n + 4]
    );
    Ok(())
}

fn is_dup(seg: &[char]) -> Result<(), usize> {
    let mut result = Ok(());
    //aabb
    for i in 0..LEN_OF_MARKER - 1 {
        if seg[i + 1..LEN_OF_MARKER].contains(&seg[i]) {
            result = Err(i + 1);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_is_dup() {
        assert_eq!(is_dup(&['a', 'a', 'b', 'b']), Err(3));
        assert_eq!(is_dup(&['m', 'q', 'l', 'l']), Err(3));
        assert_eq!(is_dup(&['a', 'a', 'a', 'b']), Err(2));
        assert_eq!(is_dup(&['a', 'a', 'c', 'b']), Err(1));
        assert_eq!(is_dup(&['b', 'v', 'w', 'x']), Ok(()));
        assert_eq!(is_dup(&['b', 'v', 'w', 'b']), Err(1));
    }
}
