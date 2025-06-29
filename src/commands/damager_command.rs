use std::{str::FromStr, sync::Arc};
use tokio::time::Duration;
use uuid::Uuid;

use async_trait::async_trait;

use crate::{damager_state_manager::ACTIVE_UUIDS, task_util::start_damage_loop};

use pumpkin::{
    command::{
        CommandExecutor, CommandSender,
        args::{Arg, ConsumedArgs, simple::SimpleArgConsumer},
        dispatcher::{CommandError, CommandError::InvalidConsumption},
        tree::{
            CommandTree,
            builder::{argument, require},
        },
    },
    entity::player::Player,
    server::Server,
};

use pumpkin_util::text::TextComponent;

const NAMES: [&str; 2] = ["damager", "dmg"];
const DESCRIPTION: &str =
    "Initiate a damager task that continously damages you according to the chosen difficulty.";
const DAMAGER_ARG_NAME: &str = "difficulty";
const MSG_INVALID_ARG: &str =
    "Invalid argument. Possible options are: easy, medium, hard, extreme, calamity";
const MSG_NOT_PLAYER: &str = "Only players may use this command.";

enum Damager {
    Easy,
    Medium,
    Hard,
    Extreme,
    Calamity,
}
#[allow(dead_code)]
impl Damager {
    pub fn get_name(&self) -> &'static str {
        match self {
            Damager::Easy => "Easy",
            Damager::Medium => "Medium",
            Damager::Hard => "Hard",
            Damager::Extreme => "Extreme",
            Damager::Calamity => "Calamity",
        }
    }

    pub fn get_damage(&self) -> f32 {
        match self {
            Damager::Easy => 4.0,
            Damager::Medium => 5.0,
            Damager::Hard => 7.0,
            Damager::Extreme => 9.0,
            Damager::Calamity => 12.0,
        }
    }
}

impl FromStr for Damager {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "easy" => Ok(Damager::Easy),
            "medium" => Ok(Damager::Medium),
            "hard" => Ok(Damager::Hard),
            "extreme" => Ok(Damager::Extreme),
            "calamity" => Ok(Damager::Calamity),
            _ => Err(()),
        }
    }
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

            return Err(CommandError::GeneralCommandIssue(MSG_NOT_PLAYER.into()));
        };

        let damager_type: String = input
            .parse::<String>()
            .map_err(|_| CommandError::GeneralCommandIssue(MSG_INVALID_ARG.into()))?;

        let uuid = player.gameprofile.id;

        handle_input(player, Option::from(damager_type), uuid).await;

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

pub(crate) async fn handle_input(player: &Arc<Player>, input: Option<String>, uuid: Uuid) {
    let Some(s) = input else {
        log::error!("Damager input is None. How did you even do this?");
        return;
    };

    match Damager::from_str(&s) {
        Ok(damager) => {
            toggle_damager(uuid);

            if ACTIVE_UUIDS.contains(&uuid) {
                start_damage_loop(
                    Duration::from_millis(500),
                    Arc::clone(player),
                    damager.get_damage(),
                );
            }
        }
        Err(_) => {
            log::warn!("This damager type does not exist: '{s}'");
        }
    }
}

pub fn init_command_tree() -> CommandTree {
    CommandTree::new(NAMES, DESCRIPTION)
        .then(argument(DAMAGER_ARG_NAME, SimpleArgConsumer).execute(DamagerExecutorWithArg))
        .then(require(|sender| sender.is_player()).execute(DamagerExecutorNoArg))
}
