use std::collections::HashMap;

#[derive(Default, Copy, Clone, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Default)]
pub struct Render {
    pub sprite_index: usize,
    pub z_order: usize,
}

#[derive(Default)]
pub struct Inventory {
    pub items: HashMap<Item, u32>,
}

#[derive(Default, Eq, Hash, Clone)]
pub struct Item {
    pub name: String,
}

// #[derive(Clone, Eq, Hash)]
// pub enum ItemType {
//     Wood { name: String },
//     Water { name: String },
// }

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

#[derive(Default, Clone)]
pub struct Harvestable {
    pub pos: Position,
    pub items: Vec<Item>,
}
