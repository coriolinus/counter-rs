//! Counter counts recurrent elements of iterables. It is based on [the Python implementation](https://docs.python.org/3.5/library/collections.html#collections.Counter).
//!
//! The struct [`Counter`](struct.Counter.html) is the entry-point type for this module.
//!
//! # Examples
//!
//! ## Just count an iterable
//!
//! ```rust
//! use counter::Counter;
//! let char_counts = "barefoot".chars().collect::<Counter<_>>();
//! let counts_counts = char_counts.values().collect::<Counter<_>>();
//! ```
//!
//! ## Update a count
//!
//! ```rust
//! # use counter::Counter;
//! let mut counts = "aaa".chars().collect::<Counter<_>>();
//! counts[&'a'] += 1;
//! counts[&'b'] += 1;
//! ```
//!
//! ```rust
//! # use counter::Counter;
//! let mut counts = "able babble table babble rabble table able fable scrabble"
//!     .split_whitespace().collect::<Counter<_>>();
//! // add or subtract an iterable of the same type
//! counts += "cain and abel fable table cable".split_whitespace();
//! // or add or subtract from another Counter of the same type
//! let other_counts = "scrabble cabbie fable babble"
//!     .split_whitespace().collect::<Counter<_>>();
//! let difference = counts - other_counts;
//! ```
//!
//! ## Get items with keys
//!
//! ```rust
//! # use counter::Counter;
//! let counts = "aaa".chars().collect::<Counter<_>>();
//! assert_eq!(counts[&'a'], 3);
//! assert_eq!(counts[&'b'], 0);
//! ```
//!
//! ## Get the most common items
//!
//! `most_common_ordered()` uses the natural ordering of keys which are `Ord`.
//!
//! ```rust
//! # use counter::Counter;
//! let by_common = "eaddbbccc".chars().collect::<Counter<_>>().most_common_ordered();
//! let expected = vec![('c', 3), ('b', 2), ('d', 2), ('a', 1), ('e', 1)];
//! assert!(by_common == expected);
//! ```
//!
//! ## Get the most common items using your own ordering
//!
//! For example, here we break ties reverse alphabetically.
//!
//! ```rust
//! # use counter::Counter;
//! let counter = "eaddbbccc".chars().collect::<Counter<_>>();
//! let by_common = counter.most_common_tiebreaker(|&a, &b| b.cmp(&a));
//! let expected = vec![('c', 3), ('d', 2), ('b', 2), ('e', 1), ('a', 1)];
//! assert!(by_common == expected);
//! ```
//!
//! ## Treat it like a Map
//!
//! `Counter<T, N>` implements `Deref<Target=HashMap<T, N>>` and
//! `DerefMut<Target=HashMap<T, N>>`, which means that you can perform any operations
//! on it which are valid for a [`HashMap`](https://doc.rust-lang.org/std/collections/struct.HashMap.html).
//!
//! ```rust
//! # use counter::Counter;
//! let mut counter = "aa-bb-cc".chars().collect::<Counter<_>>();
//! counter.remove(&'-');
//! assert!(counter == "aabbcc".chars().collect::<Counter<_>>());
//! ```
//!
//! Note that `Counter<T, N>` itself implements `Index`. `Counter::index` returns a reference to a `zero` value for missing keys.
//!
//! ```rust
//! # use counter::Counter;
//! let counter = "aaa".chars().collect::<Counter<_>>();
//! assert_eq!(counter[&'b'], 0);
//! // panics
//! // assert_eq!((*counter)[&'b'], 0);
//! ```
//!
//! # Advanced Usage
//!
//! ## Count any iterable which is `Hash + Eq`
//!
//! You can't use the `most_common*` functions unless T is also `Clone`, but simple counting works fine on a minimal data type.
//!
//! ```rust
//! # use counter::Counter;
//! #[derive(Debug, Hash, PartialEq, Eq)]
//! struct Inty {
//!     i: usize,
//! }
//!
//! impl Inty {
//!     pub fn new(i: usize) -> Inty {
//!         Inty { i: i }
//!     }
//! }
//!
//! // <https://en.wikipedia.org/wiki/867-5309/Jenny>
//! let intys = vec![
//!     Inty::new(8),
//!     Inty::new(0),
//!     Inty::new(0),
//!     Inty::new(8),
//!     Inty::new(6),
//!     Inty::new(7),
//!     Inty::new(5),
//!     Inty::new(3),
//!     Inty::new(0),
//!     Inty::new(9),
//! ];
//!
//! let inty_counts = intys.iter().collect::<Counter<_>>();
//! println!("{:?}", inty_counts);
//! // {Inty { i: 8 }: 2, Inty { i: 0 }: 3, Inty { i: 9 }: 1, Inty { i: 3 }: 1,
//! //  Inty { i: 7 }: 1, Inty { i: 6 }: 1, Inty { i: 5 }: 1}
//! assert!(inty_counts.get(&Inty { i: 8 }) == Some(&2));
//! assert!(inty_counts.get(&Inty { i: 0 }) == Some(&3));
//! assert!(inty_counts.get(&Inty { i: 6 }) == Some(&1));
//! ```
//!
//! ## Use your own type for the count
//!
//! Sometimes `usize` just isn't enough. If you find yourself overflowing your
//! machine's native size, you can use your own type. Here, we use an `i8`, but
//! you can use most numeric types, including bignums, as necessary.
//!
//! ```rust
//! # use counter::Counter;
//! # use std::collections::HashMap;
//! let counter: Counter<_, i8> = "abbccc".chars().collect();
//! let expected: HashMap<char, i8> = [('a', 1), ('b', 2), ('c', 3)].iter().cloned().collect();
//! assert!(counter.into_map() == expected);
//! ```

