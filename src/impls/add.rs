use crate::Counter;

use num_traits::{One, Zero};

use std::hash::Hash;
use std::ops::{Add, AddAssign};

impl<T, N> Add for Counter<T, N>
where
    T: Clone + Hash + Eq,
    N: AddAssign + Zero,
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
    fn add(mut self, rhs: Counter<T, N>) -> Self::Output {
        self += rhs;
        self
    }
}

impl<T, N> AddAssign for Counter<T, N>
where
    T: Hash + Eq,
    N: Zero + AddAssign,
{
    /// Add another counter to this counter.
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
        for (key, value) in rhs.map {
            let entry = self.map.entry(key).or_insert_with(N::zero);
            *entry += value;
        }
    }
}

impl<I, T, N> Add<I> for Counter<T, N>
where
    I: IntoIterator<Item = T>,
    T: Hash + Eq,
    N: AddAssign + Zero + One,
{
    type Output = Self;
    /// Consume `self` producing a `Counter` like `self` updated with the counts of
    /// the elements of `I`.
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

impl<I, T, N> AddAssign<I> for Counter<T, N>
where
    I: IntoIterator<Item = T>,
    T: Hash + Eq,
    N: AddAssign + Zero + One,
{
    /// Directly add the counts of the elements of `I` to `self`.
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
