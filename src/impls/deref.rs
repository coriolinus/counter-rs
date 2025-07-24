use crate::Counter;

use std::collections::HashMap;
use std::hash::{BuildHasher, Hash};
use std::ops::{Deref, DerefMut};

type CounterMap<T, N, S> = HashMap<T, N, S>;

impl<T, N, S> Deref for Counter<T, N, S>
where
    T: Hash + Eq,
    S: BuildHasher + Default,
{
    type Target = CounterMap<T, N, S>;
    fn deref(&self) -> &CounterMap<T, N, S> {
        &self.map
    }
}

impl<T, N, S> DerefMut for Counter<T, N, S>
where
    T: Hash + Eq,
    S: BuildHasher + Default,
{
    fn deref_mut(&mut self) -> &mut CounterMap<T, N, S> {
        &mut self.map
    }
}
