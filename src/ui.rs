use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

use super::player::Player;
use super::common_components::*;

pub fn ui_windows(mut egui_context: ResMut<EguiContext>, q_player: Query<(&Player, &Position, &Inventory)>) {
    let ctx = &mut egui_context.ctx;

    for (player, pos, inventory) in q_player.iter() {
        let player_name = format!("{}", player.name);
        let pos = format!("x:{} - y:{}", pos.x, pos.y);
        let health = format!("Health: {}", (player.health as i32).to_string());
        let thirst = format!("Thirst: {}", (player.thirst as i32).to_string());
        let hunger = format!("Hunger: {}", (player.hunger as i32).to_string());
        let temperature = format!("Temperature: {}", (player.temperature as i32).to_string());

        egui::Window::new(player_name).show(ctx, |ui| {
            ui.label(pos);
            ui.label(health);
            ui.label(thirst);
            ui.label(hunger);
            ui.label(temperature);
        });

        egui::Window::new("Inventory").show(ctx, |ui| {
            for (item, qty) in inventory.items.iter() {            
            ui.label(format!("{}: {}", item.name, qty));
            }
        });
    }
}

// use bevy::prelude::*;
// use bevy_megaui::{
//     megaui::{hash, Vector2},
//     MegaUiContext,
// };

// use super::player::Player;

// pub fn ui_windows(_world: &mut World, resources: &mut Resources) {
//     let mut ui = resources.get_thread_local_mut::<MegaUiContext>().unwrap();

//     let health = format!("Health: {}", 100);// player.health.to_string());
//     let thirst = format!("Thirst: {}", 100);//player.thirst.to_string());
//     let hunger = format!("Hunger: {}", 100);//player.hunger.to_string());

//     ui.draw_window(
//         hash!(),
//         Vector2::new(5.0, 5.0),
//         Vector2::new(200.0, 100.0),
//         None,
//         |ui| {
//             ui.label(None, "Player stats:");
//             ui.separator();
//             ui.label(None, &health);
//             ui.label(None, &thirst);
//             ui.label(None, &hunger);
//         },
//     );
// }

/////////////////////////////////////////////////////////////////////////////////

// struct ButtonMaterials {
//     normal: Handle<ColorMaterial>,
//     hovered: Handle<ColorMaterial>,
//     pressed: Handle<ColorMaterial>,
// }

// impl FromResources for ButtonMaterials {
//     fn from_resources(resources: &Resources) -> Self {
//         let mut materials = resources.get_mut::<Assets<ColorMaterial>>().unwrap();
//         ButtonMaterials {
//             normal: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
//             hovered: materials.add(Color::rgb(0.25, 0.25, 0.25).into()),
//             pressed: materials.add(Color::rgb(0.35, 0.75, 0.35).into()),
//         }
//     }
// }

// pub fn button_system(
//     button_materials: Res<ButtonMaterials>,
//     mut interaction_query: Query<
//         (&Interaction, &mut Handle<ColorMaterial>, &Children),
//         (Mutated<Interaction>, With<Button>),
//     >,
//     mut text_query: Query<&mut Text>,
// ) {
//     for (interaction, mut material, children) in interaction_query.iter_mut() {
//         let mut text = text_query.get_mut(children[0]).unwrap();
//         match *interaction {
//             Interaction::Clicked => {
//                 // text.sections[0].value = "Press".to_string();
//                 *material = button_materials.pressed.clone();
//             }
//             Interaction::Hovered => {
//                 // text.sections[0].value = "Hover".to_string();
//                 *material = button_materials.hovered.clone();
//             }
//             Interaction::None => {
//                 // text.sections[0].value = "Button".to_string();
//                 *material = button_materials.normal.clone();
//             }
//         }
//     }
// }

// pub fn setup(
//     commands: &mut Commands,
//     asset_server: Res<AssetServer>,
//     button_materials: Res<ButtonMaterials>,
// ) {
//     commands
//         // ui camera
//         .spawn(CameraUiBundle::default())
//         .spawn(ButtonBundle {
//             style: Style {
//                 size: Size::new(Val::Px(150.0), Val::Px(65.0)),
//                 // center button
//                 margin: Rect::all(Val::Auto),
//                 // horizontally center child text
//                 justify_content: JustifyContent::Center,
//                 // vertically center child text
//                 align_items: AlignItems::Center,
//                 ..Default::default()
//             },
//             material: button_materials.normal.clone(),
//             ..Default::default()
//         })
//         .with_children(|parent| {
//             parent.spawn(TextBundle {
//                 text: Text::with_section(
//                     "Button",
//                     TextStyle {
//                         font: asset_server.load("fonts/FiraSans-Bold.ttf"),
//                         font_size: 40.0,
//                         color: Color::rgb(0.9, 0.9, 0.9),
//                     },
//                     Default::default(),
//                 ),
//                 ..Default::default()
//             });
//         });
// }