#[cfg(test)]
#[macro_use]
extern crate maplit;

extern crate num_traits;
use num_traits::{One, Zero};

use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;
use std::iter;
use std::ops::{Add, AddAssign, BitAnd, BitOr, Deref, DerefMut, Index, IndexMut, Sub, SubAssign};

type CounterMap<T, N> = HashMap<T, N>;

#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct Counter<T: Hash + Eq, N = usize> {
    map: CounterMap<T, N>,
    // necessary for `Index::index` since we cannot declare generic `static` variables.
    zero: N,
}

impl<T, N> Counter<T, N>
where
    T: Hash + Eq,
    N: PartialOrd + AddAssign + SubAssign + Zero + One,
{
    /// Create a new, empty `Counter`
    pub fn new() -> Counter<T, N> {
        Counter {
            map: HashMap::new(),
            zero: N::zero(),
        }
    }

    /// Create a new `Counter` initialized with the given iterable
    pub fn init<I>(iterable: I) -> Counter<T, N>
    where
        I: IntoIterator<Item = T>,
    {
        let mut counter = Counter::new();
        counter.update(iterable);
        counter
    }

    /// Add the counts of the elements from the given iterable to this counter
    pub fn update<I>(&mut self, iterable: I)
    where
        I: IntoIterator<Item = T>,
    {
        for item in iterable.into_iter() {
            let entry = self.map.entry(item).or_insert(N::zero());
            *entry += N::one();
        }
    }

    /// Consumes this counter and returns a HashMap mapping the items to the counts.
    pub fn into_map(self) -> HashMap<T, N> {
        self.map
    }

    /// Remove the counts of the elements from the given iterable to this counter
    ///
    /// Non-positive counts are automatically removed
    ///
    /// ```rust
    /// # use counter::Counter;
    /// # use std::collections::HashMap;
    /// let mut counter = "abbccc".chars().collect::<Counter<_>>();
    /// counter.subtract("abba".chars());
    /// let expect = [('c', 3)].iter().cloned().collect::<HashMap<_, _>>();
    /// assert_eq!(counter.into_map(), expect);
    /// ```
    pub fn subtract<I>(&mut self, iterable: I)
    where
        I: IntoIterator<Item = T>,
    {
        for item in iterable.into_iter() {
            let mut remove = false;
            if let Some(entry) = self.map.get_mut(&item) {
                if *entry > N::zero() {
                    *entry -= N::one();
                }
                remove = *entry == N::zero();
            }
            if remove {
                self.map.remove(&item);
            }
        }
    }
}

