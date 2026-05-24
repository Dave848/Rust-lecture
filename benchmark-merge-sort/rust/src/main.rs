use std::time::Instant;
use rand::Rng;
use std::thread;

// merge function
fn merge(arr: &mut [i32], mid: usize) {
    let temp = arr.to_vec();
    let (left, right) = temp.split_at(mid);

    let mut i = 0;
    let mut j = 0;
    let mut k = 0;

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            arr[k] = left[i];
            i += 1;
        } else {
            arr[k] = right[j];
            j += 1;
        }
        k += 1;
    }

    while i < left.len() {
        arr[k] = left[i];
        i += 1;
        k += 1;
    }

    while j < right.len() {
        arr[k] = right[j];
        j += 1;
        k += 1;
    }
}

fn merge_sort(arr: &mut [i32], depth: usize) {
    let len = arr.len();
    if len <= 1 {
        return;
    }

    let mid = len / 2;

    if depth < 4 {
        let (left, right) = arr.split_at_mut(mid);

        thread::scope(|s| {
            s.spawn(|| merge_sort(left, depth + 1));
            s.spawn(|| merge_sort(right, depth + 1));
        });
    } else {
        let (left, right) = arr.split_at_mut(mid);
        merge_sort(left, depth + 1);
        merge_sort(right, depth + 1);
    }

    merge(arr, mid);
}

fn main() {
    let n = 10_000_000;

    let mut rng = rand::thread_rng();
    let mut data: Vec<i32> = (0..n)
        .map(|_| rng.gen_range(0..n))
        .collect();

    let start = Instant::now();

    merge_sort(&mut data, 0);

    let duration = start.elapsed();
    println!("Rust merge sort: {:?}", duration);
}