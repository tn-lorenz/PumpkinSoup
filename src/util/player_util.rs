use async_trait::async_trait;
use futures::future::join_all;
use pumpkin::entity::player::Player;
use pumpkin_data::item::Item;
use pumpkin_world::item::ItemStack;
use std::sync::Arc;

#[async_trait]
#[allow(dead_code)]
pub(crate) trait PlayerUtil {
    async fn set_item(&self, slot: i16, item: ItemStack);
    async fn fill_inventory_with_soup(&self);
    async fn clear_inventory(&self);
    async fn remove_stack(&self, slot: usize) -> ItemStack;
    async fn get_food_level(&self) -> u8;
    async fn set_food_level(&self, level: u8);
    async fn is_hungry(&self) -> bool;
    async fn get_saturation_level(&self) -> f32;
    async fn set_saturation_level(&self, level: f32);
    async fn get_health(&self) -> f32;
}

#[async_trait]
#[allow(dead_code)]
impl PlayerUtil for Arc<Player> {
    async fn set_item(&self, slot: i16, mut item: ItemStack) {
        self.remove_stack(slot.try_into().unwrap()).await;
        self.inventory().insert_stack(slot, &mut item).await;
    }

    // TODO: Consider clearing off-hand and armor slots.
    async fn fill_inventory_with_soup(&self) {
        let soup = ItemStack::new(1, &Item::MUSHROOM_STEW);
        let futures = (0..36).map(|i| self.set_item(i, soup));
        join_all(futures).await;
    }

    // TODO: Also clear off-hand and armor slots.
    async fn clear_inventory(&self) {
        let futures = (0..36).map(|i| self.remove_stack(i));
        join_all(futures).await;
    }

    async fn remove_stack(&self, slot: usize) -> ItemStack {
        if slot < self.inventory().main_inventory.len() {
            let mut removed = ItemStack::EMPTY;
            let mut guard = self.inventory().main_inventory[slot].lock().await;
            std::mem::swap(&mut removed, &mut *guard);
            removed
        } else {
            let slot = self.inventory().equipment_slots.get(&slot).unwrap();
            self.inventory()
                .entity_equipment
                .lock()
                .await
                .put(slot, ItemStack::EMPTY)
                .await
        }
    }

    async fn get_food_level(&self) -> u8 {
        self.hunger_manager.level.load()
    }

    async fn set_food_level(&self, level: u8) {
        self.hunger_manager.level.store(level.clamp(0, 20));
        self.send_health().await;
    }

    async fn is_hungry(&self) -> bool {
        self.get_food_level().await < 20
    }

    async fn get_saturation_level(&self) -> f32 {
        self.hunger_manager.saturation.load()
    }

    // TODO: Find out the actual max, this makes no sense
    async fn set_saturation_level(&self, level: f32) {
        self.hunger_manager.saturation.store(level.clamp(0.0, 20.0));
        self.send_health().await;
    }

    async fn get_health(&self) -> f32 {
        self.living_entity.health.load()
    }
}
