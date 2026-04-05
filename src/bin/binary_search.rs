fn main() {
    let arr1: [i32; 7] = [-2, -1, 5, 7, 9, 15, 19];
    let result = binary_search(&arr1, 19);
    match result {
        Some((found_value, index)) => println!("Найденное значение {} имеет индекс {}", found_value, index + 1),
        None => {println!("Значение не найдено");}
    };
}

fn binary_search(arr: &[i32], target: i32) -> Option<(i32, usize)> {
    if arr.is_empty() { return None }

    let mut left = 0;
    let mut right = arr.len();
    let mut step = 0;

    while left < right {
        let mid = (left + right) / 2;

        if arr[mid] == target {
            step += 1;
            println!("Шаг: {step}");
            return Some((target, mid));
        } else if arr[mid] < target {
            left = mid + 1;
        } else {
            right = mid;
        }

        step += 1;
        println!("Шаг: {step}")
    }

    None
}