mod calculate;

fn main() {
    println!("Hello, world!");

    let numbers: Vec<i32> = vec![1, 2, 2, 3, 4, 5, 5, 5, 6, 7];
    let largest: i32 = calculate::calculate_largest(&numbers);
    let mean: f64 = calculate::calculate_mean(&numbers);
    let median: f64 = calculate::calculate_median(&numbers);
    let mode: i32 = calculate::calculate_mode(&numbers);

    println!("Largest: {}", largest);
    println!("Mean: {}", mean);
    println!("Median: {}", median);
    println!("Mode: {}", mode);
}
