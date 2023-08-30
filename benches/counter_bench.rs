use counter::Counter;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

#[inline]
fn generate_alphabet_string(length: usize) -> String {
    let alphabets_64 = "abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijkl";
    let quotient = length / 64;
    let remainder = length % 64;

    let mut result = String::with_capacity(length);

    for _ in 0..quotient {
        result.push_str(alphabets_64);
    }
    result.push_str(&alphabets_64[0..remainder]);

    result
}

#[allow(unused)]
#[inline]
fn create_from_length(test_string_len: usize) {
    let test_string = generate_alphabet_string(test_string_len);
    let counter_from_iter: Counter<char, usize> = Counter::init(test_string.chars());
}

#[allow(unused)]
#[inline]
fn update_a_count(black_box: usize) {
    let mut counts = "able babble table babble rabble table able fable scrabble"
        .split_whitespace()
        .collect::<Counter<_>>();
    counts += "cain and abel fable table cable".split_whitespace();
    let other_counts = "scrabble cabbie fable babble"
        .split_whitespace()
        .collect::<Counter<_>>();
    let difference = counts - other_counts;
}

#[allow(unused)]
#[inline]
fn get_most_common_items(black_box: usize) {
    let by_common = "eaddbbccc"
        .chars()
        .collect::<Counter<_>>()
        .most_common_ordered();
    let expected = vec![('c', 3), ('b', 2), ('d', 2), ('a', 1), ('e', 1)];
    assert!(by_common == expected);
}

#[allow(unused)]
#[inline]
fn get_k_most_common(black_box: usize) {
    let by_common = "eaddbbccc"
        .chars()
        .collect::<Counter<_>>()
        .k_most_common_ordered(2);
    let expected = vec![('c', 3), ('b', 2)];
    assert!(by_common == expected);
}

#[allow(unused)]
#[inline]
fn most_common_tiebreaker_benched(black_box: usize) {
    let counter = "eaddbbccc".chars().collect::<Counter<_>>();
    let by_common = counter.most_common_tiebreaker(|&a, &b| b.cmp(&a));
    let expected = vec![('c', 3), ('d', 2), ('b', 2), ('e', 1), ('a', 1)];
    assert_eq!(by_common, expected);
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("create from 1K length", |b| {
        b.iter(|| create_from_length(black_box(1 << 10)))
    });
    c.bench_function("create from 1M length", |b| {
        b.iter(|| create_from_length(black_box(1 << 20)))
    });

    c.bench_function("just count an iterable", |b| {
        b.iter(|| get_most_common_items(black_box(20)))
    });
    c.bench_function("get_k_most_common", |b| {
        b.iter(|| get_k_most_common(black_box(20)))
    });
    c.bench_function("most_common_tiebreaker_benched", |b| {
        b.iter(|| most_common_tiebreaker_benched(black_box(20)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);