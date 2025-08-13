use crate::Counter;

use num_traits::Zero;

use std::collections::HashMap;
use std::hash::Hash;

impl<T, N, S> Counter<T, N, S>
where
    T: Hash + Eq,
    N: Zero,
    S: Default,
{
    /// Create a new, empty `Counter`
    pub fn new() -> Self {
        Counter {
            map: HashMap::<T, N, S>::default(),
            zero: N::zero(),
        }
    }

    /// Create a new, empty `Counter` with the specified capacity.
    ///
    /// Note that `capacity` in this case indicates how many distinct items may be counted without reallocation.
    /// It is not related to the total number of items which may be counted.
    /// For example, `"aaa"` requires a capacity of 1. `"abc"` requires a capacity of 3.
    pub fn with_capacity(capacity: usize) -> Self {
        Counter {
            map: HashMap::with_capacity_and_hasher(capacity, S::default()),
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
