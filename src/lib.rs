//! Counter based on the Python implementation of same:
//! <https://docs.python.org/3.5/library/collections.html#collections.Counter>
//!
//! Counts recurring elements from an iterable.

use std::collections::HashMap;
use std::hash::Hash;

use std::ops::{Add, Sub, AddAssign, SubAssign, BitAnd, BitOr, Deref, DerefMut};

type CounterMap<T> = HashMap<T, usize>;

#[derive(Clone, PartialEq, Eq)]
pub struct Counter<T: Hash + Eq> {
    map: CounterMap<T>,
}

impl<T> Counter<T>
where
    T: Hash + Eq,
{
    /// Create a new, empty `Counter`
    pub fn new() -> Counter<T> {
        Counter { map: HashMap::new() }
    }

    /// Create a new `Counter` initialized with the given iterable
    pub fn init<I>(iterable: I) -> Counter<T>
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
            let entry = self.map.entry(item).or_insert(0);
            *entry += 1;
        }
    }

    /// Remove the counts of the elements from the given iterable to this counter
    ///
    /// Non-positive counts are automatically removed
    pub fn subtract<I>(&mut self, iterable: I)
    where
        I: IntoIterator<Item = T>,
    {
        for item in iterable.into_iter() {
            let mut remove = false;
            if let Some(entry) = self.map.get_mut(&item) {
                if *entry > 0 {
                    *entry -= 1;
                }
                remove = *entry == 0;
            }
            if remove {
                self.map.remove(&item);
            }
        }
    }
}

impl<T> Counter<T>
where
    T: Hash + Eq + Clone,
{
    /// Create an iterator over `(frequency, elem)` pairs, sorted most to least common.
    ///
    /// FIXME: This is pretty inefficient: it copies everything into a vector, sorts
    /// the vector, and returns an iterator over the vector. It would be much better
    /// to create some kind of MostCommon struct which implements `Iterator` which
    /// does all the necessary work on demand. PRs appreciated here!
    pub fn most_common(&self) -> ::std::vec::IntoIter<(T, usize)> {
        use std::cmp::Ordering;
        self.most_common_tiebreaker(|ref _a, ref _b| Ordering::Equal)
    }


    /// Create an iterator over `(frequency, elem)` pairs, sorted most to least common.
    ///
    /// In the event that two keys have an equal frequency, use the supplied ordering function
    /// to further arrange the results.
    ///
    /// FIXME: This is pretty inefficient: it copies everything into a vector, sorts
    /// the vector, and returns an iterator over the vector. It would be much better
    /// to create some kind of MostCommon struct which implements `Iterator` which
    /// does all the necessary work on demand. PRs appreciated here!
    pub fn most_common_tiebreaker<F>(&self, tiebreaker: F) -> ::std::vec::IntoIter<(T, usize)>
    where
        F: Fn(&T, &T) -> ::std::cmp::Ordering,
    {
        use std::cmp::Ordering;

        let mut items = self.map
            .iter()
            .map(|(key, &count)| (key.clone(), count))
            .collect::<Vec<_>>();
        items.sort_by(|&(ref a_item, a_count), &(ref b_item, b_count)| {
            match b_count.cmp(&a_count) {
                Ordering::Equal => tiebreaker(&a_item, &b_item),
                unequal @ _ => unequal,
            }
        });
        items.into_iter()
    }
}

impl<T> Counter<T>
where
    T: Hash + Eq + Clone + Ord,
{
    /// Create an iterator over `(frequency, elem)` pairs, sorted most to least common.
    ///
    /// In the event that two keys have an equal frequency, use the natural ordering of the keys
    /// to further sort the results.
    ///
    /// FIXME: This is pretty inefficient: it copies everything into a vector, sorts
    /// the vector, and returns an iterator over the vector. It would be much better
    /// to create some kind of MostCommon struct which implements `Iterator` which
    /// does all the necessary work on demand. PRs appreciated here!
    pub fn most_common_ordered(&self) -> ::std::vec::IntoIter<(T, usize)> {
        self.most_common_tiebreaker(|ref a, ref b| a.cmp(&b))
    }
}

impl<T> AddAssign for Counter<T>
where
    T: Clone + Hash + Eq,
{
    /// Add another counter to this counter
    ///
    /// `c += d;` -> `c[x] += d[x]` for all `x`
    fn add_assign(&mut self, rhs: Self) {
        for (key, value) in rhs.map.iter() {
            let entry = self.map.entry(key.clone()).or_insert(0);
            *entry += *value;
        }
    }
}

