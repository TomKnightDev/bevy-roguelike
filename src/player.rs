use bevy::{prelude::*, render::camera::Camera, utils::HashSet};
use bevy_tilemap::prelude::*;

use super::common_components::*;
use super::game::*;
use super::sprite_tools::*;
use super::world::*;

#[derive(Default)]
pub struct Player {
    pub start_pos: (i32, i32),
    pub name: String,
    pub health: f32,
    pub thirst: f32,
    pub hunger: f32,
    pub temperature: f32,
}

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub position: Position,
    pub render: Render,
    pub inventory: Inventory,
}

impl Player {
    fn add_fatigue(&mut self) {
        if self.thirst >= 100.0 || self.hunger >= 100.0 {
            self.health -= 0.1;
        } else {
            self.thirst += 0.05;
            self.hunger += 0.01;
        }
    }

    pub fn consume_item(&mut self, item_type: &ItemType) -> bool {
        match item_type {
            ItemType::Water {name: _, consume } => {
                for e in consume.attribute_effect.iter() {
                    match e.0 {
                        Attribute::Health => {
                            if self.health >= 100.0 {
                                return false;
                            }
                            self.health += f32::min(self.health, e.1 as f32);
                            return true;
                        }
                        Attribute::Thirst => {
                            if self.thirst <= 0.0 {
                                return false;
                            }
                            self.thirst -= f32::min(self.thirst, e.1 as f32);
                            return true;
                        }
                        Attribute::Hunger => {
                            if self.hunger <= 0.0 {
                                return false;
                            }
                            self.hunger -= f32::min(self.hunger,e.1 as f32);
                            return true;
                        }
                        Attribute::Temperature => {                            
                            self.temperature += e.1 as f32;
                            return true;
                        }
                    }
                }
            }
            _ => return false,
        }

        return false;
    }
}

// pub fn setup(commands: &mut Commands, mut player: ResMut<Player>, world_props: Res<WorldProps>) {
//     player.start_pos = (
//         world_props.tilemap_width / 2,
//         world_props.tilemap_height / 2,
//     );
//     player.name = String::from("Player1");
//     player.health = 100;

//     // let player_start = (1000/ 2, 1000 / 2);

//     // commands.spawn(Camera2dBundle {
//     //     transform: Transform {
//     //         translation: Vec3::new(
//     //             player_start.0 as f32,
//     //             player_start.1 as f32,
//     //             1.0,
//     //         ),
//     //         rotation: Quat::identity(),
//     //         scale: Vec3::new(0.3, 0.3, 1.0),
//     //     },
//     //     ..Default::default()
//     // });
// }

pub fn character_movement(
    game_state: Res<GameState>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut map_query: Query<(&mut Tilemap, &mut Timer)>,
    mut player_query: Query<(&mut Position, &Render, &mut Player)>,
    mut camera_query: Query<(&Camera, &mut Transform)>,
    world_props: Res<WorldProps>,
) {
    if !game_state.map_loaded {
        return;
    }

    for (mut map, mut timer) in map_query.iter_mut() {
        timer.tick(time.delta_seconds());
        if !timer.finished() {
            continue;
        }

        for (mut position, render, mut _player) in player_query.iter_mut() {
            for key in keyboard_input.get_pressed() {
                for (_camera, mut camera_transform) in camera_query.iter_mut() {
                    // First we need to store our very current position.
                    let previous_position = *position;

                    // Of course we need to control where we are going to move our
                    // dwarf friend.
                    use KeyCode::*;
                    match key {
                        W | Numpad8 | Up | K => {
                            try_move_player(
                                &mut position,
                                &mut camera_transform.translation,
                                (0, 1),
                                world_props.tilemap_width,
                                world_props.tilemap_height,
                                world_props.tile_size,
                                &game_state.collisions,
                            );
                        }
                        A | Numpad4 | Left | H => {
                            try_move_player(
                                &mut position,
                                &mut camera_transform.translation,
                                (-1, 0),
                                world_props.tilemap_width,
                                world_props.tilemap_height,
                                world_props.tile_size,
                                &game_state.collisions,
                            );
                        }
                        S | Numpad2 | Down | J => {
                            try_move_player(
                                &mut position,
                                &mut camera_transform.translation,
                                (0, -1),
                                world_props.tilemap_width,
                                world_props.tilemap_height,
                                world_props.tile_size,
                                &game_state.collisions,
                            );
                        }
                        D | Numpad6 | Right | L => {
                            try_move_player(
                                &mut position,
                                &mut camera_transform.translation,
                                (1, 0),
                                world_props.tilemap_width,
                                world_props.tilemap_height,
                                world_props.tile_size,
                                &game_state.collisions,
                            );
                        }

                        // Numpad9 | U =>
                        // try_move_player(
                        //     &mut position,
                        //     &mut camera_transform.translation,
                        //     (1, 1),
                        //     world_props,
                        //     game_state,
                        // ),
                        // Numpad3 | M =>
                        // try_move_player(
                        //     &mut position,
                        //     &mut camera_transform.translation,
                        //     (1, -1),
                        //     world_props,
                        //     game_state,
                        // ),
                        // Numpad1 | N =>
                        // try_move_player(
                        //     &mut position,
                        //     &mut camera_transform.translation,
                        //     (-1, -1),
                        //     world_props,
                        //     game_state,
                        // ),
                        // Numpad7 | Y =>
                        // try_move_player(
                        //     &mut position,
                        //     &mut camera_transform.translation,
                        //     (-1, 1),
                        //     world_props,
                        //     game_state,
                        // ),
                        _ => {}
                    }

                    // Finally now we will move the sprite! ... Provided he had
                    // moved!
                    move_sprite(&mut map, previous_position, *position, render);
                    _player.add_fatigue();
                }
            }
        }
    }
}

pub fn try_move_player(
    position: &mut Position,
    camera_translation: &mut Vec3,
    delta_xy: (i32, i32),
    tilemap_width: i32,
    tilemap_height: i32,
    tile_size: i32,
    collisions: &HashSet<(i32, i32)>,
) {
    let new_pos = (position.x + delta_xy.0, position.y + delta_xy.1);
    if !collisions.contains(&new_pos)
        && new_pos.0 >= 0
        && new_pos.0 < tilemap_width
        && new_pos.1 >= 0
        && new_pos.1 < tilemap_height
    {
        position.x = position.x + delta_xy.0;
        position.y = position.y + delta_xy.1;
        camera_translation.x = camera_translation.x + (delta_xy.0 as f32 * tile_size as f32);
        camera_translation.y = camera_translation.y + (delta_xy.1 as f32 * tile_size as f32);
    }
}