impl<T, N> Counter<T, N>
where
    T: Hash + Eq + Clone,
    N: Clone + Ord,
{
    /// Create a vector of `(elem, frequency)` pairs, sorted most to least common.
    ///
    /// ```rust
    /// # use counter::Counter;
    /// let mc = "pappaopolo".chars().collect::<Counter<_>>().most_common();
    /// let expected = vec![('p', 4), ('o', 3), ('a', 2), ('l', 1)];
    /// assert_eq!(mc, expected);
    /// ```
    ///
    /// Note that the ordering of duplicates is unstable.
    pub fn most_common(&self) -> Vec<(T, N)> {
        use std::cmp::Ordering;
        self.most_common_tiebreaker(|ref _a, ref _b| Ordering::Equal)
    }

    /// Create a vector of `(elem, frequency)` pairs, sorted most to least common.
    ///
    /// In the event that two keys have an equal frequency, use the supplied ordering function
    /// to further arrange the results.
    ///
    /// For example, we can sort reverse-alphabetically:
    ///
    /// ```rust
    /// # use counter::Counter;
    /// let counter = "eaddbbccc".chars().collect::<Counter<_>>();
    /// let by_common = counter.most_common_tiebreaker(|&a, &b| b.cmp(&a));
    /// let expected = vec![('c', 3), ('d', 2), ('b', 2), ('e', 1), ('a', 1)];
    /// assert_eq!(by_common, expected);
    /// ```
    pub fn most_common_tiebreaker<F>(&self, tiebreaker: F) -> Vec<(T, N)>
    where
        F: Fn(&T, &T) -> ::std::cmp::Ordering,
    {
        use std::cmp::Ordering;

        let mut items = self
            .map
            .iter()
            .map(|(key, count)| (key.clone(), count.clone()))
            .collect::<Vec<_>>();
        items.sort_by(|&(ref a_item, ref a_count), &(ref b_item, ref b_count)| {
            match b_count.cmp(&a_count) {
                Ordering::Equal => tiebreaker(&a_item, &b_item),
                unequal @ _ => unequal,
            }
        });
        items
    }
}

impl<T, N> Counter<T, N>
where
    T: Hash + Eq + Clone + Ord,
    N: Clone + Ord,
{
    /// Create a vector of `(elem, frequency)` pairs, sorted most to least common.
    ///
    /// In the event that two keys have an equal frequency, use the natural ordering of the keys
    /// to further sort the results.
    ///
    /// ```rust
    /// # use counter::Counter;
    /// let mc = "abracadabra".chars().collect::<Counter<_>>().most_common_ordered();
    /// let expect = vec![('a', 5), ('b', 2), ('r', 2), ('c', 1), ('d', 1)];
    /// assert_eq!(mc, expect);
    /// ```
    pub fn most_common_ordered(&self) -> Vec<(T, N)> {
        self.most_common_tiebreaker(|ref a, ref b| a.cmp(&b))
    }
}

impl<T, N> AddAssign for Counter<T, N>
where
    T: Clone + Hash + Eq,
    N: Clone + Zero + AddAssign,
{
    /// Add another counter to this counter
    ///
    /// `c += d;` -> `c[x] += d[x]` for all `x`
    ///
    /// ```rust
    /// # use counter::Counter;
    /// # use std::collections::HashMap;
    /// let mut c = "aaab".chars().collect::<Counter<_>>();
    /// let d = "abb".chars().collect::<Counter<_>>();
    ///
    /// c += d;
    ///
    /// let expect = [('a', 4), ('b', 3)].iter().cloned().collect::<HashMap<_, _>>();
    /// assert_eq!(c.into_map(), expect);
    /// ```
    fn add_assign(&mut self, rhs: Self) {
        for (key, value) in rhs.map.iter() {
            let entry = self.map.entry(key.clone()).or_insert(N::zero());
            *entry += value.clone();
        }
    }
}

impl<T, N> Add for Counter<T, N>
where
    T: Clone + Hash + Eq,
    N: Clone + PartialOrd + PartialEq + AddAssign + Zero,
{
    type Output = Counter<T, N>;

    /// Add two counters together.
    ///
    /// `out = c + d;` -> `out[x] == c[x] + d[x]` for all `x`
    ///
    /// ```rust
    /// # use counter::Counter;
    /// # use std::collections::HashMap;
    /// let c = "aaab".chars().collect::<Counter<_>>();
    /// let d = "abb".chars().collect::<Counter<_>>();
    ///
    /// let e = c + d;
    ///
    /// let expect = [('a', 4), ('b', 3)].iter().cloned().collect::<HashMap<_, _>>();
    /// assert_eq!(e.into_map(), expect);
    /// ```
    fn add(self, rhs: Counter<T, N>) -> Self::Output {
        let mut counter = self.clone();
        counter += rhs;
        counter
    }
}