impl<T> Add for Counter<T>
where
    T: Clone + Hash + Eq,
{
    type Output = Counter<T>;

    /// Add two counters together.
    ///
    /// `out = c + d;` -> `out[x] == c[x] + d[x]` for all `x`
    fn add(self, rhs: Counter<T>) -> Counter<T> {
        let mut counter = self.clone();
        counter += rhs;
        counter
    }
}

impl<T> SubAssign for Counter<T>
where
    T: Clone + Hash + Eq,
{
    /// Subtract (keeping only positive values).
    ///
    /// `c -= d;` -> `c[x] -= d[x]` for all `x`,
    /// keeping only items with a value greater than 0.
    fn sub_assign(&mut self, rhs: Self) {
        for (key, value) in rhs.map.iter() {
            let mut remove = false;
            if let Some(entry) = self.map.get_mut(key) {
                if *entry >= *value {
                    *entry -= *value;
                } else {
                    remove = true;
                }
                if *entry == 0 {
                    remove = true;
                }
            }
            if remove {
                self.map.remove(key);
            }
        }
    }
}

impl<T> Sub for Counter<T>
where
    T: Clone + Hash + Eq,
{
    type Output = Counter<T>;

    /// Subtract (keeping only positive values).
    ///
    /// `out = c - d;` -> `out[x] == c[x] - d[x]` for all `x`,
    /// keeping only items with a value greater than 0.
    fn sub(self, rhs: Counter<T>) -> Counter<T> {
        let mut counter = self.clone();
        counter -= rhs;
        counter
    }
}

impl<T> BitAnd for Counter<T>
where
    T: Clone + Hash + Eq,
{
    type Output = Counter<T>;

    /// Intersection
    ///
    /// `out = c & d;` -> `out[x] == min(c[x], d[x])`
    fn bitand(self, rhs: Counter<T>) -> Counter<T> {
        use std::cmp::min;
        use std::collections::HashSet;

        let self_keys = self.map.keys().collect::<HashSet<_>>();
        let other_keys = rhs.map.keys().collect::<HashSet<_>>();
        let both_keys = self_keys.intersection(&other_keys);

        let mut counter = Counter::new();
        for key in both_keys {
            counter.map.insert(
                (*key).clone(),
                min(*self.map.get(*key).unwrap(), *rhs.map.get(*key).unwrap()),
            );
        }

        counter
    }
}

impl<T> BitOr for Counter<T>
where
    T: Clone + Hash + Eq,
{
    type Output = Counter<T>;

    /// Union
    ///
    /// `out = c | d;` -> `out[x] == max(c[x], d[x])`
    fn bitor(self, rhs: Counter<T>) -> Counter<T> {
        use std::cmp::max;

        let mut counter = self.clone();
        for (key, value) in rhs.map.iter() {
            let entry = counter.map.entry(key.clone()).or_insert(0);
            *entry = max(*entry, *value);
        }
        counter
    }
}

impl<T: Hash + Eq> Deref for Counter<T> {
    type Target = CounterMap<T>;
    fn deref(&self) -> &CounterMap<T> {
        &self.map
    }
}

impl<T: Hash + Eq> DerefMut for Counter<T> {
    fn deref_mut(&mut self) -> &mut CounterMap<T> {
        &mut self.map
    }
}

impl<I, T> AddAssign<I> for Counter<T>
where
    T: Hash + Eq,
    I: IntoIterator<Item = T>,
{
    /// Directly add the counts of the elements of `I` to `self`
    fn add_assign(&mut self, rhs: I) {
        self.update(rhs);
    }
}

impl<I, T> Add<I> for Counter<T>
where
    T: Clone + Hash + Eq,
    I: IntoIterator<Item = T>,
{
    type Output = Counter<T>;
    fn add(self, rhs: I) -> Counter<T> {
        let mut ctr = self.clone();
        ctr.update(rhs);
        ctr
    }
}

impl<I, T> SubAssign<I> for Counter<T>
where
    T: Hash + Eq,
    I: IntoIterator<Item = T>,
{
    /// Directly subtract the counts of the elements of `I` from `self`
    fn sub_assign(&mut self, rhs: I) {
        self.subtract(rhs);
    }
}

