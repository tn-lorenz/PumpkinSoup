use crate::TOKIO_RUNTIME;
use crate::damager_state::ACTIVE_UUIDS;
use pumpkin::entity::EntityBase;
use pumpkin::entity::player::Player;
use pumpkin_data::damage::DamageType;
use std::sync::Arc;
use tokio::time::{Duration, sleep};

// TODO: As soon as the `on_player_death` event is available, kill this task and remove the player from `ACTIVE_UUIDS` (need to implement more concise thread handling for that)
pub fn start_damage_loop(delay: Duration, player: Arc<Player>, damage: f32) {
    TOKIO_RUNTIME.spawn(run_task_timer(delay, player, damage));
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
