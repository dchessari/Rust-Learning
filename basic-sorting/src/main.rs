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
        Some((count[middle_left] as f64 + count[middle_right] as f64) / 2.0)
    }
}

fn count_mode(count: &mut Vec<i32>) -> Option<i32> {
    if count.is_empty() {
        return None;
    }

    let mut mode_calc = HashMap::new();

    for &num in count.iter() {
        *mode_calc.entry(num).or_insert(0) += 1;
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
    let mut count = vec! {9, 3, 4, 29, 14, 20, 1, 23, 19, 3};

    let median = count_median(&mut count);
    let mode = count_mode(&mut count);

    println!("Listed numbers: {count:?}");

    match median {
        Some(value) => println!("Median: {value}"),
        None => println!("No median found"),
    }

    match mode {
        Some(value) => println!("Mode: {value}"),
        None => println!("No mode found"),
    }
}