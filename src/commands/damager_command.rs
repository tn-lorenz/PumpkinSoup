use std::sync::Arc;
use uuid::Uuid;

use crate::damager::damager_state::ACTIVE_UUIDS;
use async_trait::async_trait;
use dashmap::DashMap;
use once_cell::sync::Lazy;

use crate::commands::DamagerArgumentConsumer;
use crate::commands::soup_kit_command::RECRAFT_AMOUNT;
use crate::config::DAMAGERS;
use crate::listeners::soup_rightclick::{ACCURATE_SOUPS, CONSUMED_SOUPS};
use pumpkin::command::dispatcher::CommandError::CommandFailed;
use pumpkin::entity::EntityBase;
use pumpkin::{
    command::{
        CommandExecutor, CommandSender,
        args::{Arg, ConsumedArgs},
        dispatcher::{CommandError, CommandError::InvalidConsumption},
        tree::{
            CommandTree,
            builder::{argument, require},
        },
    },
    entity::player::Player,
    run_task_timer,
    server::Server,
};
use pumpkin_data::damage::DamageType;
use pumpkin_util::text::TextComponent;
use pumpkin_util::text::color::{Color, NamedColor, RGBColor};

const NAMES: [&str; 2] = ["damager", "dmg"];
const DESCRIPTION: &str =
    "Initiate a damager task that continously damages you according to the chosen difficulty.";
const DAMAGER_ARG_NAME: &str = "difficulty";
const MSG_NOT_PLAYER: &str = "Only players may use this command.";

pub static DAMAGE_TAKEN: Lazy<DashMap<Uuid, f32>> = Lazy::new(DashMap::new);

pub fn build_invalid_arg_msg() -> String {
    use std::fmt::Write;

    let mut msg = String::from("Invalid argument. Possible options are: ");
    let mut first = true;

    for damager in DAMAGERS.iter() {
        if !first {
            let _ = write!(msg, ", ");
        }
        let _ = write!(msg, "{}", damager.name);
        first = false;
    }

    msg
}

struct DamagerExecutorWithArg;
struct DamagerExecutorNoArg;

#[async_trait]
impl CommandExecutor for DamagerExecutorWithArg {
    async fn execute<'a>(
        &self,
        sender: &mut CommandSender,
        server: &Server,
        args: &ConsumedArgs<'a>,
    ) -> Result<(), CommandError> {
        let Some(Arg::Simple(input)) = args.get(DAMAGER_ARG_NAME) else {
            return Err(InvalidConsumption(Some(DAMAGER_ARG_NAME.into())));
        };

        let CommandSender::Player(player) = sender else {
            sender
                .send_message(
                    TextComponent::text(MSG_NOT_PLAYER)
                        .color_named(pumpkin_util::text::color::NamedColor::Red),
                )
                .await;

            return Err(CommandFailed(Box::new(TextComponent::text(MSG_NOT_PLAYER))));
        };

        let damager_type = input.to_string();

        let uuid = player.gameprofile.id;

        let _ = handle_input(player.clone(), Option::from(damager_type), uuid, server).await;

        Ok(())
    }
}

#[async_trait]
impl CommandExecutor for DamagerExecutorNoArg {
    async fn execute<'a>(
        &self,
        sender: &mut CommandSender,
        _server: &Server,
        _args: &ConsumedArgs<'a>,
    ) -> Result<(), CommandError> {
        let target = sender.as_player().ok_or(CommandError::InvalidRequirement)?;

        let uuid = target.gameprofile.id;

        if ACTIVE_UUIDS.contains(&uuid) {
            ACTIVE_UUIDS.remove(&uuid);
        }

        Ok(())
    }
}

fn toggle_damager(uuid: Uuid) {
    if ACTIVE_UUIDS.contains(&uuid) {
        ACTIVE_UUIDS.remove(&uuid);
    } else {
        ACTIVE_UUIDS.insert(uuid);
    }
}

