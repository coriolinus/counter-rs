# Counter

Simple counter library for Rust iterables. Inspired by, and largely mimicing the API of, Python's [Counter](https://docs.python.org/3.5/library/collections.html#collections.Counter).

Too tired for proper documentation at the moment; see the source. It's not really that long.

## Examples

### Get the most common characters in a string, breaking ties alphabetically

`most_common_ordered()` uses the natural ordering of keys which are `Ord`.

```rust
let by_common = Counter::init("eaddbbccc".chars())
                  .most_common_ordered()
                  .collect::<Vec<_>>();
let expected = vec![('c', 3), ('b', 2), ('d', 2), ('a', 1), ('e', 1)];
assert!(by_common == expected);
```

### Get the most common characters in a string, using your own ordering

For example, here we break ties reverse alphabetically.

```rust
let counter = Counter::init("eaddbbccc".chars());
let by_common = counter.most_common_tiebreaker(|&a, &b| b.cmp(&a)).collect::<Vec<_>>();
let expected = vec![('c', 3), ('d', 2), ('b', 2), ('e', 1), ('a', 1)];
assert!(by_common == expected);
```

### Directly modify the backing map

This is backed by a map to which you have full access:

```rust
let mut counter = Counter::init("aa-bb-cc".chars());
counter.map.remove(&'-');
assert!(counter == Counter::init("aabbcc".chars()));
```

### Count any iterable which is `Hash + Eq`

You can't use the `most_common*` functions unless T is also `Clone`, but simple counting works fine on a minimal data type.

```rust
#[derive(Debug, Hash, PartialEq, Eq)]
struct Inty {
    i: usize,
}

impl Inty {
    pub fn new(i: usize) -> Inty {
        Inty { i: i }
    }
}

// <https://en.wikipedia.org/wiki/867-5309/Jenny>
let intys = vec![
    Inty::new(8),
    Inty::new(0),
    Inty::new(0),
    Inty::new(8),
    Inty::new(6),
    Inty::new(7),
    Inty::new(5),
    Inty::new(3),
    Inty::new(0),
    Inty::new(9),
];

let inty_counts = Counter::init(intys);
println!("{:?}", inty_counts.map); 
// {Inty { i: 8 }: 2, Inty { i: 0 }: 3, Inty { i: 9 }: 1, Inty { i: 3 }: 1,
//  Inty { i: 7 }: 1, Inty { i: 6 }: 1, Inty { i: 5 }: 1}
assert!(inty_counts.map.get(&Inty { i: 8 }) == Some(&2));
assert!(inty_counts.map.get(&Inty { i: 0 }) == Some(&3));
assert!(inty_counts.map.get(&Inty { i: 6 }) == Some(&1));
```
