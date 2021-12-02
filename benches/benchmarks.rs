use arr_macro::arr;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::{
    distributions::{Distribution, Uniform},
    thread_rng,
};
use rust_sorting_algorithms::bubble_sort::BubbleSort;

pub fn benchmark_bubble_sort(c: &mut Criterion) {
    let mut rng = thread_rng();
    let dice = Uniform::from(0..=1000);

    let test_data: [u32; 250] = arr![dice.sample(&mut rng); 250];

    c.bench_function("bubble sort 250", |b| {
        b.iter(|| {
            test_data.clone().bubble_sort(black_box(
                rust_sorting_algorithms::SortingDirection::Ascending,
            ))
        })
    });
}

criterion_group!(benches, benchmark_bubble_sort);
criterion_main!(benches);
