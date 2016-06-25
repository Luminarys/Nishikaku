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
