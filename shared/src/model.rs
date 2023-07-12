// TODO Client only

use crate::types::Id;
use std::fs::ReadDir;
use std::hash::Hash;

pub struct BlockModel {
    //
}

pub struct ItemModel {
    //
}

pub fn load_item_model<T: Eq + Hash + PartialEq>(addon_path: &ReadDir, id: &Id<T>) -> ItemModel {
    ItemModel {}
}
