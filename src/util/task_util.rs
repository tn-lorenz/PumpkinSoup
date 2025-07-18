use crate::TOKIO_RUNTIME;
use crate::commands::soup_kit_command::RECRAFT_AMOUNT;
use crate::damager_state::ACTIVE_UUIDS;
use crate::listeners::soup_rightclick::{ACCURATE_SOUPS, CONSUMED_SOUPS};
use dashmap::DashMap;
use once_cell::sync::Lazy;
use pumpkin::entity::EntityBase;
use pumpkin::entity::player::Player;
use pumpkin_data::damage::DamageType;
use pumpkin_util::text::TextComponent;
use std::sync::Arc;
use tokio::time::{Duration, sleep};
use uuid::Uuid;

pub static DAMAGE_TAKEN: Lazy<DashMap<Uuid, f32>> = Lazy::new(DashMap::new);

// TODO: As soon as the `on_player_death` event is available, kill this task and remove the player from `ACTIVE_UUIDS` (need to implement more concise thread handling for that)
pub fn start_damage_loop(delay: Duration, player: Arc<Player>, damage: f32) {
    TOKIO_RUNTIME.spawn(run_task_timer(delay, player, damage));
}

pub(crate) async fn run_task_timer(delay: Duration, player: Arc<Player>, damage: f32) {
    CONSUMED_SOUPS.insert(player.gameprofile.id, 0);
    ACCURATE_SOUPS.insert(player.gameprofile.id, 0);
    DAMAGE_TAKEN.insert(player.gameprofile.id, 0.0);

    loop {
        if ACTIVE_UUIDS.contains(&player.gameprofile.id) {
            sleep(delay).await;
            execute_task(Arc::clone(&player), damage).await;
        } else {
            break;
        }

        if let Some(count) = CONSUMED_SOUPS.get(&player.gameprofile.id) {
            if *count >= get_consumable_count(player.clone()).await {
                break;
            }
        }
    }
    let count = CONSUMED_SOUPS.get(&player.gameprofile.id).unwrap();
    let damage_count = DAMAGE_TAKEN.get(&player.gameprofile.id).unwrap();
    let accurate_soups = ACCURATE_SOUPS.get(&player.gameprofile.id).unwrap();

    if *accurate_soups == get_consumable_count(player.clone()).await {
        print_congratulation_msg(player.clone()).await;
    }
    print_completion_msg(player.clone(), *count, *damage_count, *accurate_soups).await;
}

async fn execute_task(player: Arc<Player>, damage: f32) {
    player.damage(damage, DamageType::GENERIC).await;

    let id = player.gameprofile.id;

    if let Some(mut entry) = DAMAGE_TAKEN.get_mut(&id) {
        *entry += damage;
    } else {
        DAMAGE_TAKEN.insert(id, damage);
    }
}

async fn get_consumable_count(player: Arc<Player>) -> u32 {
    let recraft_amount = RECRAFT_AMOUNT.get(&player.gameprofile.id).unwrap();

    if *recraft_amount > 0 {
        32 + *recraft_amount as u32
    } else {
        35
    }
}

async fn print_congratulation_msg(player: Arc<Player>) {
    player
        .send_system_message(&TextComponent::text("§6§l~ Congratulations! ~"))
        .await;
    player
        .send_system_message(&TextComponent::text("§5You completed the damager!"))
        .await;
}

async fn print_completion_msg(
    player: Arc<Player>,
    count: u32,
    damage_count: f32,
    accurate_soups: u32,
) {
    let accuracy = if count > 0 {
        let ratio = accurate_soups as f64 / count as f64;
        (ratio * 100.0).round() as u32
    } else {
        0
    };

    player
        .send_system_message(&TextComponent::text(
            "§8§m                                     ",
        ))
        .await;
    player
        .send_system_message(&TextComponent::text(format!("§7Soups slurped: §a{count}")))
        .await;
    player
        .send_system_message(&TextComponent::text(format!(
            "§7Damage taken: §4{} §4❤",
            damage_count / 2.0
        )))
        .await;
    player
        .send_system_message(&TextComponent::text(format!(
            "§7Soup accuracy: §6{accuracy:.2}%"
        )))
        .await;
    player
        .send_system_message(&TextComponent::text(
            "§8§m                                     ",
        ))
        .await;
}
