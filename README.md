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
