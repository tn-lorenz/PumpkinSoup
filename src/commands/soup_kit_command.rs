use std::sync::Arc;

use async_trait::async_trait;

use pumpkin::{
    command::{
        args::{Arg, ConsumedArgs, simple::SimpleArgConsumer},
        dispatcher::{CommandError, CommandError::InvalidConsumption},
        tree::{
            CommandTree,
            builder::{argument, literal},
        },
        {CommandExecutor, CommandSender},
    },
    entity::player::Player,
    server::Server,
};

use pumpkin_data::item::Item;
use pumpkin_world::item::ItemStack;

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
            return Err(CommandError::GeneralCommandIssue(
                "Only players may use this command.".into(),
            ));
        };

        let amount: i32 = recraft_amount.parse().map_err(|_| {
            CommandError::GeneralCommandIssue("Invalid number for recraft_amount.".into())
        })?;

        give_kit(player, Some(amount)).await;

        Ok(())
    }
}

pub(crate) async fn give_kit(player: &Arc<Player>, recraft_amount: Option<i32>) {
    player.clear_inventory().await;

    match recraft_amount {
        Some(v) if v > 0 => {
            let amount = recraft_amount.unwrap_or(0).clamp(0, u8::MAX as i32) as u8;

            let bowls = ItemStack::new(amount, &Item::BOWL);
            let reds = ItemStack::new(amount, &Item::RED_MUSHROOM);
            let browns = ItemStack::new(amount, &Item::BROWN_MUSHROOM);

            player.fill_inventory_with_soup().await;

            player.set_item(13, bowls).await;
            player.set_item(14, reds).await;
            player.set_item(15, browns).await;
        }
        Some(0) => {
            player.fill_inventory_with_soup().await;
        }
        Some(_) => {}
        None => {
            player.fill_inventory_with_soup().await;
        }
    }
}

#[async_trait::async_trait]
pub(crate) trait PlayerUtil {
    async fn set_item(&self, slot: i16, item: ItemStack);
    async fn fill_inventory_with_soup(&self);
    async fn clear_inventory(&self);
}

#[async_trait::async_trait]
impl PlayerUtil for Arc<Player> {
    async fn set_item(&self, slot: i16, mut item: ItemStack) {
        self.inventory().insert_stack(slot, &mut item).await;
    }

    async fn clear_inventory(&self) {
        let mut air = ItemStack::new(1, &Item::AIR);

        for i in 0..35 {
            self.inventory().insert_stack(i, &mut air).await;
        }
    }

    async fn fill_inventory_with_soup(&self) {
        let mut soup = ItemStack::new(1, &Item::MUSHROOM_STEW);

        for i in 0..35 {
            self.inventory().insert_stack(i, &mut soup).await;
        }
    }
}

pub fn init_command_tree() -> CommandTree {
    CommandTree::new(NAMES, DESCRIPTION)
        .then(literal("soupkit").execute(SoupKitExecutor))
        .then(argument(RECRAFT_ARG_NAME, SimpleArgConsumer).execute(SoupKitExecutor))
}
