use std::sync::Arc;

use async_trait::async_trait;

use pumpkin::{
    entity::player::Player,
    plugin::{
        EventHandler,
        player::player_interact_event::{InteractAction, PlayerInteractEvent},
    },
    server::Server,
};

use pumpkin_api_macros::with_runtime;
use pumpkin_data::item::Item;
use pumpkin_world::item::ItemStack;

use crate::player_util::PlayerUtil;

pub struct SoupRightClickHandler;

#[with_runtime(global)]
#[async_trait]
impl EventHandler<PlayerInteractEvent> for SoupRightClickHandler {
    async fn handle_blocking(&self, _server: &Arc<Server>, event: &mut PlayerInteractEvent) {
        if matches!(
            event.action,
            InteractAction::LeftClickAir | InteractAction::LeftClickBlock
        ) {
            return;
        }

        let player = &event.player;

        let held_item = player.inventory().held_item();
        {
            let held_item_guard = held_item.lock().await;
            if held_item_guard.get_item() != &Item::MUSHROOM_STEW {
                return;
            }
        }

        let current_health = player.living_entity.health.load();
        let current_food_level = player.get_food_level().await;

        // TODO: Get max health instead
        if current_health == 20.0 {
            if player.is_hungry().await {
                player.set_food_level(current_food_level + 7).await;
                // player.set_saturation_level((20 - (current_food_level + 7)) as f32).await;
                replace_soup_with_bowl(player).await;
            }
        } else {
            player.set_health((current_health + 7.0).min(20.0)).await;
            replace_soup_with_bowl(player).await;
        }
    }
}

pub async fn replace_soup_with_bowl(player: &Arc<Player>) {
    let bowl = ItemStack::new(1, &Item::BOWL);

    player
        .set_item(player.inventory().get_selected_slot().into(), bowl)
        .await;
}
