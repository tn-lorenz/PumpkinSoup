use async_trait::async_trait;
use pumpkin::{
    command::{
        args::{Arg, ConsumedArgs, simple::SimpleArgConsumer},
        dispatcher::{CommandError, CommandError::InvalidConsumption},
        tree::{CommandTree, builder::argument},
        {CommandExecutor, CommandSender},
    },
    entity::player::Player,
    server::Server,
};
use pumpkin_util::text::TextComponent;
use std::str::FromStr;
use std::sync::Arc;
use tokio::time::Duration;

use crate::task_util::start_damage_loop;

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

struct DamagerExecutor;

#[async_trait]
impl CommandExecutor for DamagerExecutor {
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

        handle_input(player, Option::from(damager_type)).await;

        Ok(())
    }
}

pub(crate) async fn handle_input(player: &Arc<Player>, input: Option<String>) {
    match input {
        Some(s) => match Damager::from_str(&s) {
            Ok(damager) => match damager {
                Damager::Easy => {
                    start_damage_loop(
                        Duration::from_millis(500),
                        player.clone(),
                        Damager::Easy.get_damage(),
                    );
                }
                Damager::Medium => {
                    start_damage_loop(
                        Duration::from_millis(500),
                        player.clone(),
                        Damager::Medium.get_damage(),
                    );
                }
                Damager::Hard => {
                    start_damage_loop(
                        Duration::from_millis(500),
                        player.clone(),
                        Damager::Hard.get_damage(),
                    );
                }
                Damager::Extreme => {
                    start_damage_loop(
                        Duration::from_millis(500),
                        player.clone(),
                        Damager::Extreme.get_damage(),
                    );
                }
                Damager::Calamity => {
                    start_damage_loop(
                        Duration::from_millis(500),
                        player.clone(),
                        Damager::Calamity.get_damage(),
                    );
                }
            },
            Err(_) => {
                log::warn!("This damager type does not exist: '{s}'");
            }
        },
        None => {
            log::error!(
                "The damager argument is of type `None`. Idk how you even managed to do that. Wtf."
            );
        }
    }
}

pub fn init_command_tree() -> CommandTree {
    CommandTree::new(NAMES, DESCRIPTION)
        .then(argument(DAMAGER_ARG_NAME, SimpleArgConsumer).execute(DamagerExecutor))
}
