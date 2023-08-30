use counter::Counter;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

#[allow(unused)]
#[inline]
fn just_count_iterable(black_box: usize) {
    let char_counts = "barefoot".chars().collect::<Counter<_>>();
    let counts_counts = char_counts.values().collect::<Counter<_>>();
}

#[allow(unused)]
#[inline]
fn update_a_count(black_box: usize) {
    let mut counts = "aaa".chars().collect::<Counter<_>>();
    counts[&'a'] += 1;
    counts[&'b'] += 1;
}

#[allow(unused)]
#[inline]
fn update_a_count_2(black_box: usize) {
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
