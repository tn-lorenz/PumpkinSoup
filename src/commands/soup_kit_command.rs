use std::sync::Arc;

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

use pumpkin_data::item::Item;
use pumpkin_util::text::TextComponent;
use pumpkin_world::item::ItemStack;

use crate::player_util::PlayerUtil;

const NAMES: [&str; 2] = ["soup", "soupkit"];
const DESCRIPTION: &str = "Give yourself a soup kit with a variable recraft amount.";
const RECRAFT_ARG_NAME: &str = "recraft_amount";

struct SoupKitExecutor;

#[async_trait]
impl CommandExecutor for SoupKitExecutor {
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
                    TextComponent::text("Only players may use this command.")
                        .color_named(pumpkin_util::text::color::NamedColor::Red),
                )
                .await;

            return Err(CommandError::GeneralCommandIssue(
                "Only players may use this command.".into(),
            ));
        };

        let amount: i32 = recraft_amount
            .parse()
            .map_err(|_| CommandError::GeneralCommandIssue("Invalid recraft amount.".into()))?;

        give_kit(player, Some(amount)).await;

        Ok(())
    }
}

pub(crate) async fn give_kit(player: &Arc<Player>, recraft_amount: Option<i32>) {
    match recraft_amount {
        Some(v) if v > 0 => {
            let amount = recraft_amount.unwrap_or(0).clamp(0, u8::MAX as i32) as u8;

            let bowls = ItemStack::new(amount, &Item::BOWL);
            let reds = ItemStack::new(amount, &Item::RED_MUSHROOM);
            let browns = ItemStack::new(amount, &Item::BROWN_MUSHROOM);
            let sword = ItemStack::new(amount, &Item::STONE_SWORD);

            player.fill_inventory_with_soup().await;

            player.set_item(13, bowls).await;
            player.set_item(14, reds).await;
            player.set_item(15, browns).await;
            player.set_item(0, sword).await;
        }
        Some(0) => {
            player.fill_inventory_with_soup().await;
        }
        Some(_) => {
            player.fill_inventory_with_soup().await;
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
        .then(argument(RECRAFT_ARG_NAME, SimpleArgConsumer).execute(SoupKitExecutor))
        .execute(SoupKitExecutor)
}
