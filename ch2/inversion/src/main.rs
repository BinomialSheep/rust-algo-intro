fn naive(a: &[i32]) -> i64 {
    let mut count = 0;
    for i in 0..a.len() {
        for j in i + 1..a.len() {
            if a[i] > a[j] {
                count += 1;
            }
        }
    }
    count
}

/// マージソートを利用して転倒数を求める    
fn count_inversion_with_merge_sort(arr: &mut [i32]) -> i64 {
    if arr.len() <= 1 {
        return 0;
    }

    // 結果配列を初期化
    let mut result = vec![0; arr.len()];
    let mut result_index = 0;

    let mut count = 0;

    let mid = arr.len() / 2;
    count += count_inversion_with_merge_sort(&mut arr[..mid]);
    count += count_inversion_with_merge_sort(&mut arr[mid..]);

    let mut left_index = 0;
    let mut right_index = mid;

    while left_index < mid && right_index < arr.len() {
        // 安定ソートのため、同じ場合は左を採用
        if arr[left_index] <= arr[right_index] {
            result[result_index] = arr[left_index];
            left_index += 1;
            // 左側の要素の方が小さい場合は転倒数にはならない
        } else {
            result[result_index] = arr[right_index];
            right_index += 1;
            // 左側の残りの要素数分がarr[right_index]より大きい（転倒数）
            count += (mid - left_index) as i64;
        }
        result_index += 1;
    }
    // 残りの要素は転倒数にならないのでOK
    // 右が先に空になり、左だけが残っている場合
    while left_index < mid {
        result[result_index] = arr[left_index];
        left_index += 1;
        result_index += 1;
    }

    // 左が先に空になり、右だけが残っている場合
    while right_index < arr.len() {
        result[result_index] = arr[right_index];
        right_index += 1;
        result_index += 1;
    }

    // 結果を元の配列にコピー
    arr.copy_from_slice(&result);

    count
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    input.clear();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut a: Vec<i32> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    println!("{}", count_inversion_with_merge_sort(&mut a));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_naive_1() {
        let a = vec![1, 2, 3, 4, 5];
        assert_eq!(naive(&a), 0);
    }

    #[test]
    fn test_naive_2() {
        let a = vec![2, 3, 8, 6, 1];
        assert_eq!(naive(&a), 5);
    }

    #[test]
    fn test_count_inversion_with_merge_sort_1() {
        let mut a = vec![1, 2, 3, 4, 5];
        assert_eq!(count_inversion_with_merge_sort(&mut a), 0);
    }

    #[test]
    fn test_count_inversion_with_mer_2() {
        let mut a = vec![2, 3, 8, 6, 1];
        assert_eq!(count_inversion_with_merge_sort(&mut a), 5);
    }
}
