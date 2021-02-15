use bevy::{asset::LoadState, prelude::*, sprite::TextureAtlasBuilder, window::WindowMode};
use bevy_tilemap::prelude::*;
use sprite_tools::SpriteChangeEvent;
use std::collections::HashMap;
use ui::{InventoryButtonEvent, InventoryButtonEventListenerState};

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, PrintDiagnosticsPlugin};

use bevy_egui::{EguiPlugin, EguiSettings};

mod ui;

mod world;
use world::*;
pub mod common_components;
use common_components::{Attribute, Consumable, Harvestable, Inventory, Position, Render};
mod player;
use player::*;
pub mod game;
use game::*;
pub mod sprite_tools;
use sprite_tools::*;
mod player_input;
use player_input::*;

const STAGE: &str = "app_state";

pub struct Materials {
    pub wall_horizontal_material: Handle<ColorMaterial>,
}

#[derive(Default, Clone)]
struct SpriteHandles {
    handles: Vec<HandleUntyped>,
    atlas_loaded: bool,
}

fn main() -> Result<(), ()> {
    App::build()
        .add_resource(WindowDescriptor {
            title: "Roguelike".to_string(),
            width: 1920.,
            height: 1080.,
            vsync: false,
            resizable: true,
            mode: WindowMode::Windowed,
            ..Default::default()
        })
        .add_resource(ClearColor(Color::BLACK))
        .add_resource(State::new(AppState::MainMenu))
        .init_resource::<SpriteHandles>()
        .init_resource::<GameState>()
        .init_resource::<WorldMap>()
        .init_resource::<WorldProps>()
        .init_resource::<Player>()
        .init_resource::<MouseState>()
        .init_resource::<SpriteChangedEventListenerState>()
        .init_resource::<InventoryButtonEventListenerState>()
        .add_event::<SpriteChangeEvent>()
        .add_event::<InventoryButtonEvent>()
        // .init_resource::<player_input::InputState>()
        .add_plugins(DefaultPlugins)
        .add_plugins(TilemapDefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_stage_after(stage::UPDATE, STAGE, StateStage::<AppState>::default())
        .add_plugin(SetupGamePlugin)
        .add_plugin(PrintDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_system(PrintDiagnosticsPlugin::print_diagnostics_system.system())
        .on_state_update(STAGE, AppState::MainMenu, ui::main_menu.system())
        .run();

    Ok(())
}

fn setup(
    commands: &mut Commands,
    mut tile_sprite_handles: ResMut<SpriteHandles>,
    asset_server: Res<AssetServer>,
    mut egui_settings: ResMut<EguiSettings>,
) {
    egui_settings.scale_factor = 1.0;

    tile_sprite_handles.handles = asset_server.load_folder("textures").unwrap();
    commands.insert_resource(Materials {
        wall_horizontal_material: asset_server.load("textures/buildings/wall_horizontal.png"),
    });
}

pub struct SetupGamePlugin;

impl Plugin for SetupGamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        // app.add_startup_system(setup.system())
        //     .add_startup_system(world::setup.system())
        //     .add_startup_system(player_input::input_setup.system())
        //     .add_system(load.system())
        //     .add_system(build_world.system())
        //     .add_system(player::character_movement.system())
        //     .add_system(ui::ui_windows.system())
        //     .add_system(player_input::my_cursor_system.system())
        //     .add_system(sprite_tools::sprite_change_event.system())
        //     .add_system(ui::inventory_button_event.system())
        //     .add_plugin(PrintDiagnosticsPlugin::default())
        //     .add_plugin(FrameTimeDiagnosticsPlugin::default())
        //     .add_system(PrintDiagnosticsPlugin::print_diagnostics_system.system());

        app.on_state_enter(STAGE, AppState::InGame, setup.system())
            .on_state_enter(STAGE, AppState::InGame, world::setup.system())
            .on_state_enter(STAGE, AppState::InGame, player_input::input_setup.system())
            .on_state_update(STAGE, AppState::InGame, load.system())
            .on_state_update(STAGE, AppState::InGame, build_world.system())
            .on_state_update(STAGE, AppState::InGame, player::character_movement.system())
            .on_state_update(STAGE, AppState::InGame, ui::ui_windows.system())
            .on_state_update(
                STAGE,
                AppState::InGame,
                player_input::my_cursor_system.system(),
            )
            .on_state_update(
                STAGE,
                AppState::InGame,
                sprite_tools::sprite_change_event.system(),
            )
            .on_state_update(STAGE, AppState::InGame, ui::inventory_button_event.system());
    }
}

