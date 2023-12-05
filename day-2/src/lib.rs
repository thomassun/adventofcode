use std::cmp::Ordering;

#[derive(PartialEq, Debug, Clone)]
pub enum Shape {
    Paper = 1,
    Scissors = 2,
    Rock = 3,
}
impl PartialOrd for Shape {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self == other {
            return Some(Ordering::Equal);
        }
        if self == &Shape::Paper && other == &Shape::Rock {
            return Some(Ordering::Greater);
        }
        if self == &Shape::Rock && other == &Shape::Paper {
            return Some(Ordering::Less);
        }
        if self.clone() as i32 > other.clone() as i32 {
            return Some(Ordering::Greater);
        }
        Some(Ordering::Less)
    }
}
impl Shape {
    pub fn score_of_the_shape(&self) -> i32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
    pub fn match_score(&self, opponent: &Shape) -> i32 {
        if self == opponent {
            return 3;
        }
        if self > opponent {
            return 6;
        }
        0
    }
    /// this function will yiled a Shape that will win me
    pub fn to_win(&self) -> Shape {
        match self {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissors,
            Shape::Scissors => Shape::Rock,
        }
    }

    /// this function will yiled a Shape that will loss from me
    pub fn to_loss(&self) -> Shape {
        match self {
            Shape::Rock => Shape::Scissors,
            Shape::Paper => Shape::Rock,
            Shape::Scissors => Shape::Paper,
        }
    }
}

impl From<&str> for Shape {
    fn from(value: &str) -> Self {
        match value {
            "A" | "X" => Shape::Rock,
            "B" | "Y" => Shape::Paper,
            "C" | "Z" => Shape::Scissors,
            _ => panic!("invalid value"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order() {
        assert!(Shape::Paper == Shape::Paper);
        assert_eq!(Shape::Rock, Shape::Rock);
        assert!(Shape::Scissors == Shape::Scissors);

        assert!(Shape::Paper > Shape::Rock);
        assert!(Shape::Paper < Shape::Scissors);
        assert!(Shape::Rock > Shape::Scissors);
        assert!(Shape::Rock < Shape::Paper)
    }
    #[test]
    fn get_score() {
        assert_eq!(Shape::Rock.score_of_the_shape(), 1);
        assert_eq!(Shape::Paper.score_of_the_shape(), 2);
        assert_eq!(Shape::Scissors.score_of_the_shape(), 3);
    }
    #[test]
    fn score_of_match() {
        assert_eq!(Shape::Rock.match_score(&Shape::Rock), 3);
        assert_eq!(Shape::Paper.match_score(&Shape::Paper), 3);
        assert_eq!(Shape::Scissors.match_score(&Shape::Scissors), 3);

        assert_eq!(Shape::Rock.match_score(&Shape::Scissors), 6);
        assert_eq!(Shape::Rock.match_score(&Shape::Paper), 0);

        assert_eq!(Shape::Paper.match_score(&Shape::Scissors), 0);
        assert_eq!(Shape::Paper.match_score(&Shape::Rock), 6);

        assert_eq!(Shape::Scissors.match_score(&Shape::Rock), 0);
        assert_eq!(Shape::Scissors.match_score(&Shape::Paper), 6);
    }
    #[test]
    fn from_func() {
        assert_eq!(Shape::from("A"), Shape::Rock);
        assert_eq!(Shape::from("X"), Shape::Rock);

        assert_eq!(Shape::from("B"), Shape::Paper);
        assert_eq!(Shape::from("Y"), Shape::Paper);

        assert_eq!(Shape::from("C"), Shape::Scissors);
        assert_eq!(Shape::from("Z"), Shape::Scissors);
    }
    #[test]
    fn to_win() {
        assert_eq!(Shape::Rock.to_win(), Shape::Paper);
        assert_eq!(Shape::Paper.to_win(), Shape::Scissors);
        assert_eq!(Shape::Scissors.to_win(), Shape::Rock);
    }
    #[test]
    fn to_loss() {
        assert_eq!(Shape::Rock.to_loss(), Shape::Scissors);
        assert_eq!(Shape::Scissors.to_loss(), Shape::Paper);
        assert_eq!(Shape::Paper.to_loss(), Shape::Rock);
    }
}
