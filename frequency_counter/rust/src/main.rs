use std::collections::HashMap;

fn count_frequency(arr: &[i32]) -> HashMap<i32, i32> {
    let mut freq = HashMap::new();

    for &value in arr {
        *freq.entry(value).or_insert(0) += 1;
    }

    freq
}

fn main() {
    let data = [1, 2, 2, 3, 3, 3, 4];

    let result = count_frequency(&data);

    for (key, value) in &result {
        println!("{} -> {}", key, value);
    }
}

