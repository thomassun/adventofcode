use std::collections::HashSet;

pub fn split_by_half(input: &str) -> (&str, &str) {
    (&input[0..input.len() / 2], &input[input.len() / 2..])
}

pub fn find_intersection(combines: (&str, &str)) -> String {
    combines
        .0
        .chars()
        .filter(|char_a| combines.1.contains(*char_a))
        .collect::<HashSet<char>>()
        .into_iter()
        .fold("".to_owned(), |acc, c| format!("{}{}", acc, c))
}
pub fn priority_of_item(i: &char) -> usize {
    match i {
        'a'..='z' => (*i as usize) - ('a' as usize) + 1,
        'A'..='Z' => (*i as usize) - ('A' as usize) + 27,
        _ => panic!("Not implemented"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn functions() {
        assert_eq!(
            find_intersection(split_by_half("vJrwpWtwJgWrhcsFMMfFFhFp")),
            "P"
        );
        assert_eq!(
            find_intersection(split_by_half("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL")),
            "L"
        );
        assert_eq!(find_intersection(split_by_half("PmmdzqPrVvPwwTWBwg")), "P");
        assert_eq!(
            find_intersection(split_by_half("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn")),
            "V"
        );
        assert_eq!(find_intersection(split_by_half("ttgJtRGJQctTZtZT")), "T");
        assert_eq!(
            find_intersection(split_by_half("CrZsJsPPZsGzwwsLwLmpwMDw")),
            "S"
        )
    }
    #[test]
    fn t_priority_of_item() {
        assert_eq!(priority_of_item(&'a'), 1);
        assert_eq!(priority_of_item(&'A'), 27);

        assert_eq!(priority_of_item(&'z'), 26);
        assert_eq!(priority_of_item(&'Z'), 52);

        assert_eq!(priority_of_item(&'p'), 16);
        assert_eq!(priority_of_item(&'P'), 42);
        assert_eq!(priority_of_item(&'s'), 19);
    }
    #[test]
    fn intersections_for_3() {
        let a = "vJrwpWtwJgWrhcsFMMfFFhFp";
        let b = "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL";
        let c = "PmmdzqPrVvPwwTWBwg";
        assert_eq!(find_intersection((a, &find_intersection((b, c)))), "r");
    }
}
