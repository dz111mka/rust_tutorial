#![warn(clippy::all, clippy::pedantic)]

use std::cmp::Ordering;

fn main() {
    let arr1: [i32; 7] = [-2, -1, 5, 7, 9, 15, 19];
    let result = binary_search(&arr1, -2);
    match result {
        Some((found_value, index)) => println!("Найденное значение {} имеет индекс {}", found_value, index + 1),
        None => println!("Значение не найдено")
    }
}

fn binary_search(arr: &[i32], target: i32) -> Option<(i32, usize)> {
    if arr.is_empty() { return None }

    let mut left = 0;
    let mut right = arr.len();
    let mut step = 0;

    while left < right {
        let mid = usize::midpoint(left, right);

        match arr[mid].cmp(&target) {
            Ordering::Equal => {
                println!("Шаг: {}", step + 1);
                return Some((target, mid));
            }
            Ordering::Less => left = mid + 1,
            Ordering::Greater => right = mid,
        }

        step += 1;
        println!("Шаг: {step}");
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    const ARR1: [i32; 7] = [-2, -1, 5, 7, 9, 15, 19];

    #[test]
    fn element_found() {
        assert_eq!((-2, 0), binary_search(&ARR1, -2).unwrap());
    }

    #[test]
    fn element_not_found() {
        let result = binary_search(&ARR1, 123);
        assert_eq!(true, result.is_none());
    }

}