use rayon::prelude::*;

fn main() {
    let numbers: Vec<i32> = (1..=100_000).collect(); // A large vector of numbers

    let sum: i32 = numbers.par_iter()         // Parallel iterator
        .map(|&x| x * 2)                     // Operation on each element
        .sum();                              // Summing the results

    println!("The total sum is: {}", sum);
}
