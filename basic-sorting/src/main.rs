use std::collections::HashMap;

fn count_median(count: &mut Vec<i32>) -> Option<f64> {
    if count.is_empty() {
        return None;
    }

    count.sort();

    let length = count.len();
    if length % 2 == 1 {
        Some(count[length / 2] as f64)
    } else {
        let middle_right = length / 2;
        let middle_left = middle_right - 1;
        Some(count[middle_left] as f64 + count[middle_right] as f64 / 2.0)
    }
}

fn count_mode(count: &mut Vec<i32>) -> Option<i32> {
    if count.is_empty() {
        return None;
    }

    let mut mode_calc = HashMap::new();
    for &number in count {
        *mode_calc.entry(&number).or_insert(0) += 1;
    }

    let mut max_count = 0;
    let mut mode = None;

    for (&number, &count) in &mode_calc {
        if count > max_count {
            max_count = count;
            mode = Some(number);
        }
    }
    mode
}

fn main() {
    // let mut count: Vec<f64> = vec! {1, 2, 3, 4, 5, 6, 7, 8, 9, 10};
    let mut count = vec! {9, 3, 4, 29, 14, 20, 1, 23, 19, };

    println!("Listed numbers:{count:?}");
    println!("Median: {}", count_median);
    println!("Mode: {}", count_mode);
}