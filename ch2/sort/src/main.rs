use rand::Rng;
use std::time::Instant;

// fn merge_with_buf(arr: &mut [i32], mid: usize, buf: &mut [i32]) {
//     let mut left_index = 0;
//     let mut right_index = mid;
//     let mut buf_index = 0;

//     while left_index < mid && right_index < arr.len() {
//         if arr[left_index] <= arr[right_index] {
//             buf[buf_index] = arr[left_index];
//             left_index += 1;
//         } else {
//             buf[buf_index] = arr[right_index];
//             right_index += 1;
//         }
//         buf_index += 1;
//     }
//     // 残りの要素をコピー
//     buf[buf_index..(buf_index + (mid - left_index))]
//         .copy_from_slice(&arr[left_index..mid]);
//     buf_index += mid - left_index;

//     buf[buf_index..(buf_index + (arr.len() - right_index))].copy_from_slice(&arr[right_index..]);
//     buf_index += arr.len() - right_index;

//     // buf の内容を arr に反映
//     arr.copy_from_slice(&buf[..buf_index]);
// }

// fn merge_sort_with_buf(arr: &mut [i32], buf: &mut [i32]) {
//     if arr.len() <= 1 {
//         return;
//     }
//     let mid = arr.len() / 2;
//     merge_sort_with_buf(&mut arr[..mid], &mut buf[..mid]);
//     merge_sort_with_buf(&mut arr[mid..], &mut buf[mid..]);
//     merge_with_buf(arr, mid, buf);
// }

// fn merge_sort_optiized(arr: &mut [i32]) {
//     let mut buf: Vec<i32> = vec![0; arr.len()];
//     merge_sort_with_buf(arr, &mut buf);
// }

/// 二分挿入ソート（安定ソート）
/// - 同じ値があった場合は「先に来たほう」が左に残るようにする
fn binary_insertion_sort(arr: &mut [i32]) {
    for i in 1..arr.len() {
        let x = arr[i];
        let mut left = 0;
        let mut right = i;

        // 同値の場合は「右を縮めずに左を広げる」ことで、先に来た方を左に残す
        while left < right {
            let mid = (left + right) / 2;
            if arr[mid] <= x {
                // xがarr[mid]以上の場合はmid+1以降に挿入
                left = mid + 1;
            } else {
                // xがarr[mid]未満の場合はmid含む範囲が候補
                right = mid;
            }
        }

        // [left..i) を後ろにシフトして、arr[left] に x を挿入
        for j in (left..i).rev() {
            arr[j + 1] = arr[j];
        }
        arr[left] = x;
    }
}

/// 安定マージ
/// - `mid` は `arr` の先頭から数えて左配列の長さ
/// - 結果は `arr` に上書き
/// - 同値の場合は「左側を先に採用」して安定性を保つ
fn stable_merge_with_buf(arr: &mut [i32], mid: usize, buf: &mut [i32]) {
    let mut left_index = 0;
    let mut right_index = mid;
    let mut buf_idx = 0;

    while left_index < mid && right_index < arr.len() {
        if arr[left_index] <= arr[right_index] {
            buf[buf_idx] = arr[left_index];
            left_index += 1;
        } else {
            buf[buf_idx] = arr[right_index];
            right_index += 1;
        }
        buf_idx += 1;
    }

    // 左側の残りをコピー
    if left_index < mid {
        buf[buf_idx..(buf_idx + (mid - left_index))].copy_from_slice(&arr[left_index..mid]);
        buf_idx += mid - left_index;
    }

    // 右側の残りをコピー
    if right_index < arr.len() {
        buf[buf_idx..(buf_idx + (arr.len() - right_index))].copy_from_slice(&arr[right_index..]);
        buf_idx += arr.len() - right_index;
    }

    // バッファから元の配列に反映
    arr.copy_from_slice(&buf[..buf_idx]);
}

