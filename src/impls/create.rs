use crate::Counter;

use num_traits::{One, Zero};

use std::ops::AddAssign;
use std::collections::HashMap;
use std::hash::Hash;

impl<T, N> Counter<T, N>
where
    T: Hash + Eq,
    N: Zero,
{
    /// Create a new, empty `Counter`
    pub fn new() -> Self {
        Counter {
            map: HashMap::new(),
            zero: N::zero(),
        }
    }
}

impl<T, N> Counter<T, N>
where
    T: Hash + Eq,
    N: AddAssign + Zero + One,
{
    /// Create a new `Counter` initialized with the given iterable.
    pub fn init<I>(iterable: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        let mut counter = Counter::new();
        counter.update(iterable);
        counter
    }
}


impl<T, N> Default for Counter<T, N>
where
    T: Hash + Eq,
    N: Default,
{
    fn default() -> Self {
        Self {
            map: Default::default(),
            zero: Default::default(),
        }
    }
}