impl<T, N> SubAssign for Counter<T, N>
where
    T: Hash + Eq,
    N: Clone + PartialOrd + PartialEq + SubAssign + Zero,
{
    /// Subtract (keeping only positive values).
    ///
    /// `c -= d;` -> `c[x] -= d[x]` for all `x`,
    /// keeping only items with a value greater than N::zero().
    ///
    /// ```rust
    /// # use counter::Counter;
    /// # use std::collections::HashMap;
    /// let mut c = "aaab".chars().collect::<Counter<_>>();
    /// let d = "abb".chars().collect::<Counter<_>>();
    ///
    /// c -= d;
    ///
    /// let expect = [('a', 2)].iter().cloned().collect::<HashMap<_, _>>();
    /// assert_eq!(c.into_map(), expect);
    /// ```
    fn sub_assign(&mut self, rhs: Self) {
        for (key, value) in rhs.map.iter() {
            let mut remove = false;
            if let Some(entry) = self.map.get_mut(key) {
                if *entry >= *value {
                    *entry -= value.clone();
                } else {
                    remove = true;
                }
                if *entry == N::zero() {
                    remove = true;
                }
            }
            if remove {
                self.map.remove(key);
            }
        }
    }
}

impl<T, N> Sub for Counter<T, N>
where
    T: Hash + Eq,
    N: Clone + PartialOrd + PartialEq + SubAssign + Zero,
{
    type Output = Counter<T, N>;

    /// Subtract (keeping only positive values).
    ///
    /// `out = c - d;` -> `out[x] == c[x] - d[x]` for all `x`,
    /// keeping only items with a value greater than N::zero().
    ///
    /// ```rust
    /// # use counter::Counter;
    /// # use std::collections::HashMap;
    /// let c = "aaab".chars().collect::<Counter<_>>();
    /// let d = "abb".chars().collect::<Counter<_>>();
    ///
    /// let e = c - d;
    ///
    /// let expect = [('a', 2)].iter().cloned().collect::<HashMap<_, _>>();
    /// assert_eq!(e.into_map(), expect);
    /// ```
    fn sub(mut self, rhs: Counter<T, N>) -> Self::Output {
        self -= rhs;
        self
    }
}

impl<T, N> BitAnd for Counter<T, N>
where
    T: Clone + Hash + Eq,
    N: Clone + Ord + AddAssign + SubAssign + Zero + One,
{
    type Output = Counter<T, N>;

    /// Intersection
    ///
    /// `out = c & d;` -> `out[x] == min(c[x], d[x])`
    ///
    /// ```rust
    /// # use counter::Counter;
    /// # use std::collections::HashMap;
    /// let c = "aaab".chars().collect::<Counter<_>>();
    /// let d = "abb".chars().collect::<Counter<_>>();
    ///
    /// let e = c & d;
    ///
    /// let expect = [('a', 1), ('b', 1)].iter().cloned().collect::<HashMap<_, _>>();
    /// assert_eq!(e.into_map(), expect);
    /// ```
    fn bitand(self, rhs: Counter<T, N>) -> Self::Output {
        use std::cmp::min;
        use std::collections::HashSet;

        let self_keys = self.map.keys().collect::<HashSet<_>>();
        let other_keys = rhs.map.keys().collect::<HashSet<_>>();
        let both_keys = self_keys.intersection(&other_keys);

        let mut counter = Counter::new();
        for key in both_keys {
            counter.map.insert(
                (*key).clone(),
                min(self.map.get(*key).unwrap(), rhs.map.get(*key).unwrap()).clone(),
            );
        }

        counter
    }
}

