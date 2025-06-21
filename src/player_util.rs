use async_trait::async_trait;
use pumpkin::entity::player::Player;
use pumpkin_data::item::Item;
use pumpkin_world::item::ItemStack;
use std::sync::Arc;

#[async_trait]
pub(crate) trait PlayerUtil {
    async fn set_item(&self, slot: i16, item: ItemStack);
    async fn fill_inventory_with_soup(&self);
    async fn clear_inventory(&self);
    async fn remove_stack(&self, slot: usize) -> ItemStack;
}

#[async_trait]
impl PlayerUtil for Arc<Player> {
    async fn set_item(&self, slot: i16, mut item: ItemStack) {
        self.remove_stack(slot.try_into().unwrap()).await;
        self.inventory().insert_stack(slot, &mut item).await;
    }

    async fn clear_inventory(&self) {
        for i in 0..35 {
            self.remove_stack(i).await;
        }
    }

    async fn fill_inventory_with_soup(&self) {
        let soup = ItemStack::new(1, &Item::MUSHROOM_STEW);

        for i in 0..35 {
            self.set_item(i, soup).await;
        }
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
}
