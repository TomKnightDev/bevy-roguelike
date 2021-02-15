use bevy::prelude::*;
use bevy_tilemap::prelude::*;

use super::common_components::*;
use super::game::*;

pub fn move_sprite(
    map: &mut Tilemap,
    previous_position: Position,
    position: Position,
    render: &Render,
) {
    // We need to first remove where we were prior.
    map.clear_tile((previous_position.x, previous_position.y), 2)
        .unwrap();
    // We then need to update where we are going!
    let mut tile = Tile::new((position.x, position.y), render.sprite_index);
    tile.z_order = render.z_order;

    map.insert_tile(tile).unwrap();

    let previous_chuck_point = map.point_to_chunk_point((previous_position.x, previous_position.y));
    let chunk_point = map.point_to_chunk_point((position.x, position.y));

    if chunk_point != previous_chuck_point {
        //Build list of chunks around player to render
        let mut new_chunk_points = Vec::new();

        for x in chunk_point.0 - 1..=chunk_point.0 + 1 {
            for y in chunk_point.1 - 1..=chunk_point.1 + 1 {
                map.spawn_chunk((x, y)).expect("Chunk failed to load");
                new_chunk_points.push((x, y));
            }
        }

        //Despawn any chunks not adjacent to player
        for x in previous_chuck_point.0 - 1..=previous_chuck_point.0 + 1 {
            for y in previous_chuck_point.1 - 1..=previous_chuck_point.1 + 1 {
                if !new_chunk_points.contains(&(x, y)) {
                    map.despawn_chunk((x, y)).expect("Chunk despawn failed");
                }
            }
        }
    }
}

#[derive(Default)]
pub struct SpriteChangedEventListenerState {
    my_event_reader: EventReader<SpriteChangeEvent>,
}
#[derive(Default)]
pub struct SpriteChangeEvent(pub Position);

pub fn sprite_change_event(
    events: Res<Events<SpriteChangeEvent>>,
    mut event_listener_state: ResMut<SpriteChangedEventListenerState>,
    // mut event_reader: ResMut<EventReader<SpriteChangeEvent>>,
    mut query: Query<&mut Tilemap>,
    asset_server: Res<AssetServer>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut game_state: ResMut<GameState>,
) {
    for ev in event_listener_state.my_event_reader.iter(&events) {
        for mut map in query.iter_mut() {
            let texture_atlas = texture_atlases.get(map.texture_atlas()).unwrap();
            let grass_0: Handle<Texture> = asset_server.get_handle("textures/terrain/grass_0.png");
            let grass_0_index = texture_atlas.get_texture_index(&grass_0).unwrap();

            // println!("{:?}", ev.0);

            map.clear_tile((ev.0.x, ev.0.y), 0).unwrap();

            let mut tile = Tile::new((ev.0.x, ev.0.y), grass_0_index);
            tile.z_order = 1;

            map.insert_tile(tile).unwrap();

            game_state.collisions.remove(&(ev.0.x, ev.0.y));
        }
    }
}

// pub fn change_sprite(map: &mut Tilemap, position: Position) {
//     map.clear_tile((position.x, position.y), 1).unwrap();

//     let mut tile = Tile::new((position.x, position.y), 0);
//     tile.z_order = 1;

//     map.insert_tile(tile).unwrap();
// }
