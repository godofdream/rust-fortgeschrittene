#![feature(test)]

extern crate test;
use core::time;
use std::thread::sleep;

use test::Bencher;
use bench_example::fibonacci;

// run with cargo bench --bench fibonacci
// for getting the result in ns/iter

fn test_function() {
    sleep(time::Duration::from_millis(100));
}

#[bench]
fn bench_fibonacci(b: &mut Bencher) {
    b.iter(test_function);
}