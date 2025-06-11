use pumpkin_api_macros::{plugin_impl, plugin_method}; 
use async_trait::async_trait; 
use pumpkin_api_macros::with_runtime;
//use pumpkin_inventory::equipment_slot::EquipmentSlot;
use pumpkin::{
    //inventory::player::player_inventory::PlayerInventory;
    plugin::{player::player_join::PlayerJoinEvent, Context, EventHandler, EventPriority},
    server::Server,
};
use pumpkin_util::text::{color::NamedColor, TextComponent};
use std::sync::Arc;

struct MyJoinHandler; 

#[with_runtime(global)]
#[async_trait]
impl EventHandler<PlayerJoinEvent> for MyJoinHandler {
    async fn handle_blocking(&self, _server: &Arc<Server>, event: &mut PlayerJoinEvent) {
        event.join_message =
            TextComponent::text(format!("Welcome, {}!", event.player.gameprofile.name))
                .color_named(NamedColor::Green);
    }
}

#[plugin_method] 
async fn on_load(&mut self, server: &Context) -> Result<(), String> {
    pumpkin::init_log!(); 

    log::info!("PumpkinSoup has been loaded.");
    Ok(())
}

#[plugin_impl]
pub struct Plugin {}

impl Plugin {
    pub fn new() -> Self {
        Plugin {}
    }
}

impl Default for Plugin {
    fn default() -> Self {
        Self::new()
    }
}