use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiSettings};
use egui::Pos2;

use super::common_components::*;
use super::player::Player;

pub fn ui_windows(
    mut egui_context: ResMut<EguiContext>,
    mut egui_settings: ResMut<EguiSettings>,
    mut q_player: Query<(&mut Player, &Position, &mut Inventory)>,
    windows: Res<Windows>,
    mut ev_inventory_button: ResMut<Events<InventoryButtonEvent>>,
) {
    let ctx = &mut egui_context.ctx;

    for (player, pos, inventory) in q_player.iter_mut() {
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
                match item {
                    ItemType::Water { name, consume: _ } => {
                        if ui.button(format!("{}: {}", name, qty)).clicked {
                            ev_inventory_button.send(InventoryButtonEvent(item.clone()));
                        };
                    }
                    ItemType::Wood { name } => {
                        if ui.button(format!("{}: {}", name, qty)).clicked {};
                    }
                }

                // ui.label(format!("{}: {}", item.name, qty));
            }
        });

        egui::Window::new("Settings")
            .default_pos(Pos2 {
                x: windows.get_primary().unwrap().width() - 140.0,
                y: 15.0,
            })
            .show(ctx, |ui| {
                ui.heading("Scale");
                if ui.button("Increase").clicked {
                    egui_settings.scale_factor += 0.5;
                } else if ui.button("Decrease").clicked && egui_settings.scale_factor > 1.0 {
                    egui_settings.scale_factor -= 0.5;
                }
            });
    }
}

#[derive(Default)]
pub struct InventoryButtonEventListenerState {
    my_event_reader: EventReader<InventoryButtonEvent>,
}
// #[derive(Default)]
pub struct InventoryButtonEvent(pub ItemType);

pub fn inventory_button_event(
    events: Res<Events<InventoryButtonEvent>>,
    mut event_listener_state: ResMut<InventoryButtonEventListenerState>,
    mut q_player: Query<(&mut Player, &mut Inventory)>,
) {
    for ev in event_listener_state.my_event_reader.iter(&events) {
        for (mut player, mut inventory) in q_player.iter_mut() {
            if player.consume_item(&ev.0) {
                // let item = inventory.items.entry(ev.0.clone()).borrow_mut();
                let item_count = inventory.items[&ev.0];

                if item_count == 1 {
                    inventory.items.remove_entry(&ev.0);
                } else {
                    let v = inventory.items.entry(ev.0.clone()).or_insert(1);
                    *v -= 1;
                }
            }
        }
    }
}