impl<I, T> Sub<I> for Counter<T>
where
    T: Clone + Hash + Eq,
    I: IntoIterator<Item = T>,
{
    type Output = Counter<T>;
    fn sub(self, rhs: I) -> Counter<T> {
        let mut ctr = self.clone();
        ctr.subtract(rhs);
        ctr
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
        let expected: HashMap<char, usize> =
            [('a', 1), ('b', 2), ('c', 3)].iter().cloned().collect();
        assert!(counter.map == expected);

        counter.update("aeeeee".chars());
        let expected: HashMap<char, usize> = [('a', 2), ('b', 2), ('c', 3), ('e', 5)]
            .iter()
            .cloned()
            .collect();
        assert!(counter.map == expected);
    }

    #[test]
    fn test_add_update_iterable() {
        let mut counter = Counter::init("abbccc".chars());
        let expected: HashMap<char, usize> =
            [('a', 1), ('b', 2), ('c', 3)].iter().cloned().collect();
        assert!(counter.map == expected);

        counter += "aeeeee".chars();
        let expected: HashMap<char, usize> = [('a', 2), ('b', 2), ('c', 3), ('e', 5)]
            .iter()
            .cloned()
            .collect();
        assert!(counter.map == expected);
    }

    #[test]
    fn test_add_update_counter() {
        let mut counter = Counter::init("abbccc".chars());
        let expected: HashMap<char, usize> =
            [('a', 1), ('b', 2), ('c', 3)].iter().cloned().collect();
        assert!(counter.map == expected);

        let other = Counter::init("aeeeee".chars());
        counter += other;
        let expected: HashMap<char, usize> = [('a', 2), ('b', 2), ('c', 3), ('e', 5)]
            .iter()
            .cloned()
            .collect();
        assert!(counter.map == expected);
    }

    #[test]
    fn test_subtract() {
        let mut counter = Counter::init("abbccc".chars());
        counter.subtract("bbccddd".chars());
        let expected: HashMap<char, usize> = [('a', 1), ('c', 1)].iter().cloned().collect();
        assert!(counter.map == expected);
    }

    #[test]
    fn test_sub_update_iterable() {
        let mut counter = Counter::init("abbccc".chars());
        counter -= "bbccddd".chars();
        let expected: HashMap<char, usize> = [('a', 1), ('c', 1)].iter().cloned().collect();
        assert!(counter.map == expected);
    }

    #[test]
    fn test_sub_update_counter() {
        let mut counter = Counter::init("abbccc".chars());
        let other = Counter::init("bbccddd".chars());
        counter -= other;
        let expected: HashMap<char, usize> = [('a', 1), ('c', 1)].iter().cloned().collect();
        assert!(counter.map == expected);
    }

    #[test]
    fn test_composite_add_sub() {
        let mut counts = Counter::init(
            "able babble table babble rabble table able fable scrabble"
                .split_whitespace(),
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
        let by_common = counter.most_common().collect::<Vec<_>>();
        let expected = vec![('c', 3), ('b', 2), ('a', 1)];
        assert!(by_common == expected);
    }

    #[test]
    fn test_most_common_tiebreaker() {
        let counter = Counter::init("eaddbbccc".chars());
        let by_common = counter
            .most_common_tiebreaker(|&a, &b| a.cmp(&b))
            .collect::<Vec<_>>();
        let expected = vec![('c', 3), ('b', 2), ('d', 2), ('a', 1), ('e', 1)];
        assert!(by_common == expected);
    }

    #[test]
    fn test_most_common_tiebreaker_reversed() {
        let counter = Counter::init("eaddbbccc".chars());
        let by_common = counter
            .most_common_tiebreaker(|&a, &b| b.cmp(&a))
            .collect::<Vec<_>>();
        let expected = vec![('c', 3), ('d', 2), ('b', 2), ('e', 1), ('a', 1)];
        assert!(by_common == expected);
    }

    #[test]
    fn test_most_common_ordered() {
        let counter = Counter::init("eaddbbccc".chars());
        let by_common = counter.most_common_ordered().collect::<Vec<_>>();
        let expected = vec![('c', 3), ('b', 2), ('d', 2), ('a', 1), ('e', 1)];
        assert!(by_common == expected);
    }

    #[test]
    fn test_add() {
        let d = Counter::init("abbccc".chars());
        let e = Counter::init("bccddd".chars());

        let out = d + e;
        let expected = Counter::init("abbbcccccddd".chars());
        assert!(out == expected);
    }

    #[test]
    fn test_sub() {
        let d = Counter::init("abbccc".chars());
        let e = Counter::init("bccddd".chars());

        let out = d - e;
        let expected = Counter::init("abc".chars());
        assert!(out == expected);
    }

    #[test]
    fn test_intersection() {
        let d = Counter::init("abbccc".chars());
        let e = Counter::init("bccddd".chars());

        let out = d & e;
        let expected = Counter::init("bcc".chars());
        assert!(out == expected);
    }

    #[test]
    fn test_union() {
        let d = Counter::init("abbccc".chars());
        let e = Counter::init("bccddd".chars());

        let out = d | e;
        let expected = Counter::init("abbcccddd".chars());
        assert!(out == expected);
    }

    #[test]
    fn test_delete_key_from_backing_map() {
        let mut counter = Counter::init("aa-bb-cc".chars());
        counter.remove(&'-');
        assert!(counter == Counter::init("aabbcc".chars()));
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
}
