use crate::Counter;

use num_traits::Zero;

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
