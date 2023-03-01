use std::collections::HashMap;
use std::hash::{Hash, BuildHasher};

use cgmath::Vector3;

pub fn if_let<F, O, A>(option: Option<O>, args: A, if_some: F) where F: FnOnce(O, A) {
    let Some(value) = option else { return };
    if_some(value, args);
}

pub struct BoundingBox(pub Vector3<f32>, pub Vector3<f32>);

pub enum ThisOrThat<I, A> {
    This(I),
    That(A),
}

pub enum ThisOrThatOrThot<I, A, O> {
    This(I),
    That(A),
    Thot(O),
}

pub trait GetOrInsert<K, V> {
    fn get_or_insert(&mut self, get: impl Into<K>, insert: V) -> &mut V;
}

impl<K, V, S> GetOrInsert<K, V> for HashMap<K, V, S> where K: Eq, K: Hash, S: BuildHasher {
    fn get_or_insert(&mut self, get: impl Into<K>, insert: V) -> &mut V {
        self.entry(get.into()).or_insert(insert)
    }
}