impl<T, N> BitOr for Counter<T, N>
where
    T: Clone + Hash + Eq,
    N: Clone + Ord + Zero,
{
    type Output = Counter<T, N>;

    /// Union
    ///
    /// `out = c | d;` -> `out[x] == max(c[x], d[x])`
    ///
    /// ```rust
    /// # use counter::Counter;
    /// # use std::collections::HashMap;
    /// let c = "aaab".chars().collect::<Counter<_>>();
    /// let d = "abb".chars().collect::<Counter<_>>();
    ///
    /// let e = c | d;
    ///
    /// let expect = [('a', 3), ('b', 2)].iter().cloned().collect::<HashMap<_, _>>();
    /// assert_eq!(e.into_map(), expect);
    /// ```
    fn bitor(self, rhs: Counter<T, N>) -> Self::Output {
        use std::cmp::max;

        let mut counter = self.clone();
        for (key, value) in rhs.map.iter() {
            let entry = counter.map.entry(key.clone()).or_insert(N::zero());
            *entry = max(&*entry, value).clone();
        }
        counter
    }
}

impl<T, N> Deref for Counter<T, N>
where
    T: Hash + Eq,
    N: Clone,
{
    type Target = CounterMap<T, N>;
    fn deref(&self) -> &CounterMap<T, N> {
        &self.map
    }
}

impl<T, N> DerefMut for Counter<T, N>
where
    T: Hash + Eq,
    N: Clone,
{
    fn deref_mut(&mut self) -> &mut CounterMap<T, N> {
        &mut self.map
    }
}

impl<T, Q, N> Index<&'_ Q> for Counter<T, N>
where
    T: Hash + Eq + Borrow<Q>,
    Q: Hash + Eq,
    N: Zero,
{
    type Output = N;

    /// Index in immutable contexts
    ///
    /// Returns a reference to a `zero` value for missing keys.
    ///
    /// ```
    /// # use counter::Counter;
    /// let counter = Counter::<_>::init("aabbcc".chars());
    /// assert_eq!(counter[&'a'], 2);
    /// assert_eq!(counter[&'b'], 2);
    /// assert_eq!(counter[&'c'], 2);
    /// assert_eq!(counter[&'d'], 0);
    /// ```
    ///
    /// Note that the `zero` is a struct filed but not one of the values of the inner `HashMap`. This method does not modify any existing value.
    ///
    /// ```
    /// # use counter::Counter;
    /// let counter = Counter::<_>::init("".chars());
    /// assert_eq!(counter[&'a'], 0);
    /// assert_eq!(counter.get(&'a'), None); // as `Deref<Target = HashMap<_, _>>`
    /// ```
    fn index(&self, key: &'_ Q) -> &N {
        self.map.get(key).unwrap_or(&self.zero)
    }
}

impl<T, Q, N> IndexMut<&'_ Q> for Counter<T, N>
where
    T: Hash + Eq + Borrow<Q>,
    Q: Hash + Eq + ToOwned<Owned = T>,
    N: Zero,
{
    /// Index in mutable contexts
    ///
    /// If the given key is not present, creates a new entry and initializes it with a `zero` value.
    ///
    /// ```
    /// # use counter::Counter;
    /// let mut counter = Counter::<_>::init("aabbcc".chars());
    /// counter[&'c'] += 1;
    /// counter[&'d'] += 1;
    /// assert_eq!(counter[&'c'], 3);
    /// assert_eq!(counter[&'d'], 1);
    /// ```
    ///
    /// Unlike `Index::index`, The returned mutable reference to the `zero` is actually one of the values of the inner `HashMap`.
    ///
    /// ```
    /// # use counter::Counter;
    /// let mut counter = Counter::<_>::init("".chars());
    /// assert_eq!(counter.get(&'a'), None); // as `Deref<Target = HashMap<_, _>>`
    /// let _ = &mut counter[&'a'];
    /// assert_eq!(counter.get(&'a'), Some(&0));
    /// ```
    fn index_mut(&mut self, key: &'_ Q) -> &mut N {
        self.map.entry(key.to_owned()).or_insert_with(N::zero)
    }
}

