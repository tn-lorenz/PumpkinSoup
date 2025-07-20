use std::sync::Arc;

use async_trait::async_trait;
use dashmap::DashMap;
use futures::join;
use once_cell::sync::Lazy;
use pumpkin::{
    entity::player::Player,
    plugin::{
        EventHandler,
        player::player_interact_event::{InteractAction, PlayerInteractEvent},
    },
    server::Server,
};

use crate::damager::damager_state::ACTIVE_UUIDS;
use crate::util::player_util::PlayerUtil;
use pumpkin_api_macros::with_runtime;
use pumpkin_data::item::Item;
use pumpkin_world::item::ItemStack;
use uuid::Uuid;

pub struct SoupRightClickHandler;

pub static CONSUMED_SOUPS: Lazy<DashMap<Uuid, u32>> = Lazy::new(DashMap::new);
pub static ACCURATE_SOUPS: Lazy<DashMap<Uuid, u32>> = Lazy::new(DashMap::new);

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
        let active = ACTIVE_UUIDS.contains(&player.gameprofile.id);

        let held_item = player.inventory().held_item();
        {
            let held_item_guard = held_item.lock().await;
            if held_item_guard.get_item() != &Item::MUSHROOM_STEW {
                return;
            }
        }

        let current_health = player.living_entity.health.load();
        let current_food_level = player.hunger_manager.level.load();

        // TODO: Get max health instead
        if current_health == 20.0 {
            if player.is_hungry().await {
                join!(
                    player.set_food_level(current_food_level + 7),
                    replace_soup_with_bowl(player),
                );
            }
        } else {
            if active {
                let mut consumed_count = CONSUMED_SOUPS.entry(player.gameprofile.id).or_insert(0);
                *consumed_count += 1;

                if current_health <= 14.0 {
                    let mut accurate_count =
                        ACCURATE_SOUPS.entry(player.gameprofile.id).or_insert(0);
                    *accurate_count += 1;
                }
            }

            join!(
                player.set_health((current_health + 7.0).min(20.0)),
                replace_soup_with_bowl(player),
            );
        }
    }
}

pub async fn replace_soup_with_bowl(player: &Arc<Player>) {
    let bowl = ItemStack::new(1, &Item::BOWL);

    player
        .set_item(player.inventory().get_selected_slot().into(), bowl)
        .await;
}
