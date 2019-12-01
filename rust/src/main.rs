extern crate floating_duration;

use std::time::{Instant};
use floating_duration::TimeAsFloat;

fn main() {
    let start = Instant::now();
    let f = fibonacci(50);
    let duration = start.elapsed().as_fractional_secs();
    println!("{:.2} s", duration); 
    println!("{}", f); // dirty hack to force fibonacci function to be used
}

fn fibonacci(n: u32) -> u32 {
    if n <= 1 {
        return 1
    }
    return fibonacci(n-1) + fibonacci(n-2);
}
