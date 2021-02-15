use bevy::{utils::HashSet};

use super::common_components::*;

#[derive(Default, Clone)]
pub struct GameState {
    pub map_loaded: bool,
    pub spawned: bool,
    pub collisions: HashSet<(i32, i32)>,
    pub harvestable_tiles: Vec<Harvestable>,
}

impl GameState {
    
}
#[derive(Clone)]
pub enum AppState {
    MainMenu,
    InGame,
}
