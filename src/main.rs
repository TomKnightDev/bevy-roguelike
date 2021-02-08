use bevy::{
    asset::LoadState, prelude::*, render::camera::Camera, sprite::TextureAtlasBuilder,
    utils::HashSet, window::WindowMode,
};
use bevy_tilemap::prelude::*;
use serde::Deserialize;
use serde_json;

use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Default, Clone)]
struct SpriteHandles {
    handles: Vec<HandleUntyped>,
    atlas_loaded: bool,
}

#[derive(Default, Clone)]
struct GameState {
    map_loaded: bool,
    spawned: bool,
    collisions: HashSet<(i32, i32)>,
}

impl GameState {
    fn try_move_player(
        &mut self,
        position: &mut Position,
        camera_translation: &mut Vec3,
        delta_xy: (i32, i32),
    ) {
        let new_pos = (position.x + delta_xy.0, position.y + delta_xy.1);
        if !self.collisions.contains(&new_pos) {
            position.x = position.x + delta_xy.0;
            position.y = position.y + delta_xy.1;
            camera_translation.x = camera_translation.x + (delta_xy.0 as f32 * 32.);
            camera_translation.y = camera_translation.y + (delta_xy.1 as f32 * 32.);
        }
    }
}

#[derive(Default, Clone)]
struct WorldMap {
    height: i64,
    width: i64,
    tiles: Vec<Vec<i64>>,
}

#[derive(Default)]
struct Player {}

#[derive(Default, Copy, Clone, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Default)]
struct Render {
    sprite_index: usize,
    z_order: usize,
}

#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    position: Position,
    render: Render,
}

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "Square Tiles".to_string(),
            width: 1920.,
            height: 1080.,
            vsync: false,
            resizable: true,
            mode: WindowMode::Windowed,
            ..Default::default()
        })
        .init_resource::<SpriteHandles>()
        .init_resource::<GameState>()
        .init_resource::<WorldMap>()
        .add_plugins(DefaultPlugins)
        .add_plugins(TilemapDefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(load.system())
        .add_system(build_world.system())
        .add_system(character_movement.system())
        .run()
}

fn setup(
    commands: &mut Commands,
    mut tile_sprite_handles: ResMut<SpriteHandles>,
    mut world_map: ResMut<WorldMap>,
    asset_server: Res<AssetServer>,
) {
    tile_sprite_handles.handles = asset_server.load_folder("textures").unwrap();
    //Get world map
    let file = File::open("assets/Tilemap/test_world.json").expect("File not found");
    let reader = BufReader::new(file);

    let json_map: serde_json::Value =
        serde_json::from_reader(reader).expect("Read of json file failed");
    let layer_0_data = json_map["layers"][0]
        .get("data")
        .expect("Failed")
        .as_array()
        .expect("");

    world_map.height = json_map["height"].as_i64().unwrap();
    world_map.width = json_map["width"].as_i64().unwrap();

    let width = world_map.width;

    for x in 0..world_map.width {
        world_map.tiles.push(Vec::new());

        for y in 0..world_map.height {
            world_map.tiles[x as usize]
                .push(layer_0_data[((width * x) + y) as usize].as_i64().unwrap());
        }
    }

    // for i in world_map.tiles.iter() {
    //     println!("{:?}", i);
    // }
}

fn load(
    commands: &mut Commands,
    mut sprite_handles: ResMut<SpriteHandles>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Texture>>,
    asset_server: Res<AssetServer>,
) {
    if sprite_handles.atlas_loaded {
        return;
    }

    // Lets load all our textures from our folder!
    let mut texture_atlas_builder = TextureAtlasBuilder::default();
    if let LoadState::Loaded =
        asset_server.get_group_load_state(sprite_handles.handles.iter().map(|handle| handle.id))
    {
        for handle in sprite_handles.handles.iter() {
            let texture = textures.get(handle).unwrap();
            texture_atlas_builder.add_texture(handle.clone_weak().typed::<Texture>(), &texture);
        }

        let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();
        let atlas_handle = texture_atlases.add(texture_atlas);

        let tilemap = Tilemap::builder()
            .topology(GridTopology::Square)
            .dimensions(4, 4)
            .chunk_dimensions(10, 10)
            .tile_dimensions(32, 32)
            .auto_chunk()
            .auto_configure()
            .z_layers(3)
            .texture_atlas(atlas_handle)
            .finish()
            .unwrap();

        let tilemap_components = TilemapBundle {
            tilemap,
            transform: Default::default(),
            global_transform: Default::default(),
        };

        commands.spawn(Camera2dBundle::default());
        commands
            .spawn(tilemap_components)
            .with(Timer::from_seconds(0.075, true));

        sprite_handles.atlas_loaded = true;
    }
}

