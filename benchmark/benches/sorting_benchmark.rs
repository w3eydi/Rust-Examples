use benchmark::sorting::*;
use criterion::{
    black_box,
    criterion_group,
    criterion_main,
    Criterion
};

fn selection_sorting_bench(c: &mut Criterion) {
    let mut array = black_box(
        [4, 5, 1, 2, 3, -1, -5, 9]
    );

    c.bench_function("selection sorting",
        |b| b.iter(|| selection_sort(&mut array))
    );
}

fn bubble_sorting_bench(c: &mut Criterion) {
    let mut array = black_box(
        [4, 5, 1, 2, 3, -1, -5, 9]
    );

    c.bench_function("bubble sorting",
        |b| b.iter(||bubble_sort(&mut array))    
    );
}

criterion_group!(benches, selection_sorting_bench, bubble_sorting_bench);
criterion_main!(benches);