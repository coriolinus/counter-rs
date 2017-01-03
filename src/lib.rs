//! Counter based on the Python implementation of same:
//! <https://docs.python.org/3.5/library/collections.html#collections.Counter>
//!
//! Counts recurring elements from an iterable.

use std::collections::HashMap;
use std::hash::Hash;

use std::ops::{Add, Sub, BitAnd, BitOr};

#[derive(Clone)]
pub struct Counter<T> {
    /// HashMap backing this Counter
    ///
    /// Public to expose the HashMap API for direct manipulation.
    pub hashmap: HashMap<T, usize>,
}

impl<'a, T> Counter<T>
    where T: 'a + Hash + Eq
{
    /// Create a new, empty `Counter`
    pub fn new() -> Counter<T> {
        Counter { hashmap: HashMap::new() }
    }

    /// Create a new `Counter` initialized with the given iterable
    pub fn init<I>(iterable: I) -> Counter<T>
        where I: IntoIterator<Item = &'a T>
    {
        let mut counter = Counter::new();
        counter.update(iterable);
        counter
    }

    /// Add the counts of the elements from the given iterable to this counter
    pub fn update<I>(&mut self, iterable: I)
        where I: IntoIterator<Item = &'a T>
    {
        unimplemented!()
    }

    /// Remove the counts of the elements from the given iterable to this counter
    pub fn subtract<I>(&mut self, iterable: I)
        where I: IntoIterator<Item = &'a T>
    {
        unimplemented!()
    }

    /// Create an iterator over `(frequency, elem)` pairs, sorted most to least common.
    /// TODO: create an actual iterator, not a vector
    pub fn most_common(&self) -> Vec<(usize, T)> {
        unimplemented!()
    }
}

impl<T> Add for Counter<T> {
    type Output = Counter<T>;

    /// Add two counters together.
    ///
    /// `out[x] == c[x] + d[x]`
    fn add(self, rhs: Counter<T>) -> Counter<T> {
        unimplemented!()
    }
}

impl<T> Sub for Counter<T> {
    type Output = Counter<T>;

    /// Subtract (keeping only positive values).
    ///
    /// `out[x] == c[x] - d[x]`
    fn sub(self, rhs: Counter<T>) -> Counter<T> {
        unimplemented!()
    }
}

impl<T> BitAnd for Counter<T> {
    type Output = Counter<T>;

    /// Intersection
    ///
    /// `out[x] == min(c[x], d[x])`
    fn bitand(self, rhs: Counter<T>) -> Counter<T> {
        unimplemented!()
    }
}

impl<T> BitOr for Counter<T> {
    type Output = Counter<T>;

    /// Union
    ///
    /// `out[x] == max(c[x], d[x])`
    fn bitor(self, rhs: Counter<T>) -> Counter<T> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
