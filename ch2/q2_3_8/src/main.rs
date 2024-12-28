use std::collections::BTreeSet;

fn solve(x: i32, a: &[i32]) -> bool {
    let mut s = BTreeSet::new();
    for &num in a {
        if s.contains(&(x - num)) {
            return true;
        }
        s.insert(num);
    }
    false
}

fn main() {
    let x = 10;
    let _n = 5;
    let a = vec![1, 2, 3, 7, 8];

    println!("{}", if solve(x, &a) { "Yes" } else { "No" });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_case() {
        let x = 10;
        let a = vec![1, 2, 3, 7];
        assert_eq!(solve(x, &a), true);
    }

    #[test]
    fn test_no_pair() {
        let x = 6;
        let a = vec![1, 2, 3];
        assert_eq!(solve(x, &a), false);
    }

    #[test]
    fn test_empty_array() {
        let x = 5;
        let a = vec![];
        assert_eq!(solve(x, &a), false);
    }

    #[test]
    fn test_negative_values() {
        let x = -2;
        let a = vec![1, -3, 4, -1];
        assert_eq!(solve(x, &a), true);
    }

    #[test]
    fn test_duplicate_values() {
        let x = 4;
        let a = vec![2, 2, 2];
        assert_eq!(solve(x, &a), true);
    }
}
