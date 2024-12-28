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

// 普通のマージソート
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

// マージソートの途中で、閾値以下なら挿入ソートを使用する
fn hybrid_sort(arr: &mut [i32], threshold: usize) {
    if arr.len() <= 1 {
        return;
    }
    // 閾値以下なら挿入ソートを使用
    if arr.len() <= threshold {
        insertion_sort(arr);
        return;
    }

    let mid = arr.len() / 2;
    // 左半分、右半分をそれぞれ再帰的にソート
    hybrid_sort(&mut arr[..mid], threshold);
    hybrid_sort(&mut arr[mid..], threshold);
    // マージ
    merge(arr, mid);
}

// 二分挿入ソート
fn binary_insertion_sort(arr: &mut [i32]) {
    for i in 1..arr.len() {
        let key = arr[i];
        let mut left = 0;
        let mut right = i;

        // 二分探索で挿入位置を見つける
        while left < right {
            let mid = (left + right) / 2;
            if arr[mid] <= key {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        // 挿入位置を見つけたら、要素をシフトして挿入
        for j in (left..i).rev() {
            arr[j + 1] = arr[j];
        }
        arr[left] = key;
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let iterations = 3;
    let thresholds = vec![16, 32, 48, 64];

    // マージソートとハイブリッドソートを比較
    for i in 10..26 {
        let mut total_merge_duration = 0;
        let mut total_hybrid_duration = 0;

        let list_length = 2_i32.pow(i);
        for _ in 0..iterations {
            let mut original_numbers: Vec<i32> = (0..list_length).map(|_| rng.gen_range(0..10000)).collect();
            
            let start = Instant::now();
            merge_sort(&mut original_numbers);
            let duration = start.elapsed();
            total_merge_duration += duration.as_millis();
            
            let start = Instant::now();
            let mut numbers = original_numbers.clone();
            hybrid_sort(&mut numbers, 16);
            let duration = start.elapsed();
            total_hybrid_duration += duration.as_millis();
        }
        println!("リストの長さ: {}, マージソート時間: {}ms, ハイブリッドソート時間: {}ms", list_length, total_merge_duration / iterations, total_hybrid_duration / iterations);
    }

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

    #[test]
    fn test_binary_insertion_sort() {
        let mut numbers = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
        binary_insertion_sort(&mut numbers);
        assert!(is_sorted(&numbers));
    }

    #[test]
    fn test_hybrid_sort() {
        let mut numbers = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 8, 9, 7, 9, 3, 2, 3, 8, 4, 6, 2];
        hybrid_sort(&mut numbers, 5);
        assert!(is_sorted(&numbers));
    }
}
