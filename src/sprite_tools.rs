use bevy_tilemap::prelude::*;

use super::common_components::*;

pub fn move_sprite(
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

    let previous_chuck_point = map.point_to_chunk_point((previous_position.x, previous_position.y));
    let chunk_point = map.point_to_chunk_point((position.x, position.y));

    if chunk_point != previous_chuck_point {
        map.spawn_chunk_containing_point((position.x, position.y))
            .unwrap();
        // map.despawn_chunk(previous_chuck_point).unwrap();
    }
}
