use crate::Counter;

use num_traits::Zero;

use std::hash::{BuildHasher, Hash};
use std::ops::{Add, AddAssign};

impl<T, N, S> Add for Counter<T, N, S>
where
    T: Clone + Hash + Eq,
    N: AddAssign + Zero,
    S: BuildHasher,
{
    type Output = Counter<T, N, S>;

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
    fn add(mut self, rhs: Counter<T, N, S>) -> Self::Output {
        self += rhs;
        self
    }
}

impl<T, N, S> AddAssign for Counter<T, N, S>
where
    T: Hash + Eq,
    N: Zero + AddAssign,
    S: BuildHasher,
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
