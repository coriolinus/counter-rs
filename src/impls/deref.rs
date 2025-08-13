use crate::Counter;

use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

impl<T, N, S> Deref for Counter<T, N, S> {
    type Target = HashMap<T, N, S>;
    fn deref(&self) -> &HashMap<T, N, S> {
        &self.map
    }
}

impl<T, N, S> DerefMut for Counter<T, N, S> {
    fn deref_mut(&mut self) -> &mut HashMap<T, N, S> {
        &mut self.map
    }
}