/// ハイブリッド・ボトムアップ・マージソート（安定ソート）
/// - 小さな区間（threshold以下）では二分挿入ソート
/// - それ以上はボトムアップでマージソート
fn hybrid_bottom_up_merge_sort(arr: &mut [i32], threshold: usize) {
    let n = arr.len();
    if n <= 1 {
        return;
    }

    // 1. まず、要素数 threshold ごとに区切り、各区間を二分挿入ソート
    let mut start = 0;
    while start < n {
        let end = (start + threshold).min(n);
        binary_insertion_sort(&mut arr[start..end]);
        start += threshold;
    }

    // 2. バッファを一度だけ確保し、マージに使い回す
    let mut buf = vec![0; n];

    // 3. ボトムアップでマージしていく
    //    最初は width = threshold（前ステップで幅 threshold の区間は整列済み）
    //    以後、width を倍々に増やしながら隣接区間をマージする
    let mut width = threshold;
    while width < n {
        let mut i = 0;
        while i < n {
            let left = i;
            let mid = (i + width).min(n);
            let right = (i + 2 * width).min(n);

            // [left..mid], [mid..right] をマージ
            if mid < right {
                // マージ先のスライスを arr[left..right] とする
                stable_merge_with_buf(&mut arr[left..right], mid - left, &mut buf[left..right]);
            }
            i += 2 * width;
        }
        width *= 2;
    }
}

fn is_sorted(arr: &[i32]) -> bool {
    for i in 0..arr.len() - 1 {
        if arr[i] > arr[i + 1] {
            return false;
        }
    }
    true
}

fn main() {
    let mut rng = rand::thread_rng();
    let iterations = 1;

    // マージソートとハイブリッドソートを比較
    for i in 25..31 {
        let mut mysort_duration = 0;
        let mut standard_duration = 0;
        let mut unstable_duration = 0;
        let list_length = 2_i32.pow(i);
        for _ in 0..iterations {
            let original_numbers: Vec<i32> =
                (0..list_length).map(|_| rng.gen_range(0..1000000)).collect();

            let start = Instant::now();
            let mut numbers = original_numbers.clone();
            hybrid_bottom_up_merge_sort(&mut numbers, 16);
            let duration = start.elapsed();
            mysort_duration += duration.as_millis();
            assert!(is_sorted(&numbers));

            let start = Instant::now();
            let mut numbers = original_numbers.clone();
            // 標準機能のソート
            numbers.sort();
            let duration = start.elapsed();
            standard_duration += duration.as_millis();

            // 標準の不安定ソート
            let start = Instant::now();
            let mut numbers = original_numbers.clone();
            numbers.sort_unstable();
            let duration = start.elapsed();
            unstable_duration += duration.as_millis();
        }
        println!(
                "リストの長さ: {}, 改良後安定ハイブリッドソート時間: {}ms, 標準ソート時間: {}ms, 不安定ソート時間: {}ms",
                list_length,
                mysort_duration / iterations,
                standard_duration / iterations,
                unstable_duration / iterations
            );
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     fn is_sorted(arr: &[i32]) -> bool {
//         for i in 0..arr.len() - 1 {
//             if arr[i] > arr[i + 1] {
//                 return false;
//             }
//         }
//         true
//     }

//     #[test]
//     fn test_insertion_sort() {
//         let mut numbers = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
//         insertion_sort(&mut numbers);
//         assert!(is_sorted(&numbers));
//     }

//     #[test]
//     fn test_merge_sort() {
//         let mut numbers = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
//         merge_sort(&mut numbers);
//         assert!(is_sorted(&numbers));
//     }

//     #[test]
//     fn test_binary_insertion_sort() {
//         let mut numbers = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
//         binary_insertion_sort(&mut numbers);
//         assert!(is_sorted(&numbers));
//     }

//     #[test]
//     fn test_hybrid_sort() {
//         let mut numbers = vec![
//             3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 8, 9, 7, 9, 3, 2, 3, 8, 4, 6, 2,
//         ];
//         hybrid_sort(&mut numbers, 5);
//         assert!(is_sorted(&numbers));
//     }

//     #[test]
//     fn test_merge_sort_optimized() {
//         let mut numbers = vec![
//             3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 8, 9, 7, 9, 3, 2, 3, 8, 4, 6, 2,
//         ];
//         merge_sort_optiized(&mut numbers);
//         assert!(is_sorted(&numbers));
//     }
// }
