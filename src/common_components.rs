use std::collections::HashMap;

#[derive(Default, Copy, Clone, PartialEq, Debug)]
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
    pub items: HashMap<ItemType, u32>,
}

// #[derive(Default, Eq, Hash, Clone)]
// pub struct Item {
//     pub name: String,
// }

// impl PartialEq for Item {
//     fn eq(&self, other: &Self) -> bool {
//         self.name == other.name
//     }
// }

#[derive(Clone, Eq, Hash, Debug)]
pub enum ItemType {
    Wood { name: String },
    Water { name: String, consume: Consumable },
}

impl PartialEq for ItemType {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

#[derive(Default, Clone)]
pub struct Harvestable {
    pub pos: Position,
    pub items: Vec<ItemType>,
}

#[derive(Default, Clone, Eq, Hash, Debug)]
pub struct Consumable {
    pub attribute_effect: Vec<(Attribute, i32)>,
}

impl PartialEq for Consumable {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

// impl Consumable {
//     fn Consume(&self) {
//         self.attribute_effect
//     }
// }

#[derive(Clone, Hash, Eq, Debug)]
pub enum Attribute {
    Thirst,
    Hunger,
    Health,
    Temperature,
}

impl PartialEq for Attribute {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}
