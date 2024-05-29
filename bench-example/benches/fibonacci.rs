#![feature(test)]

extern crate test;
use test::Bencher;

// run with cargo bench --bench fibonacci
// for getting the result in ns/iter


fn fibonacci(n: u64) -> u64 {
    // println!("Calculating fibonacci({})", n); for getting the output don't forget "--nocapture"
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

#[bench]
fn bench_fibonacci(b: &mut Bencher) {
    b.iter(|| fibonacci(20));
}