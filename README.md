# Counter

Counter counts recurrent elements of iterables. It is based on [the Python implementation](https://docs.python.org/3.5/library/collections.html#collections.Counter).

## Examples

### Just count an iterable

```rust
use counter::Counter;
let char_counts = "barefoot".chars().collect::<Counter<_>>();
let counts_counts = char_counts.values().collect::<Counter<_>>();
```

### Update a count

```rust
let mut counts = "able babble table babble rabble table able fable scrabble"
    .split_whitespace().collect::<Counter<_>>();
// add or subtract an iterable of the same type
counts += "cain and abel fable table cable".split_whitespace();
// or add or subtract from another Counter of the same type
let other_counts = "scrabble cabbie fable babble"
    .split_whitespace().collect::<Counter<_>>();
let difference = counts - other_counts;
```

### Get the most common items

`most_common_ordered()` uses the natural ordering of keys which are `Ord`.

```rust
let by_common = "eaddbbccc".chars().collect::<Counter<_>>().most_common_ordered();
let expected = vec![('c', 3), ('b', 2), ('d', 2), ('a', 1), ('e', 1)];
assert!(by_common == expected);
```

### Get the most common items using your own ordering

For example, here we break ties reverse alphabetically.

```rust
let counter = "eaddbbccc".chars().collect::<Counter<_>>();
let by_common = counter.most_common_tiebreaker(|&a, &b| b.cmp(&a));
let expected = vec![('c', 3), ('d', 2), ('b', 2), ('e', 1), ('a', 1)];
assert!(by_common == expected);
```

### Treat it like a Map

`Counter<T, N>` implements `Deref<Target=HashMap<T, N>>` and
`DerefMut<Target=HashMap<T, N>>`, which means that you can perform any operations
on it which are valid for a [`HashMap`](https://doc.rust-lang.org/std/collections/struct.HashMap.html).

```rust
let mut counter = "aa-bb-cc".chars().collect::<Counter<_>>();
counter.remove(&'-');
assert!(counter == "aabbcc".chars().collect::<Counter<_>>());
```

## Advanced Usage

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

let inty_counts = intys.iter().collect::<Counter<_>>();
println!("{:?}", inty_counts);
// {Inty { i: 8 }: 2, Inty { i: 0 }: 3, Inty { i: 9 }: 1, Inty { i: 3 }: 1,
//  Inty { i: 7 }: 1, Inty { i: 6 }: 1, Inty { i: 5 }: 1}
assert!(inty_counts.get(&Inty { i: 8 }) == Some(&2));
assert!(inty_counts.get(&Inty { i: 0 }) == Some(&3));
assert!(inty_counts.get(&Inty { i: 6 }) == Some(&1));
```

### Use your own type for the count

Sometimes `usize` just isn't enough. If you find yourself overflowing your
machine's native size, you can use your own type. Here, we use an `i8`, but
you can use most numeric types, including bignums, as necessary.

```rust
let counter: Counter<_, i8> = "abbccc".chars().collect();
let expected: HashMap<char, i8> = [('a', 1), ('b', 2), ('c', 3)].iter().cloned().collect();
assert!(counter.into_map() == expected);
```
