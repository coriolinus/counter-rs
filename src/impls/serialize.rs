use crate::Counter;

use num_traits::Zero;
use serde::de::Deserializer;
use serde::ser::Serializer;
use serde::{Deserialize, Serialize};
use std::hash::Hash;

impl<T, N, St> Serialize for Counter<T, N, St>
where
    T: Serialize + Hash + Eq,
    N: Serialize,
    St: BuildHasher + Default,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.map.serialize(serializer)
    }
}

impl<'de, T, N, St> Deserialize<'de> for Counter<T, N, St>
where
    T: Deserialize<'de> + Hash + Eq,
    N: Deserialize<'de> + Zero,
    St: BuildHasher + Default,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let map = <_>::deserialize(deserializer)?;
        let zero = N::zero();
        Ok(Counter { map, zero })
    }
}