fn build_world(
    commands: &mut Commands,
    mut game_state: ResMut<GameState>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
    mut query: Query<&mut Tilemap>,
    world_map: ResMut<WorldMap>,
) {
    if game_state.map_loaded {
        return;
    }

    for mut map in query.iter_mut() {
        let chunk_width = (map.width().unwrap() * map.chunk_width()) as i32;
        let chunk_height = (map.height().unwrap() * map.chunk_height()) as i32;
        let texture_atlas = texture_atlases.get(map.texture_atlas()).unwrap();

        let floor: Handle<Texture> = asset_server.get_handle("textures/terrain/square-floor.png");
        let wall: Handle<Texture> = asset_server.get_handle("textures/terrain/square-wall.png");
        let floor_index = texture_atlas.get_texture_index(&floor).unwrap();
        let wall_index = texture_atlas.get_texture_index(&wall).unwrap();

        let mut tiles = Vec::new();
        let mut world_x = 0;
        let mut world_y = 0;

        for x in 0..chunk_width {
            for y in 0..chunk_height {
                let y = y - chunk_height / 2;
                let x = x - chunk_width / 2;

                let mut tile = Tile::new((x, y), floor_index);

                if world_x < 100 && world_y < 100 {
                    let tile_index = world_map.tiles[world_x as usize][world_y as usize] as usize;
                    match tile_index {
                        1 => tile.sprite_index = floor_index,
                        _ => {
                            tile.sprite_index = wall_index;
                            game_state.collisions.insert((x, y));
                        }
                    }
                }

                tiles.push(tile);

                world_y += 1;
            }

            world_y = 0;
            world_x += 1;
        }

        map.add_layer_with_kind(LayerKind::Sparse, 1).unwrap();

        // Now lets add in a dwarf friend!
        let dwarf_sprite: Handle<Texture> =
            asset_server.get_handle("textures/creatures/square-dwarf.png");
        let dwarf_sprite_index = texture_atlas.get_texture_index(&dwarf_sprite).unwrap();
        // We add in a Z order of 1 to place the tile above the background on Z
        // order 0.
        let mut dwarf_tile = Tile::new((0, 0), dwarf_sprite_index);
        dwarf_tile.z_order = 1;
        tiles.push(dwarf_tile);

        commands.spawn(PlayerBundle {
            player: Player {},
            position: Position { x: 0, y: 0 },
            render: Render {
                sprite_index: dwarf_sprite_index,
                z_order: 1,
            },
        });

        map.insert_tiles(tiles).unwrap();

        // map.spawn_chunk((-1, 0)).unwrap();
        map.spawn_chunk((0, 0)).unwrap();
        // map.spawn_chunk((1, 0)).unwrap();
        // map.spawn_chunk((-1, 1)).unwrap();
        // map.spawn_chunk((0, 1)).unwrap();
        // map.spawn_chunk((1, 1)).unwrap();
        // map.spawn_chunk((-1, -1)).unwrap();
        // map.spawn_chunk((0, -1)).unwrap();
        // map.spawn_chunk((1, -1)).unwrap();

        game_state.map_loaded = true;
    }
}

fn move_sprite(
    map: &mut Tilemap,
    previous_position: Position,
    position: Position,
    render: &Render,
    // camera_translation: &mut Transform,
) {
    // println!(
    //     "Previous pos - {}/{}, New pos - {}/{}",
    //     previous_position.x, previous_position.y, position.x, position.y
    // );

    // We need to first remove where we were prior.
    map.clear_tile((previous_position.x, previous_position.y), 1)
        .unwrap();
    // We then need to update where we are going!
    let mut tile = Tile::new((position.x, position.y), render.sprite_index);
    tile.z_order = render.z_order;

    map.insert_tile(tile).unwrap();

    map.spawn_chunk_containing_point((position.x, position.y)).unwrap();
    // map.despawn_chunk((previous_position.x, previous_position.y)).unwrap();
}

fn character_movement(
    mut game_state: ResMut<GameState>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut map_query: Query<(&mut Tilemap, &mut Timer)>,
    mut player_query: Query<(&mut Position, &Render, &Player)>,
    mut camera_query: Query<(&Camera, &mut Transform)>,
) {
    if !game_state.map_loaded {
        return;
    }

    for (mut map, mut timer) in map_query.iter_mut() {
        timer.tick(time.delta_seconds());
        if !timer.finished() {
            continue;
        }

        for (mut position, render, _player) in player_query.iter_mut() {
            for key in keyboard_input.get_pressed() {
                for (_camera, mut camera_transform) in camera_query.iter_mut() {
                    // First we need to store our very current position.
                    let previous_position = *position;

                    // Of course we need to control where we are going to move our
                    // dwarf friend.
                    use KeyCode::*;
                    match key {
                        W | Numpad8 | Up | K => {
                            game_state.try_move_player(
                                &mut position,
                                &mut camera_transform.translation,
                                (0, 1),
                            );
                        }
                        A | Numpad4 | Left | H => {
                            game_state.try_move_player(
                                &mut position,
                                &mut camera_transform.translation,
                                (-1, 0),
                            );
                        }
                        S | Numpad2 | Down | J => {
                            game_state.try_move_player(
                                &mut position,
                                &mut camera_transform.translation,
                                (0, -1),
                            );
                        }
                        D | Numpad6 | Right | L => {
                            game_state.try_move_player(
                                &mut position,
                                &mut camera_transform.translation,
                                (1, 0),
                            );
                        }

                        Numpad9 | U => game_state.try_move_player(
                            &mut position,
                            &mut camera_transform.translation,
                            (1, 1),
                        ),
                        Numpad3 | M => game_state.try_move_player(
                            &mut position,
                            &mut camera_transform.translation,
                            (1, -1),
                        ),
                        Numpad1 | N => game_state.try_move_player(
                            &mut position,
                            &mut camera_transform.translation,
                            (-1, -1),
                        ),
                        Numpad7 | Y => game_state.try_move_player(
                            &mut position,
                            &mut camera_transform.translation,
                            (-1, 1),
                        ),

                        _ => {}
                    }

                    // Finally now we will move the sprite! ... Provided he had
                    // moved!
                    move_sprite(&mut map, previous_position, *position, render);
                }
            }
        }
    }
}
