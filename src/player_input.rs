use bevy::prelude::*; //, utils::tracing::Instrument};
                      // use std::collections::HashMap;

use super::common_components::*;
use super::game::*;
use super::player::*;
use super::sprite_tools::*;
use super::world::*;

pub struct MainCamera;

#[derive(Default, Clone)]
pub struct MouseState {
    pub pos: Position,
}

pub fn my_cursor_system(
    commands: &mut Commands,
    ev_cursor: Res<Events<CursorMoved>>,
    mut evr_cursor: Local<EventReader<CursorMoved>>,
    wnds: Res<Windows>,
    mut q_camera: Query<&mut Transform, With<MainCamera>>,
    world_props: Res<WorldProps>,
    mut mouse_state: ResMut<MouseState>,
    mut game_state: ResMut<GameState>,
    mut q_player: Query<(&Player, &mut Inventory, &Position)>,
    mut ev_sprite_changed: ResMut<Events<SpriteChangeEvent>>,
    mut q_cursor: Query<(&MouseState, &mut Transform, Entity)>,
    key: Res<Input<KeyCode>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // assuming there is exactly one main camera entity, so this is OK
    //let camera_transform = q_camera.iter().next().unwrap();

    if key.just_pressed(KeyCode::Escape) {
        for (_, _, e) in &mut q_cursor.iter_mut() {
            commands.despawn(e);

            commands
                .spawn(SpriteBundle {
                    material: materials.add(asset_server.load("textures/cursor.png").into()), //materials.wall_horizontal_material.clone(),
                    transform: Transform {
                        translation: Vec3::new(
                            mouse_state.pos.x as f32,
                            mouse_state.pos.y as f32,
                            5.0,
                        ),
                        scale: Vec3::new(1.0, 1.0, 1.0),
                        rotation: Quat::identity(),
                    },
                    ..Default::default()
                })
                .with(MouseState {
                    pos: mouse_state.pos,
                });
        }
    }

    for mut camera_transform in q_camera.iter_mut() {
        if key.just_pressed(KeyCode::E) {
            camera_transform.scale = Vec3::new(
                f32::max(0.1, camera_transform.scale.x - 0.1),
                f32::max(0.1, camera_transform.scale.y - 0.1),
                camera_transform.scale.z,
            );
        } else if key.just_pressed(KeyCode::Q) && camera_transform.scale.x < 1.0 {
            camera_transform.scale = Vec3::new(
                f32::min(1.0, camera_transform.scale.x + 0.1),
                f32::min(1.0, camera_transform.scale.y + 0.1),
                camera_transform.scale.z,
            );
        }

        if key.just_pressed(KeyCode::PageDown) {
            camera_transform.translation.z -= 1.0;
        } else if key.just_pressed(KeyCode::PageUp) {
            camera_transform.translation.z += 1.0;
        }

        let mut p_pos = Position::default();
        for (_, _, pos) in q_player.iter_mut() {
            p_pos = Position { x: pos.x, y: pos.y };
        }

        for ev in evr_cursor.iter(&ev_cursor) {
            // get the size of the window that the event is for
            let wnd = wnds.get(ev.id).unwrap();
            let size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

            // the default orthographic projection is in pixels from the center;
            // just undo the translation
            let p = ev.position - size / 2.0;

            // apply the camera transform
            let pos_wld = camera_transform.compute_matrix() * p.extend(0.0).extend(1.0);

            //set the mouse cursor position
            let x = if pos_wld.x as i32 / world_props.tile_size > p_pos.x {
                p_pos.x + 1
            } else if (pos_wld.x as i32 / world_props.tile_size) < p_pos.x {
                p_pos.x - 1
            } else {
                p_pos.x
            };

            let y = if pos_wld.y as i32 / world_props.tile_size > p_pos.y {
                p_pos.y + 1
            } else if (pos_wld.y as i32 / world_props.tile_size) < p_pos.y {
                p_pos.y - 1
            } else {
                p_pos.y
            };

            if x != p_pos.x || y != p_pos.y {
                mouse_state.pos = Position { x: x, y: y };

                for (_, mut trans, _) in &mut q_cursor.iter_mut() {
                    trans.translation = Vec3::new(
                        (mouse_state.pos.x * world_props.tile_size) as f32 + 4.0,
                        (mouse_state.pos.y * world_props.tile_size) as f32 + 4.0,
                        camera_transform.translation.z - 0.5,
                    );
                }
            }
            // mouse_state.pos = Position {
            //     x: (pos_wld.x / world_props.tile_size as f32) as i32,
            //     y: (pos_wld.y / world_props.tile_size as f32) as i32,
            // };

            // for (_, mut trans, _) in &mut q_cursor.iter_mut() {
            //     trans.translation = Vec3::new(
            //         (mouse_state.pos.x * world_props.tile_size) as f32 + 4.0,
            //         (mouse_state.pos.y * world_props.tile_size) as f32 + 4.0,
            //         camera_transform.translation.z - 0.5,
            //     );
            // }
        }
    }

    // if btn.just_pressed(MouseButton::Left) {
    if key.just_pressed(KeyCode::F) {
        println!("World coords: {}/{}", mouse_state.pos.x, mouse_state.pos.y);

        //Harvestable
        let hv = game_state
            .harvestable_tiles
            .iter()
            .filter(|h| h.pos == mouse_state.pos)
            .next();

        if !hv.is_none() {
            let hi = hv.unwrap();
            let i = hi.items.iter().next().unwrap();
            let pos = hi.pos;

            for (_, mut inventory, _) in q_player.iter_mut() {
                // inventory.items.keys().filter(|item| item == i);

                if inventory.items.contains_key(i) {
                    *inventory.items.get_mut(i).unwrap() += 1;
                } else {
                    inventory.items.insert(i.clone(), 1);
                }
            }

            let index = game_state
                .harvestable_tiles
                .iter()
                .position(|p| p.pos == pos)
                .unwrap();

            eprintln!("{},{:?}", index, pos);

            game_state.harvestable_tiles.remove(index);

            //Change tile
            ev_sprite_changed.send(SpriteChangeEvent(mouse_state.pos));
            // }
        }
    }
}

pub fn input_setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let texture_handle = asset_server.load("textures/cursor.png");

    commands
        .spawn(SpriteBundle {
            material: materials.add(texture_handle.into()),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 5.0),
                scale: Vec3::new(1.0, 1.0, 1.0),
                rotation: Quat::identity(),
            },
            ..Default::default()
        })
        .with(MouseState {
            pos: Position { x: 0, y: 0 },
        });
}
