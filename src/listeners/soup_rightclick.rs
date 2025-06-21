use std::sync::Arc;

use async_trait::async_trait;
use tokio::time::{Duration, sleep};

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

        let old_health = player.living_entity.health.load();

        if old_health < 20.0 {
            let new_health = (old_health + 7.0).min(20.0);
            player.set_health(new_health).await;
            replace_soup_with_bowl(player).await;
        }
    }
}

pub async fn replace_soup_with_bowl(player: &Arc<Player>) {
    sleep(Duration::from_millis(50)).await;

    let bowl = ItemStack::new(1, &Item::BOWL);

    player
        .set_item(player.inventory().get_selected_slot().into(), bowl)
        .await;
}
