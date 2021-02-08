use bevy::prelude::*;

use std::fs::File;
use std::io::BufReader;

#[derive(Default, Clone)]
pub struct WorldProps {
    pub chunk_width: i32,
    pub chunk_height: i32,
    pub tilemap_width: i32,
    pub tilemap_height: i32,
    pub tile_size: i32,
}

// impl World {
//     fn new(chunkwidth: i32, chunkheight: i32, tilemapwidth : i32, tilemapheight: i32, tilesize: i32) -> World {
//        let w = World {
//             chunk_width: chunkwidth,
//             chunk_height: chunkheight,
//             tilemap_width: tilemapwidth,
//             tilemap_height: tilemapheight,
//             tile_size: tilesize,
//         };
//         return w
//     }
// }

// const CHUNK_WIDTH: i32 = 50;
// const CHUNK_HEIGHT: i32 = 50;
// const TILEMAP_WIDTH: i32 = 1000; //CHUNK_WIDTH as i32 * 5;
// const TILEMAP_HEIGHT: i32 = 1000; //CHUNK_HEIGHT as i32 * 5;
// const TILE_SIZE: i32 = 8;

#[derive(Default, Clone)]
pub struct WorldMap {
    pub height: i64,
    pub width: i64,
    pub tiles: Vec<Vec<i64>>,
}

pub fn setup(
    mut world_map: ResMut<WorldMap>,
    mut world: ResMut<WorldProps>,
) {
    world.chunk_width = 50;
    world.chunk_height = 50;
    world.tilemap_width = 1000;
    world.tilemap_height = 1000;
    world.tile_size = 8;

    //Get world map
    let file = File::open("assets/tilemap.json").expect("File not found");
    let reader = BufReader::new(file);

    let mut json_map: serde_json::Value =
        serde_json::from_reader(reader).expect("Read of json file failed");

    world_map.height = json_map["height"].as_i64().unwrap();
    world_map.width = json_map["width"].as_i64().unwrap();

    let layer_0_data = json_map["layers"][0]
        .get_mut("data")
        .expect("Failed")
        .as_array_mut()
        .expect("");

    let height = world_map.height;
    let mut count = 0;

    for _ in (0..world_map.width).rev() {
        world_map.tiles.push(Vec::new());
        for y in (0..world_map.height).rev() {
            world_map.tiles[count as usize].push(layer_0_data[((height * y) + count) as usize].as_i64().unwrap());
        }

        count += 1;
    }
}
