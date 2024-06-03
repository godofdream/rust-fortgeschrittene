use criterion::{black_box, criterion_group, criterion_main, Criterion};
use criterion_example::{fibonacci, iterative_fibonacci, recursive_fibonacci};


#[allow(clippy::unit_arg)]
fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fibonacci", |b| {
        b.iter(|| {
            black_box({
                let _result = fibonacci(10);
            }); 
        });
    });

    c.bench_function("iterative_fibonacci", |b| {
        b.iter(|| {
            black_box({
                let _result = iterative_fibonacci(10);
            }); 
        });
    });

    c.bench_function("recursive_fibonacci", |b| {
        b.iter(|| {
            black_box({
                let _result = recursive_fibonacci(10);
            }); 
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