impl<I, T, N> AddAssign<I> for Counter<T, N>
where
    I: IntoIterator<Item = T>,
    T: Hash + Eq,
    N: PartialOrd + AddAssign + SubAssign + Zero + One,
{
    /// Directly add the counts of the elements of `I` to `self`
    ///
    /// ```rust
    /// # use counter::Counter;
    /// # use std::collections::HashMap;
    /// let mut counter = Counter::init("abbccc".chars());
    ///
    /// counter += "aeeeee".chars();
    /// let expected: HashMap<char, usize> = [('a', 2), ('b', 2), ('c', 3), ('e', 5)]
    ///     .iter().cloned().collect();
    /// assert_eq!(counter.into_map(), expected);
    /// ```
    fn add_assign(&mut self, rhs: I) {
        self.update(rhs);
    }
}

impl<I, T, N> Add<I> for Counter<T, N>
where
    I: IntoIterator<Item = T>,
    T: Hash + Eq,
    N: PartialOrd + AddAssign + SubAssign + Zero + One,
{
    type Output = Self;
    /// Consume self producing a Counter like self updated with the counts of
    /// the elements of I.
    ///
    /// ```rust
    /// # use counter::Counter;
    /// # use std::collections::HashMap;
    /// let counter = Counter::init("abbccc".chars());
    ///
    /// let new_counter = counter + "aeeeee".chars();
    /// let expected: HashMap<char, usize> = [('a', 2), ('b', 2), ('c', 3), ('e', 5)]
    ///     .iter().cloned().collect();
    /// assert_eq!(new_counter.into_map(), expected);
    /// ```
    fn add(mut self, rhs: I) -> Self::Output {
        self.update(rhs);
        self
    }
}

impl<I, T, N> SubAssign<I> for Counter<T, N>
where
    I: IntoIterator<Item = T>,
    T: Hash + Eq,
    N: PartialOrd + AddAssign + SubAssign + Zero + One,
{
    /// Directly subtract the counts of the elements of `I` from `self`,
    /// keeping only items with a value greater than N::zero().
    ///
    /// ```rust
    /// # use counter::Counter;
    /// # use std::collections::HashMap;
    /// let mut c = "aaab".chars().collect::<Counter<_>>();
    /// c -= "abb".chars();
    ///
    /// let expect = [('a', 2)].iter().cloned().collect::<HashMap<_, _>>();
    /// assert_eq!(c.into_map(), expect);
    /// ```
    fn sub_assign(&mut self, rhs: I) {
        self.subtract(rhs);
    }
}

impl<I, T, N> Sub<I> for Counter<T, N>
where
    I: IntoIterator<Item = T>,
    T: Clone + Hash + Eq,
    N: Clone + PartialOrd + AddAssign + SubAssign + Zero + One,
{
    type Output = Self;
    /// Consume self producing a Counter like self with the counts of the
    /// elements of I subtracted, keeping only positive values.
    ///
    /// ```rust
    /// # use counter::Counter;
    /// # use std::collections::HashMap;
    /// let c = "aaab".chars().collect::<Counter<_>>();
    /// let e = c - "abb".chars();
    ///
    /// let expect = [('a', 2)].iter().cloned().collect::<HashMap<_, _>>();
    /// assert_eq!(e.into_map(), expect);
    /// ```
    fn sub(self, rhs: I) -> Self::Output {
        let mut ctr = self.clone();
        ctr.subtract(rhs);
        ctr
    }
}

impl<T, N> iter::FromIterator<T> for Counter<T, N>
where
    T: Hash + Eq,
    N: PartialOrd + AddAssign + SubAssign + Zero + One,
{
    /// Produce a Counter from an iterator of items. This is called automatically
    /// by `iter.collect()`.
    ///
    /// ```rust
    /// # use counter::Counter;
    /// # use std::collections::HashMap;
    /// let counter = "abbccc".chars().collect::<Counter<_>>();
    /// let expect = [('a', 1), ('b', 2), ('c', 3)].iter().cloned().collect::<HashMap<_, _>>();
    /// assert_eq!(counter.into_map(), expect);
    /// ```
    ///
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Counter::<T, N>::init(iter)
    }
}

