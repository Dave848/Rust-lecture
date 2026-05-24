use std::time::Instant;


// To run this benchmark, use the following command in the terminal:
// cargo run --release
fn main() {
    let n = 1000;

    let a = vec![vec![1.0f64; n]; n];
    let b = vec![vec![2.0f64; n]; n];
    let mut c = vec![vec![0.0f64; n]; n];

    let start = Instant::now();

    for i in 0..n {
        for j in 0..n {
            let mut sum = 0.0;
            for k in 0..n {
                sum += a[i][k] * b[k][j];
            }
            c[i][j] = sum;
        }
    }

    let duration = start.elapsed();
    println!("Rust matrix multiply: {:?}", duration);
    println!("checksum: {}", c[0][0]);
}