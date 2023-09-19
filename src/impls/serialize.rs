use crate::Counter;

use std::fmt;
use std::marker::PhantomData;
use std::hash::Hash;
use num_traits::Zero;
use serde::{Serialize, Deserialize};
use serde::ser::{SerializeMap, Serializer};
use serde::de::{Deserializer, Visitor, MapAccess};


impl<T, N> Serialize for Counter<T, N> 
where
    T: Serialize + Hash + Eq,
    N: Serialize,
{
    fn serialize<S>(&self, serializer:S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        let mut map = serializer.serialize_map(Some(self.len()))?;
        for (k, v) in self.into_iter() {
            map.serialize_entry(&k, &v)?;
        }
        map.end()
    }
}

struct CounterVisitor<T, N>
where
    T: Hash + Eq
{
    marker: PhantomData<fn() -> Counter<T, N>>
}

impl<'de, T, N> CounterVisitor<T, N>
where
    T: Deserialize<'de> + Hash + Eq,
    N: Deserialize<'de>,
{
    fn new() -> Self {
        CounterVisitor {
            marker: PhantomData
        }
    }
}

impl<'de, T, N> Visitor<'de> for CounterVisitor<T, N>
where
    T: Deserialize<'de> + Hash + Eq,
    N: Deserialize<'de> + Zero,
{
    type Value = Counter<T, N>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a counter for type T")
    }

    fn visit_map<A>(self, mut access: A) -> std::result::Result<Self::Value, A::Error>
    where A: MapAccess<'de> {
        let mut map = Counter::new();
        
        while let Some((k, v)) = access.next_entry()? {
            map.insert(k, v);
        }
        Ok(map)
    }
}

impl<'de, T, N> Deserialize<'de> for Counter<T, N>
where
    T: Deserialize<'de> + Hash + Eq,
    N: Deserialize<'de> + Zero,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: Deserializer<'de> {
        deserializer.deserialize_map(CounterVisitor::new())
    }
}