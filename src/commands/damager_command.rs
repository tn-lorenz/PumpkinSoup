use std::sync::Arc;
use tokio::time::Duration;
use uuid::Uuid;

use async_trait::async_trait;

use crate::damager::damager_state::ACTIVE_UUIDS;
use crate::util::task_util::start_damage_loop;

use crate::commands::DamagerArgumentConsumer;
use crate::config::DAMAGERS;
use pumpkin::command::dispatcher::CommandError::CommandFailed;
use pumpkin::{command::{
    CommandExecutor, CommandSender,
    args::{Arg, ConsumedArgs},
    dispatcher::{CommandError, CommandError::InvalidConsumption},
    tree::{
        CommandTree,
        builder::{argument, require},
    },
}, entity::player::Player, run_task_later, run_task_timer, server::Server};
use pumpkin::entity::EntityBase;
use pumpkin_data::damage::DamageType;
use pumpkin_util::text::TextComponent;
use crate::util::global::get_context;

const NAMES: [&str; 2] = ["damager", "dmg"];
const DESCRIPTION: &str =
    "Initiate a damager task that continously damages you according to the chosen difficulty.";
const DAMAGER_ARG_NAME: &str = "difficulty";
const MSG_NOT_PLAYER: &str = "Only players may use this command.";

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
        _server: &Server,
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

        let _ = handle_input(player, Option::from(damager_type), uuid).await;

        Ok(())
    }
}

#[async_trait]
impl CommandExecutor for DamagerExecutorNoArg {
    async fn execute<'a>(
        &self,
        sender: &mut CommandSender,
        server: &Server,
        _args: &ConsumedArgs<'a>,
    ) -> Result<(), CommandError> {
        let target = sender.as_player().ok_or(CommandError::InvalidRequirement)?;

        let uuid = target.gameprofile.id;

        if ACTIVE_UUIDS.contains(&uuid) {
            ACTIVE_UUIDS.remove(&uuid);
        }

        let target_clone = Arc::clone(&target);

        run_task_timer!(get_context().server.clone(), 10, {
            let target = Arc::clone(&target_clone);
            target.damage(4.0, DamageType::GENERIC).await;
        });

        run_task_later!(server, 20 * 3, {
            target.send_system_message(&TextComponent::text("This message has been printed after 3 seconds!")).await;
        });

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
    player: &Arc<Player>,
    input: Option<String>,
    uuid: Uuid,
) -> Result<(), CommandError> {
    let Some(s) = input else {
        log::error!("Damager input is None. How did you even do this?");
        return Ok(());
    };

    let maybe_damager = DAMAGERS
        .iter()
        .find(|d| d.name.eq_ignore_ascii_case(&s))
        .map(|d| d.clone());

    match maybe_damager {
        Some(damager) => {
            toggle_damager(uuid);

            if ACTIVE_UUIDS.contains(&uuid) {
                start_damage_loop(
                    Duration::from_millis(damager.delay as u64),
                    Arc::clone(player),
                    damager.damage as f32,
                );
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
