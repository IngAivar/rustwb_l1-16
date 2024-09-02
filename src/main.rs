fn binary_search(arr: &[i32], target: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len() - 1;

    while left <= right {
        let middle = left + (right - left) / 2; // Чтобы избежать переполнения при больших числах

        if arr[middle] == target {
            return Some(middle);
        } else if arr[middle] < target {
            left = middle + 1;
        } else {
            right = middle - 1;
        }
    }

    None
}

fn main() {
    let numbers = [2, 3, 4, 10, 40];
    let x = 10;

    match binary_search(&numbers, x) {
        Some(index) => println!("Элемент {} найден на позиции {}", x, index),
        None => println!("Элемент {} не найден", x),
    }
}