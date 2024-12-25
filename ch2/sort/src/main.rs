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

fn is_sorted(arr: &[i32]) -> bool {
    for i in 0..arr.len() - 1 {
        if arr[i] > arr[i + 1] {
            return false;
        }
    }
    true
}

fn main() {
    let mut numbers = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];

    println!("ソート前: {:?}", numbers);
    
    insertion_sort(&mut numbers);
    
    println!("ソート後: {:?}", numbers);
    assert!(is_sorted(&numbers));
}