fn load(
    commands: &mut Commands,
    mut sprite_handles: ResMut<SpriteHandles>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Texture>>,
    asset_server: Res<AssetServer>,
    worldprops: Res<WorldProps>,
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

            // let ta = TextureAtlas::from_grid(
            //     sprite_handles.handles[0].clone_weak().typed::<Texture>(),
            //     Vec2::new(10.0, 10.0),
            //     30 as usize,
            //     30 as usize,
            // );

            // let atlas_handle = texture_atlases.add(ta);
        }

        let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();
        let atlas_handle = texture_atlases.add(texture_atlas);

        let tilemap = Tilemap::builder()
            .topology(GridTopology::Square)
            .dimensions(
                worldprops.tilemap_width as u32,
                worldprops.tilemap_height as u32,
            )
            .chunk_dimensions(
                worldprops.chunk_width as u32,
                worldprops.chunk_height as u32,
            )
            .tile_dimensions(worldprops.tile_size as u32, worldprops.tile_size as u32)
            .auto_chunk()
            // .auto_configure()
            // .z_layers(3)
            .texture_atlas(atlas_handle)
            .finish()
            .unwrap();

        let tilemap_components = TilemapBundle {
            tilemap,
            transform: Default::default(),
            global_transform: Default::default(),
        };

        commands
            .spawn(Camera2dBundle {
                transform: Transform {
                    translation: Vec3::new(
                        0.0, //500 as f32 * worldprops.tile_size as f32,
                        0.0, //500 as f32 * worldprops.tile_size as f32,
                        15.0,
                    ),
                    rotation: Quat::identity(),
                    scale: Vec3::new(0.2, 0.2, 1.0),
                },
                ..Default::default()
            })
            .with(player_input::MainCamera);

        commands
            .spawn(tilemap_components)
            .with(Timer::from_seconds(0.1, true));

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
    worldprops: Res<WorldProps>,
    player: Res<Player>,
) {
    if game_state.map_loaded {
        return;
    }

    for mut map in query.iter_mut() {
        // let chunk_width = (map.width().unwrap() * map.chunk_width()) as i32;
        // let chunk_height = (map.height().unwrap() * map.chunk_height()) as i32;
        let texture_atlas = texture_atlases.get(map.texture_atlas()).unwrap();

        let grass_0: Handle<Texture> = asset_server.get_handle("textures/terrain/grass_0.png");
        let grass_1: Handle<Texture> = asset_server.get_handle("textures/terrain/grass_1.png");
        let grass_2: Handle<Texture> = asset_server.get_handle("textures/terrain/grass_2.png");
        let grass_3: Handle<Texture> = asset_server.get_handle("textures/terrain/grass_3.png");
        let grass_0_index = texture_atlas.get_texture_index(&grass_0).unwrap();
        let grass_1_index = texture_atlas.get_texture_index(&grass_1).unwrap();
        let grass_2_index = texture_atlas.get_texture_index(&grass_2).unwrap();
        let grass_3_index = texture_atlas.get_texture_index(&grass_3).unwrap();

        let tree_0: Handle<Texture> = asset_server.get_handle("textures/terrain/tree_0.png");
        let tree_1: Handle<Texture> = asset_server.get_handle("textures/terrain/tree_1.png");
        let tree_2: Handle<Texture> = asset_server.get_handle("textures/terrain/tree_2.png");
        let tree_0_index = texture_atlas.get_texture_index(&tree_0).unwrap();
        let tree_1_index = texture_atlas.get_texture_index(&tree_1).unwrap();
        let tree_2_index = texture_atlas.get_texture_index(&tree_2).unwrap();

        let water_4: Handle<Texture> = asset_server.get_handle("textures/terrain/water_4.png");
        let water_4_index = texture_atlas.get_texture_index(&water_4).unwrap();

        // let wall: Handle<Texture> = asset_server.get_handle("textures/terrain/square-wall.png");
        // let wall_index = texture_atlas.get_texture_index(&wall).unwrap();

        let mut tiles = Vec::new();
        let mut underground_tiles = Vec::new();
        for x in 0..worldprops.tilemap_width {
            for y in 0..worldprops.tilemap_height {
                let mut tile = Tile::new((x, y), grass_0_index);
                tile.z_order = 1;
                let tile_index = world_map.tiles[x as usize][y as usize] as usize;

                //Collidables and Harvestables
                if (tile_index > 3 && tile_index < 7) || tile_index == 11 {
                    game_state.collisions.insert((x, y));

                    if tile_index > 3 && tile_index < 7 {
                        let h = Harvestable {
                            pos: Position { x: x, y: y },
                            items: vec![common_components::ItemType::Wood {
                                name: String::from("Wood"),
                            }],
                        };
                        game_state.harvestable_tiles.push(h);
                    } else if tile_index == 11 {
                        let h = Harvestable {
                            pos: Position { x: x, y: y },
                            items: vec![common_components::ItemType::Water {
                                name: String::from("Water"),
                                consume: Consumable {
                                    attribute_effect: vec![(Attribute::Thirst, 20)],
                                },
                            }],
                        };
                        game_state.harvestable_tiles.push(h)
                    }
                }

                match tile_index {
                    1 => tile.sprite_index = grass_0_index,
                    2 => tile.sprite_index = grass_1_index,
                    3 => tile.sprite_index = grass_2_index,
                    4 => tile.sprite_index = tree_0_index,
                    5 => tile.sprite_index = tree_1_index,
                    6 => tile.sprite_index = tree_2_index,
                    11 => tile.sprite_index = water_4_index,
                    22 => tile.sprite_index = grass_3_index,
                    _ => {
                        tile.sprite_index = grass_0_index;
                        // game_state.collisions.insert((x, y));
                    }
                }

                tiles.push(tile);

                //Underground tiles
                let mut underground_tile = Tile::new((x, y), grass_0_index);
                underground_tile.z_order = 0;
                underground_tiles.push(underground_tile);
            }
        }

        map.add_layer_with_kind(LayerKind::Sparse, 1).unwrap();

        // Now lets add in a dwarf friend!
        let player_sprite: Handle<Texture> =
            asset_server.get_handle("textures/creatures/player.png");
        let player_sprite_index = texture_atlas.get_texture_index(&player_sprite).unwrap();
        // We add in a Z order of 1 to place the tile above the background on Z
        // order 0.
        let mut player_tile = Tile::new(player.start_pos, player_sprite_index);
        player_tile.z_order = 2;
        tiles.push(player_tile);

        let player_start = (0, 0); //(worldprops.tilemap_width / 2, worldprops.tilemap_height / 2);

        commands.spawn(PlayerBundle {
            player: Player {
                start_pos: player_start,
                name: String::from("Player1"),
                health: 100.0,
                thirst: 0.0,
                hunger: 0.0,
                temperature: 50.0,
            },
            position: Position {
                x: player_start.0,
                y: player_start.1,
            },
            render: Render {
                sprite_index: player_sprite_index,
                z_order: 2,
            },
            inventory: Inventory {
                items: HashMap::new(),
            },
        });

        map.insert_tiles(tiles).unwrap();
        map.insert_tiles(underground_tiles).unwrap();

        let chunk_point = map.point_to_chunk_point(player_start);

        for x in chunk_point.0 - 1..=chunk_point.0 + 1 {
            for y in chunk_point.1 - 1..=chunk_point.1 + 1 {
                map.spawn_chunk((x, y)).expect("Chunk failed to load");
            }
        }

        // map.spawn_chunk_containing_point(player_start).unwrap();

        // map.spawn_chunk((-1, 0)).unwrap();
        // map.spawn_chunk((0, 0)).unwrap();
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
