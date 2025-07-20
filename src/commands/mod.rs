use async_trait::async_trait;
use pumpkin::command::args::{Arg, ArgumentConsumer, ConsumedArgs, DefaultNameArgConsumer, FindArg, GetClientSideArgParser};
use pumpkin::command::CommandSender;
use pumpkin::command::dispatcher::CommandError;
use pumpkin::command::dispatcher::CommandError::InvalidConsumption;
use pumpkin::command::tree::RawArgs;
use pumpkin::server::Server;
use pumpkin_protocol::java::client::play::{ArgumentType, CommandSuggestion, StringProtoArgBehavior, SuggestionProviders};
use crate::damager::Damager;

pub(crate) mod damager_command;
pub mod soup_kit_command;
// pub mod test_pdc_command;
// pub mod test_add_pdc_command;

pub struct DamagerArgumentConsumer;

impl GetClientSideArgParser for DamagerArgumentConsumer {
    fn get_client_side_parser(&self) -> ArgumentType {
        ArgumentType::String(StringProtoArgBehavior::SingleWord)
    }

    fn get_client_side_suggestion_type_override(&self) -> Option<SuggestionProviders> {
        None
    }
}

#[async_trait]
impl ArgumentConsumer for DamagerArgumentConsumer {
    async fn consume<'a>(
        &'a self,
        _sender: &CommandSender,
        _server: &'a Server,
        args: &mut RawArgs<'a>,
    ) -> Option<Arg<'a>> {
        let s = args.pop()?;

        if crate::config::damagers::DAMAGERS.iter().any(|d| d.name.eq_ignore_ascii_case(s)) {
            Some(Arg::Simple(s))
        } else {
            None
        }
    }

    async fn suggest<'a>(
        &'a self,
        _sender: &CommandSender,
        _server: &'a Server,
        _input: &'a str,
    ) -> Result<Option<Vec<CommandSuggestion>>, CommandError> {
        let suggestions = crate::config::damagers::DAMAGERS
            .iter()
            .map(|d| CommandSuggestion::new(d.name.clone(), None))
            .collect();

        Ok(Some(suggestions))
    }
}

impl DefaultNameArgConsumer for DamagerArgumentConsumer {
    fn default_name(&self) -> &'static str {
        "damager"
    }
}

impl<'a> FindArg<'a> for DamagerArgumentConsumer {
    type Data = Damager;

    fn find_arg(args: &'a ConsumedArgs, name: &str) -> Result<Self::Data, CommandError> {
        match args.get(name) {
            Some(Arg::Simple(data)) => Ok((*data).parse().unwrap()),
            _ => Err(InvalidConsumption(Some(name.to_string()))),
        }
    }
}
