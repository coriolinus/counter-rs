use crate::Counter;

use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

impl<T, N> Deref for Counter<T, N> {
    type Target = HashMap<T, N>;
    fn deref(&self) -> &HashMap<T, N> {
        &self.map
    }
}

impl<T, N> DerefMut for Counter<T, N> {
    fn deref_mut(&mut self) -> &mut HashMap<T, N> {
        &mut self.map
    }
}
