use std::collections::{HashMap as OHashMap, HashSet as OHashSet};
use std::hash::{Hash, BuildHasherDefault};
use std::cmp::Eq;
use fnv::FnvHasher;

pub type HashMap<K, V> = OHashMap<K, V, BuildHasherDefault<FnvHasher>>;
pub type HashSet<T> = OHashSet<T, BuildHasherDefault<FnvHasher>>;

pub fn hashmap<K: Hash + Eq, V>() -> OHashMap<K, V, BuildHasherDefault<FnvHasher>> {
    let fnv = BuildHasherDefault::<FnvHasher>::default();
    OHashMap::with_hasher(fnv)
}

pub fn hashset<T: Hash + Eq>() -> OHashSet<T, BuildHasherDefault<FnvHasher>> {
    let fnv = BuildHasherDefault::<FnvHasher>::default();
    OHashSet::with_hasher(fnv)
}

pub trait ToPolar {
    fn to_polar(&self) -> Self;
}

pub trait ToCartesian {
    fn to_cartesian(&self) -> Self;
}

use nalgebra::Vector2;

impl ToPolar for Vector2<f32> {
    fn to_polar(&self) -> Vector2<f32> {
        Vector2::new(
            (self.x.powi(2) + self.y.powi(2)).sqrt(),
            (self.y.atan2(self.x))
        )
    }
}

impl ToCartesian for Vector2<f32> {
    fn to_cartesian(&self) -> Vector2<f32> {
        Vector2::new(
            (self.x * self.y.cos()),
            (self.x * self.y.sin())
        )
    }
}
