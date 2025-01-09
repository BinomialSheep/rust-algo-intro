use std::io::{self, BufRead};

fn merge(arr: &mut [i32], mid: usize) {
    let left = arr[0..mid].to_vec();
    let right = arr[mid..].to_vec();

    let mut i = 0;

    let mut left_idx = 0;
    let mut right_idx = 0;
    while left_idx < left.len() && right_idx < right.len() {
        if left[left_idx] <= right[right_idx] {
            // 左から採用
            arr[i] = left[left_idx];
            left_idx += 1;
        } else {
            // 右から採用
            arr[i] = right[right_idx];
            right_idx += 1;
        }
        i += 1;
    }
    while left_idx < left.len() {
        arr[i] = left[left_idx];
        left_idx += 1;
        i += 1;
    }
    while right_idx < right.len() {
        arr[i] = right[right_idx];
        right_idx += 1;
        i += 1;
    }
}

fn merge_sort(arr: &mut [i32]) {
    /* 基底段階 */
    if arr.len() <= 1 {
        return;
    }
    /* 再帰段階 */
    // 分割
    let mid = arr.len() / 2;
    let (left, right) = arr.split_at_mut(mid);
    // 統治
    merge_sort(left);
    merge_sort(right);
    // 結合
    merge(arr, mid);
}

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn read_vec<T: std::str::FromStr>() -> Vec<T>
where
    T::Err: std::fmt::Debug,
{
    read_line()
        .split_whitespace()
        .map(|x| x.parse::<T>().unwrap())
        .collect()
}

fn print_vec<T: std::fmt::Display>(vec: &[T]) {
    println!(
        "{}",
        vec.iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}

// verify：https://algo-method.com/tasks/444/submissions/1633556
fn main() {
    let _n: usize = read_line().parse().unwrap();
    let mut arr: Vec<i32> = read_vec();

    merge_sort(&mut arr);

    print_vec(&arr);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_sort_reversed() {
        let mut arr = vec![5, 4, 3, 2, 1];
        merge_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }
}
