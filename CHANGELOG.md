## [0.7.0] - 2025-08-13

### 🚀 Features

- Get trait bounds out of struct definition
- Add support for custom hashers

### 💼 Other

- Use a more modern ci config
- Try again to improve ci
- Default branch is `master` in this repo
- Add changelog generation pre-release hook

### ⚙️ Miscellaneous Tasks

- Fix `cargo clippy`

## [0.6.0] - 2024-06-30

### 💼 Other

- update edition, add `counter` keyword
- refactor tests and impls into distinct modules
- small doc formatting
- deprecate `init` method
- do not use deprecated `init` method
- With capacity
- Clippy fixes
- Implement Serialize and Deserialize

## [0.5.7] - 2022-10-12

### 💼 Other

- Implement is_subset and is_superset tests
- Implement bitwise and and or assignments
- Fix spelling error in doc header
- Multiset bag documentation

## [0.5.6] - 2022-07-16

### 💼 Other

- Relax more trait bounds
- Improve concision
- Fix minor issues in documentation
- Add method to return the `k` most common items and speed up `most_common_*()` methods

## [0.5.5] - 2022-05-05

### 💼 Other

- Add method `Counter::total()`
- Relax trait bounds

## [0.5.4] - 2022-04-02

### 💼 Other

- Relax trait bounds for `Default` implementation

## [0.5.3] - 2022-02-07

### 💼 Other

- `N: Clone` bound is not required on {Add,Sub}{,Assign}

## [0.5.2] - 2020-06-23

### impl `Extend`, `IntoIterator` for `Counter`

Implementing these traits gives users more ways to usefully combine their counts.

## [0.5.1] - 2020-05-19

### 💼 Other

- Remove unnecessary cloning

## [0.5.0] - 2020-05-19

### Implicit zero for unknown entries

Counters now implement `Index` and `IndexMut`, so they can have implicit zero counts. In other words:

```rust
let counter = "aaa".chars().collect::<Counter<_>>();
assert_eq!(counter[&'b'], 0);
// panics
// assert_eq!((*counter)[&'b'], 0);
```

This is a breaking change, causing a minor version bump, because it is not impossible that previous code depended on indexing panicing for unknown entries.
Code which does not panic as part of its intended control flow will not be affected.

## [0.4.3] - 2018-08-08

### Bound N on Clone, not Copy

All `Copy` types are also `Clone` types where the clone bound happens to be really cheap. Bounding `N: Clone` instead of `N: Copy` means that we can use numeric types like `num::BigInteger`, which are _not_ `Copy`, and things still work. You pay a bit more runtime cost, but if you're using a non-default counter type, presumably you know the costs of your actions.

## [0.1.0] - 2017-01-04
