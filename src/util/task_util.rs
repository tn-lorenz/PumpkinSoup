use crate::damager_state::ACTIVE_UUIDS;
use pumpkin::entity::EntityBase;
use pumpkin::entity::player::Player;
use pumpkin_data::damage::DamageType;
use std::{sync::Arc, thread};
use tokio::runtime::Runtime;
use tokio::time::{Duration, sleep};

pub fn start_damage_loop(delay: Duration, player: Arc<Player>, damage: f32) {
    thread::spawn(move || {
        let rt = Runtime::new().expect("Failed to create Tokio Runtime");

        rt.block_on(async move {
            run_task_timer(delay, player, damage).await;
        });
    });
}

pub(crate) async fn run_task_timer(delay: Duration, player: Arc<Player>, damage: f32) {
    loop {
        if ACTIVE_UUIDS.contains(&player.gameprofile.id) {
            sleep(delay).await;
            execute_task(Arc::clone(&player), damage).await;
        } else {
            break;
        }
    }
}

async fn execute_task(player: Arc<Player>, damage: f32) {
    player.damage(damage, DamageType::GENERIC).await;
}
