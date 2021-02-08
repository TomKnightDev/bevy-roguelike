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
    // events to get cursor position
    ev_cursor: Res<Events<CursorMoved>>,
    mut evr_cursor: Local<EventReader<CursorMoved>>,
    // need to get window dimensions
    wnds: Res<Windows>,
    // query to get camera transform
    q_camera: Query<&Transform, With<MainCamera>>,
    world_props: Res<WorldProps>,
    btn: Res<Input<MouseButton>>,
    mut mouse_state: ResMut<MouseState>,
    mut game_state: ResMut<GameState>,
    mut q_player: Query<(&Player, &mut Inventory)>,
    mut ev_sprite_changed: ResMut<Events<SpriteChangeEvent>>,
) {
    // assuming there is exactly one main camera entity, so this is OK
    //let camera_transform = q_camera.iter().next().unwrap();

    for camera_transform in q_camera.iter() {
        for ev in evr_cursor.iter(&ev_cursor) {
            // get the size of the window that the event is for
            let wnd = wnds.get(ev.id).unwrap();
            let size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

            // the default orthographic projection is in pixels from the center;
            // just undo the translation
            let p = ev.position - size / 2.0;

            // apply the camera transform
            let pos_wld = camera_transform.compute_matrix() * p.extend(0.0).extend(1.0);
            mouse_state.pos = Position {
                x: (pos_wld.x / world_props.tile_size as f32) as i32,
                y: (pos_wld.y / world_props.tile_size as f32) as i32,
            };
            // mouse_state.pos_y = as i32;
        }
    }

    if btn.just_pressed(MouseButton::Left) {
        println!("World coords: {}/{}", mouse_state.pos.x, mouse_state.pos.y);

        //Harvestable
        let hv = game_state
            .harvestable_tiles
            .iter_mut()
            .filter(|h| h.pos == mouse_state.pos)
            .next();

        if !hv.is_none() {
            let i = hv.unwrap().items.iter_mut().next().unwrap();

            for (_, mut inventory) in q_player.iter_mut() {
                // if !q_player.iter_mut().next().is_none() {
                // eprintln!("Got {}", i);
                // let mut p = q_player.iter_mut().next().unwrap().1;
                    eprintln!("BEFORE");
                if inventory.items.get(&i).is_none() {
                    eprintln!("NONE");
                    inventory.items.insert(i.clone(), 1);
                } else {
                    *inventory.items.get_mut(i).unwrap() += 1;
                }
                // *item_inv += 1;
            }
            //Change tile
            ev_sprite_changed.send(SpriteChangeEvent(mouse_state.pos));
            // }
        }
    }
}
