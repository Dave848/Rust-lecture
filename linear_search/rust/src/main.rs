fn linear_search(arr: &[i32], target: i32) -> Option<usize> {
    for (i, &value) in arr.iter().enumerate() {
        if value == target {
            return Some(i);
        }
    }
    None
}

fn main() {
    let data = [10, 20, 30, 40];

    match linear_search(&data, 30) {
        Some(i) => println!("Found at index {}", i),
        None => println!("Not found"),
    }
}