impl<T, N> iter::FromIterator<(T, N)> for Counter<T, N>
where
    T: Hash + Eq,
    N: PartialOrd + AddAssign + SubAssign + Zero + One,
{
    /// `from_iter` creates a counter from `(item, count)` tuples.
    ///
    /// The counts of duplicate items are summed.
    /// ```rust
    /// # use counter::Counter;
    /// # use std::collections::HashMap;
    /// let counter = [('a', 1), ('b', 2), ('c', 3), ('a', 4)].iter()
    ///     .cloned().collect::<Counter<_>>();
    /// let expect = [('a', 5), ('b', 2), ('c', 3)].iter()
    ///     .cloned().collect::<HashMap<_, _>>();
    /// assert_eq!(counter.into_map(), expect);
    /// ```
    fn from_iter<I: IntoIterator<Item = (T, N)>>(iter: I) -> Self {
        let mut cnt = Counter::new();
        for (item, item_count) in iter.into_iter() {
            let entry = cnt.map.entry(item).or_insert(N::zero());
            *entry += item_count;
        }
        cnt
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_creation() {
        let _: Counter<usize> = Counter::new();

        let initializer = &[1];
        let counter = Counter::init(initializer);

        let mut expected = HashMap::new();
        static ONE: usize = 1;
        expected.insert(&ONE, 1);
        assert!(counter.map == expected);
    }

    #[test]
    fn test_update() {
        let mut counter = Counter::init("abbccc".chars());
        let expected = hashmap! {
            'a' => 1,
            'b' => 2,
            'c' => 3,
        };
        assert!(counter.map == expected);

        counter.update("aeeeee".chars());
        let expected = hashmap! {
            'a' => 2,
            'b' => 2,
            'c' => 3,
            'e' => 5,
        };
        assert!(counter.map == expected);
    }

    #[test]
    fn test_add_update_iterable() {
        let mut counter = Counter::init("abbccc".chars());
        let expected = hashmap! {
            'a' => 1,
            'b' => 2,
            'c' => 3,
        };
        assert!(counter.map == expected);

        counter += "aeeeee".chars();
        let expected = hashmap! {
            'a' => 2,
            'b' => 2,
            'c' => 3,
            'e' => 5,
        };
        assert!(counter.map == expected);
    }

    #[test]
    fn test_add_update_counter() {
        let mut counter = Counter::init("abbccc".chars());
        let expected = hashmap! {
            'a' => 1,
            'b' => 2,
            'c' => 3,
        };
        assert!(counter.map == expected);

        let other = Counter::init("aeeeee".chars());
        counter += other;
        let expected = hashmap! {
            'a' => 2,
            'b' => 2,
            'c' => 3,
            'e' => 5,
        };
        assert!(counter.map == expected);
    }

    #[test]
    fn test_subtract() {
        let mut counter = Counter::init("abbccc".chars());
        counter.subtract("bbccddd".chars());
        let expected = hashmap! {
            'a' => 1,
            'c' => 1,
        };
        assert!(counter.map == expected);
    }

    #[test]
    fn test_sub_update_iterable() {
        let mut counter = Counter::init("abbccc".chars());
        counter -= "bbccddd".chars();
        let expected = hashmap! {
            'a' => 1,
            'c' => 1,
        };
        assert!(counter.map == expected);
    }

    #[test]
    fn test_sub_update_counter() {
        let mut counter = Counter::init("abbccc".chars());
        let other = Counter::init("bbccddd".chars());
        counter -= other;
        let expected = hashmap! {
            'a' => 1,
            'c' => 1,
        };
        assert!(counter.map == expected);
    }

    #[test]
    fn test_composite_add_sub() {
        let mut counts = Counter::<_>::init(
            "able babble table babble rabble table able fable scrabble".split_whitespace(),
        );
        // add or subtract an iterable of the same type
        counts += "cain and abel fable table cable".split_whitespace();
        // or add or subtract from another Counter of the same type
        let other_counts = Counter::init("scrabble cabbie fable babble".split_whitespace());
        let _diff = counts - other_counts;
    }

    #[test]
    fn test_most_common() {
        let counter = Counter::init("abbccc".chars());
        let by_common = counter.most_common();
        let expected = vec![('c', 3), ('b', 2), ('a', 1)];
        assert!(by_common == expected);
    }

    #[test]
    fn test_most_common_tiebreaker() {
        let counter = Counter::init("eaddbbccc".chars());
        let by_common = counter.most_common_tiebreaker(|&a, &b| a.cmp(&b));
        let expected = vec![('c', 3), ('b', 2), ('d', 2), ('a', 1), ('e', 1)];
        assert!(by_common == expected);
    }

    #[test]
    fn test_most_common_tiebreaker_reversed() {
        let counter = Counter::init("eaddbbccc".chars());
        let by_common = counter.most_common_tiebreaker(|&a, &b| b.cmp(&a));
        let expected = vec![('c', 3), ('d', 2), ('b', 2), ('e', 1), ('a', 1)];
        assert!(by_common == expected);
    }

    #[test]
    fn test_most_common_ordered() {
        let counter = Counter::init("eaddbbccc".chars());
        let by_common = counter.most_common_ordered();
        let expected = vec![('c', 3), ('b', 2), ('d', 2), ('a', 1), ('e', 1)];
        assert!(by_common == expected);
    }

    #[test]
    fn test_add() {
        let d = Counter::<_>::init("abbccc".chars());
        let e = Counter::<_>::init("bccddd".chars());

        let out = d + e;
        let expected = Counter::init("abbbcccccddd".chars());
        assert!(out == expected);
    }

    #[test]
    fn test_sub() {
        let d = Counter::<_>::init("abbccc".chars());
        let e = Counter::<_>::init("bccddd".chars());

        let out = d - e;
        let expected = Counter::init("abc".chars());
        assert!(out == expected);
    }

    #[test]
    fn test_intersection() {
        let d = Counter::<_>::init("abbccc".chars());
        let e = Counter::<_>::init("bccddd".chars());

        let out = d & e;
        let expected = Counter::init("bcc".chars());
        assert!(out == expected);
    }

    #[test]
    fn test_union() {
        let d = Counter::<_>::init("abbccc".chars());
        let e = Counter::<_>::init("bccddd".chars());

        let out = d | e;
        let expected = Counter::init("abbcccddd".chars());
        assert!(out == expected);
    }

    #[test]
    fn test_delete_key_from_backing_map() {
        let mut counter = Counter::<_>::init("aa-bb-cc".chars());
        counter.remove(&'-');
        assert!(counter == Counter::init("aabbcc".chars()));
    }

    #[test]
    fn test_from_iter_simple() {
        let counter = "abbccc".chars().collect::<Counter<_>>();
        let expected = hashmap! {
            'a' => 1,
            'b' => 2,
            'c' => 3,
        };
        assert!(counter.map == expected);
    }

    #[test]
    fn test_from_iter_tuple() {
        let items = [('a', 1), ('b', 2), ('c', 3)];
        let counter = items.iter().cloned().collect::<Counter<_>>();
        let expected: HashMap<char, usize> = items.iter().cloned().collect();
        assert_eq!(counter.map, expected);
    }

    #[test]
    fn test_from_iter_tuple_with_duplicates() {
        let items = [('a', 1), ('b', 2), ('c', 3)];
        let counter = items
            .iter()
            .cycle()
            .take(items.len() * 2)
            .cloned()
            .collect::<Counter<_>>();
        let expected: HashMap<char, usize> = items.iter().map(|(c, n)| (*c, n * 2)).collect();
        assert_eq!(counter.map, expected);
    }

    #[test]
    fn test_count_minimal_type() {
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
        // println!("{:?}", inty_counts.map); // test runner blanks this
        // {Inty { i: 8 }: 2, Inty { i: 0 }: 3, Inty { i: 9 }: 1, Inty { i: 3 }: 1,
        //  Inty { i: 7 }: 1, Inty { i: 6 }: 1, Inty { i: 5 }: 1}
        assert!(inty_counts.map.get(&Inty { i: 8 }) == Some(&2));
        assert!(inty_counts.map.get(&Inty { i: 0 }) == Some(&3));
        assert!(inty_counts.map.get(&Inty { i: 6 }) == Some(&1));
    }

    #[test]
    fn test_collect() {
        let counter: Counter<_> = "abbccc".chars().collect();
        let expected = hashmap! {
            'a' => 1,
            'b' => 2,
            'c' => 3,
        };
        assert!(counter.map == expected);
    }

    #[test]
    fn test_non_usize_count() {
        let counter: Counter<_, i8> = "abbccc".chars().collect();
        let expected = hashmap! {
            'a' => 1,
            'b' => 2,
            'c' => 3,
        };
        assert!(counter.map == expected);
    }
}
