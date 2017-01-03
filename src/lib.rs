//! Counter based on the Python implementation of same:
//! <https://docs.python.org/3.5/library/collections.html#collections.Counter>
//!
//! Counts recurring elements from an iterable.

use std::collections::HashMap;
use std::hash::Hash;

use std::ops::{Add, Sub, BitAnd, BitOr};

#[derive(Clone)]
pub struct Counter<'a, T: 'a> {
    /// HashMap backing this Counter
    ///
    /// Public to expose the HashMap API for direct manipulation.
    pub hashmap: HashMap<&'a T, usize>,
}

impl<'a, T> Counter<'a, T>
    where T: 'a + Hash + Eq
{
    /// Create a new, empty `Counter`
    pub fn new() -> Counter<'a, T> {
        Counter { hashmap: HashMap::new() }
    }

    /// Create a new `Counter` initialized with the given iterable
    pub fn init<I>(iterable: I) -> Counter<'a, T>
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

impl<'a, T> Add for Counter<'a, T> {
    type Output = Counter<'a, T>;

    /// Add two counters together.
    ///
    /// `out[x] == c[x] + d[x]`
    fn add(self, rhs: Counter<'a, T>) -> Counter<'a, T> {
        unimplemented!()
    }
}

impl<'a, T> Sub for Counter<'a, T> {
    type Output = Counter<'a, T>;

    /// Subtract (keeping only positive values).
    ///
    /// `out[x] == c[x] - d[x]`
    fn sub(self, rhs: Counter<'a, T>) -> Counter<'a, T> {
        unimplemented!()
    }
}

impl<'a, T> BitAnd for Counter<'a, T> {
    type Output = Counter<'a, T>;

    /// Intersection
    ///
    /// `out[x] == min(c[x], d[x])`
    fn bitand(self, rhs: Counter<'a, T>) -> Counter<'a, T> {
        unimplemented!()
    }
}

impl<'a, T> BitOr for Counter<'a, T> {
    type Output = Counter<'a, T>;

    /// Union
    ///
    /// `out[x] == max(c[x], d[x])`
    fn bitor(self, rhs: Counter<'a, T>) -> Counter<'a, T> {
        unimplemented!()
    }
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
