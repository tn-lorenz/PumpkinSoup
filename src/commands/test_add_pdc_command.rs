/* use async_trait::async_trait;
use pumpkin::command::tree::builder::require;
use pumpkin::{command::{
    args::ConsumedArgs,
    dispatcher::CommandError,
    tree::CommandTree,
    {CommandExecutor, CommandSender},
}, ns_key, server::Server};
use pumpkin::entity::EntityBase;
use pumpkin::plugin::persistence::{PersistentDataHolder, PersistentDataType};

const NAMES: [&str; 1] = ["pdc"];
const DESCRIPTION: &str = "Should set a String under pumpkinsoup:test";

struct Executor;

#[async_trait]
impl CommandExecutor for Executor {
    async fn execute<'a>(
        &self,
        sender: &mut CommandSender,
        _server: &Server,
        _args: &ConsumedArgs<'a>,
    ) -> Result<(), CommandError> {
        let target = sender.as_player().ok_or(CommandError::InvalidRequirement)?;

        let entity = target.get_entity();
        let key = ns_key!("test");

        entity.insert(&key, PersistentDataType::String("hiiii".to_string()));

        log::info!("PDC gesetzt: {} -> {:?}", key, entity.get(&key));

        let nbt = entity.read_nbt(&Default::default());

        Ok(())
    }
}


pub fn init_command_tree() -> CommandTree {
    CommandTree::new(NAMES, DESCRIPTION)
        .then(require(|sender| sender.is_player()).execute(Executor))
} */
