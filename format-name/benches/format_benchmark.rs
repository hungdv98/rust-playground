use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

fn format_name_original(name: &str) -> String {
    name.split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => {
                    let head = first.to_uppercase().collect::<String>();
                    let tail = chars.as_str().to_lowercase();
                    head + &tail
                }
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}

fn format_name_optimized(name: &str) -> String {
    let mut result = String::with_capacity(name.len());
    let mut words = name.split_whitespace().peekable();

    while let Some(word) = words.next() {
        let mut chars = word.chars();
        if let Some(first) = chars.next() {
            result.extend(first.to_uppercase());
            for c in chars {
                result.push(c.to_lowercase().next().unwrap());
            }
        }
        if words.peek().is_some() {
            result.push(' ');
        }
    }
    result
}

fn criterion_benchmark(c: &mut Criterion) {
    let base_input =
        "nguyen van a mang tinh chat minh hoa cho mot chuoi rat dai de test hieu nang ";
    let large_input = base_input.repeat(1000);

    let mut group = c.benchmark_group("Large_Scale_Naming");

    group.sample_size(100);

    group.bench_function("Original_Vec_Join_Large", |b| {
        b.iter(|| format_name_original(black_box(&large_input)))
    });

    group.bench_function("Optimized_Single_Alloc_Large", |b| {
        b.iter(|| format_name_optimized(black_box(&large_input)))
    });

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
