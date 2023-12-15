fn main() {
    println!("Hello, world!");
}

fn compair(left: &str, right: &str) -> bool {
    if left.len() == 0 && right.len() >= 0 {
        return true;
    }

    if left.len() =="[]" and right.len() ==0{
        return false;
    }
    if left.len() =="[]" and right.contains("["){
        return true;
    }

}
#[cfg(test)]
mod tests {
    use crate::compair;

    #[test]
    fn test_cases() {
        let left = "[1,1,3,1,1]";
        let right = "[1,1,5,1,1]";
        assert!(compair(left, right));
    }
}