pub(crate) async fn handle_input(
    player: Arc<Player>,
    input: Option<String>,
    uuid: Uuid,
    server: &Server,
) -> Result<(), CommandError> {
    let Some(s) = input else {
        log::error!("Damager input is None. How did you even do this?");
        return Ok(());
    };

    let maybe_damager = DAMAGERS
        .iter()
        .find(|d| d.name.eq_ignore_ascii_case(&s))
        .map(|d| d.clone());

    let player_clone = player.clone();

    match maybe_damager {
        Some(damager) => {
            toggle_damager(uuid);

            if ACTIVE_UUIDS.contains(&uuid) {
                CONSUMED_SOUPS.insert(uuid, 0);
                ACCURATE_SOUPS.insert(uuid, 0);
                DAMAGE_TAKEN.insert(uuid, 0.0);

                // TODO: reintroduce fetching delay time from .toml
                run_task_timer!(server, ticks = 10, |handle| {
                    let player = player.clone();

                    async move {
                        let new_health = player.living_entity.health.load() - damager.damage as f32;

                        if new_health <= 0.0 {
                            player.living_entity.health.store(0.0);
                            handle.cancel();
                            ACTIVE_UUIDS.remove(&uuid);
                            return;
                        }

                        if let Some(count) = CONSUMED_SOUPS.get(&uuid)
                            && *count >= get_consumable_count(&player.clone()).await
                        {
                            return;
                        }

                        player
                            .damage(damager.damage as f32, DamageType::GENERIC)
                            .await;

                        if let Some(mut entry) = DAMAGE_TAKEN.get_mut(&uuid) {
                            *entry += damager.damage as f32;
                        } else {
                            DAMAGE_TAKEN.insert(uuid, damager.damage as f32);
                        }
                    }
                });

                let count = CONSUMED_SOUPS.get(&uuid).unwrap();
                let damage_count = DAMAGE_TAKEN.get(&uuid).unwrap();
                let accurate_soups = ACCURATE_SOUPS.get(&uuid).unwrap();
                let perfect_run = *accurate_soups == get_consumable_count(&player_clone).await;

                print_congratulation_msg(&player_clone, perfect_run).await;
                print_completion_msg(&player_clone, *count, *damage_count, *accurate_soups).await;
            }
            Ok(())
        }
        None => Err(CommandFailed(Box::new(TextComponent::text(
            build_invalid_arg_msg(),
        )))),
    }
}

pub fn init_command_tree() -> CommandTree {
    CommandTree::new(NAMES, DESCRIPTION)
        .then(argument(DAMAGER_ARG_NAME, DamagerArgumentConsumer).execute(DamagerExecutorWithArg))
        .then(require(|sender| sender.is_player()).execute(DamagerExecutorNoArg))
}

async fn get_consumable_count(player: &Arc<Player>) -> u32 {
    let recraft_amount = RECRAFT_AMOUNT.get(&player.gameprofile.id).unwrap();

    if *recraft_amount > 0 {
        32 + *recraft_amount as u32
    } else {
        35
    }
}

async fn print_congratulation_msg(player: &Arc<Player>, perfect_run: bool) {
    let state = if perfect_run { "completed" } else { "survived" };
    player
        .send_system_message(
            &TextComponent::text("-=Congratulations!=-")
                .color(Color::Rgb(RGBColor::new(123, 223, 242)))
                .bold(),
        )
        .await;
    player
        .send_system_message(
            &TextComponent::text(format!("You {state} the damager!"))
                .color(Color::Rgb(RGBColor::new(123, 223, 242))),
        )
        .await;
}

async fn print_completion_msg(
    player: &Arc<Player>,
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
        .send_system_message(
            &TextComponent::text("                                     ")
                .color(Color::Rgb(RGBColor::new(89, 212, 250)))
                .strikethrough(),
        )
        .await;
    player
        .send_system_message(
            &TextComponent::text("Soups slurped: ")
                .color_named(NamedColor::Gray)
                .add_child(TextComponent::text(format!("{count}")).color_named(NamedColor::Green)),
        )
        .await;
    player
        .send_system_message(
            &TextComponent::text("Damage taken: ")
                .color_named(NamedColor::Gray)
                .add_child(
                    TextComponent::text(format!("{:.1}", damage_count / 2.0))
                        .color_named(NamedColor::DarkRed),
                )
                .add_child(TextComponent::text("❤").color_named(NamedColor::DarkRed)),
        )
        .await;
    player
        .send_system_message(
            &TextComponent::text("Soup accuracy: ")
                .color_named(NamedColor::Gray)
                .add_child(
                    TextComponent::text(format!("{accuracy:.2}%")).color_named(NamedColor::Gold),
                ),
        )
        .await;
    player
        .send_system_message(
            &TextComponent::text("§m                                     ")
                .color(Color::Rgb(RGBColor::new(89, 212, 250)))
                .strikethrough(),
        )
        .await;
}
