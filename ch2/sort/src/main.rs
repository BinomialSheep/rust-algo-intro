use rand::Rng;
use std::time::Instant;

// 挿入ソート
fn insertion_sort(arr: &mut [i32]) {
    for i in 1..arr.len() {
        for j in (0..i).rev() {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            } else {
                break;
            }
        }
    }
}

fn merge(arr: &mut [i32], mid: usize) {
    let mut left_index = 0;
    let mut right_index = mid;

    // 結果配列を初期化
    let mut result = vec![0; arr.len()];
    let mut result_index = 0;

    // 左、右が両方空でない場合
    while left_index < mid && right_index < arr.len() {
        // 安定ソートのため、同じ場合は左を採用
        if arr[left_index] <= arr[right_index] {
            result[result_index] = arr[left_index];
            left_index += 1;
        } else {
            result[result_index] = arr[right_index];
            right_index += 1;
        }
        result_index += 1;
    }
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
}

fn merge_sort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }
    let mid = arr.len() / 2;
    // 左半分、右半分をそれぞれ再帰的にソート
    merge_sort(&mut arr[..mid]);
    merge_sort(&mut arr[mid..]);
    // マージ
    merge(arr, mid);
}

fn main() {
    let mut rng = rand::thread_rng();
    let mut total_insertion_duration = 0;
    let mut total_merge_duration = 0;
    let iterations = 5;

    for _ in 0..iterations {
        let mut numbers: Vec<i32> = (0..10000).map(|_| rng.gen_range(0..10000)).collect();
        let mut numbers2 = numbers.clone();

        // 挿入ソートの速度計測
        let start = Instant::now();
        insertion_sort(&mut numbers);
        let duration = start.elapsed();
        total_insertion_duration += duration.as_millis();

        // マージソートの速度計測
        let start = Instant::now();
        merge_sort(&mut numbers2);
        let duration = start.elapsed();
        total_merge_duration += duration.as_millis();
    }

    println!("挿入ソートの平均実行時間: {} ms", total_insertion_duration / iterations);
    println!("マージソートの平均実行時間: {} ms", total_merge_duration / iterations);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_sorted(arr: &[i32]) -> bool {
        for i in 0..arr.len() - 1 {
            if arr[i] > arr[i + 1] {
                return false;
            }
        }
        true
    }

    #[test]
    fn test_insertion_sort() {
        let mut numbers = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
        insertion_sort(&mut numbers);
        assert!(is_sorted(&numbers));
    }

    #[test]
    fn test_merge_sort() {
        let mut numbers = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
        merge_sort(&mut numbers);
        assert!(is_sorted(&numbers));
    }
}
