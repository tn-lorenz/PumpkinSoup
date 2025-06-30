use std::sync::Arc;

use async_trait::async_trait;
use futures::join;
use pumpkin::command::tree::builder::require;
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
use pumpkin_data::item::Item;
use pumpkin_util::text::TextComponent;
use pumpkin_world::item::ItemStack;

use crate::util::player_util::PlayerUtil;

const NAMES: [&str; 2] = ["soup", "soupkit"];
const DESCRIPTION: &str = "Give yourself a soup kit with a variable recraft amount.";
const RECRAFT_ARG_NAME: &str = "recraft_amount";
const MSG_INVALID_RC_AMOUNT: &str = "Invalid argument. Recraft amount must be between 0 and 64.";
const MSG_NOT_PLAYER: &str = "Only players may use this command.";

struct SoupKitExecutorWithArg;
struct SoupKitExecutorNoArg;

#[async_trait]
impl CommandExecutor for SoupKitExecutorWithArg {
    async fn execute<'a>(
        &self,
        sender: &mut CommandSender,
        _server: &Server,
        args: &ConsumedArgs<'a>,
    ) -> Result<(), CommandError> {
        let Some(Arg::Simple(recraft_amount)) = args.get(RECRAFT_ARG_NAME) else {
            return Err(InvalidConsumption(Some(RECRAFT_ARG_NAME.into())));
        };

        let CommandSender::Player(player) = sender else {
            sender
                .send_message(
                    TextComponent::text(MSG_NOT_PLAYER)
                        .color_named(pumpkin_util::text::color::NamedColor::Red),
                )
                .await;

            return Err(CommandError::CommandFailed(Box::new(TextComponent::text(
                MSG_NOT_PLAYER,
            ))));
        };

        let amount: u8 = recraft_amount.parse::<u8>().map_err(|_| {
            CommandError::CommandFailed(Box::new(TextComponent::text(MSG_INVALID_RC_AMOUNT)))
        })?;

        if amount > 64 {
            return Err(CommandError::CommandFailed(Box::new(TextComponent::text(
                MSG_INVALID_RC_AMOUNT,
            ))));
        }

        give_kit(player, Some(amount)).await;

        Ok(())
    }
}

#[async_trait]
impl CommandExecutor for SoupKitExecutorNoArg {
    async fn execute<'a>(
        &self,
        sender: &mut CommandSender,
        _server: &Server,
        _args: &ConsumedArgs<'a>,
    ) -> Result<(), CommandError> {
        let target = sender.as_player().ok_or(CommandError::InvalidRequirement)?;

        give_kit(&target, Option::from(0)).await;

        Ok(())
    }
}

pub(crate) async fn give_kit(player: &Arc<Player>, recraft_amount: Option<u8>) {
    let sword = ItemStack::new(1, &Item::STONE_SWORD);

    match recraft_amount {
        Some(v) if v > 0 => {
            let amount = recraft_amount.unwrap_or(0).clamp(0, 64);

            let bowls = ItemStack::new(amount, &Item::BOWL);
            let reds = ItemStack::new(amount, &Item::RED_MUSHROOM);
            let browns = ItemStack::new(amount, &Item::BROWN_MUSHROOM);

            player.fill_inventory_with_soup().await;

            join!(
                player.set_item(13, bowls),
                player.set_item(14, reds),
                player.set_item(15, browns),
                player.set_item(0, sword)
            );
        }
        Some(0) => {
            player.fill_inventory_with_soup().await;
            player.set_item(0, sword).await;
        }
        Some(_) => {
            player.fill_inventory_with_soup().await;
            player.set_item(0, sword).await;
        }
        None => {
            log::error!(
                "The recraft argument is of type `None`. Idk how you even managed to do that. Wtf."
            );
        }
    }
}

pub fn init_command_tree() -> CommandTree {
    CommandTree::new(NAMES, DESCRIPTION)
        .then(argument(RECRAFT_ARG_NAME, SimpleArgConsumer).execute(SoupKitExecutorWithArg))
        .then(require(|sender| sender.is_player()).execute(SoupKitExecutorNoArg))
}
