use crate::Counter;

use num_traits::Zero;

use std::collections::HashMap;
use std::hash::{Hash, RandomState};

impl<T, N> Counter<T, N, RandomState>
where
    T: Hash + Eq,
    N: Zero,
{
    /// Create a new, empty `Counter` which uses the default hasher.
    ///
    /// Unlike [`Counter::default`], this does not require the hasher type to be named:
    /// it is always [`RandomState`], just as it is for [`HashMap::new`].
    ///
    /// ```rust
    /// # use counter::Counter;
    /// # use std::collections::HashMap;
    /// let mut counter = Counter::new();
    /// counter.update("aaa".chars());
    /// let expect = [('a', 3)].into_iter().collect::<HashMap<char, usize>>();
    /// assert_eq!(counter.into_map(), expect);
    /// ```
    ///
    /// To create a `Counter` with a non-default hasher, use [`with_hasher`].
    ///
    /// [`with_hasher`]: Counter::with_hasher
    pub fn new() -> Self {
        Counter {
            map: HashMap::default(),
            zero: N::zero(),
        }
    }

    /// Create a new, empty `Counter` with the specified capacity, which uses the default hasher.
    ///
    /// Note that `capacity` in this case indicates how many distinct items may be counted without reallocation.
    /// It is not related to the total number of items which may be counted.
    /// For example, `"aaa"` requires a capacity of 1. `"abc"` requires a capacity of 3.
    ///
    /// To create a `Counter` with a non-default hasher, use [`with_capacity_and_hasher`].
    ///
    /// [`with_capacity_and_hasher`]: Counter::with_capacity_and_hasher
    pub fn with_capacity(capacity: usize) -> Self {
        Counter {
            map: HashMap::with_capacity_and_hasher(capacity, RandomState::default()),
            zero: N::zero(),
        }
    }
}

impl<T, N, S> Counter<T, N, S>
where
    T: Hash + Eq,
    N: Zero,
{
    /// Create a new, empty `Counter` which will use the given hasher.
    ///
    /// ```rust
    /// # use counter::Counter;
    /// # use std::collections::HashMap;
    /// # use std::hash::RandomState;
    /// let mut counter = Counter::with_hasher(RandomState::new());
    /// counter.update("aaa".chars());
    /// let expect = [('a', 3)].into_iter().collect::<HashMap<char, usize>>();
    /// assert_eq!(counter.into_map(), expect);
    /// ```
    pub fn with_hasher(hash_builder: S) -> Self {
        Counter {
            map: HashMap::with_hasher(hash_builder),
            zero: N::zero(),
        }
    }

    /// Create a new, empty `Counter` with the specified capacity, which will use the given hasher.
    ///
    /// Note that `capacity` in this case indicates how many distinct items may be counted without reallocation.
    /// It is not related to the total number of items which may be counted.
    /// For example, `"aaa"` requires a capacity of 1. `"abc"` requires a capacity of 3.
    pub fn with_capacity_and_hasher(capacity: usize, hash_builder: S) -> Self {
        Counter {
            map: HashMap::with_capacity_and_hasher(capacity, hash_builder),
            zero: N::zero(),
        }
    }
}

impl<T, N, S> Default for Counter<T, N, S>
where
    N: Default,
    S: Default,
{
    fn default() -> Self {
        Self {
            map: HashMap::default(),
            zero: N::default(),
        }
    }
}
