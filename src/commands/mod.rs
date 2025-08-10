use crate::commands::damager_command::build_invalid_arg_msg;
use crate::config::DAMAGERS;
use crate::damager::Damager;
use async_trait::async_trait;
use pumpkin::command::CommandSender;
use pumpkin::command::args::{
    Arg, ArgumentConsumer, ConsumedArgs, DefaultNameArgConsumer, FindArg, GetClientSideArgParser,
};
use pumpkin::command::dispatcher::CommandError;
use pumpkin::command::dispatcher::CommandError::{CommandFailed, InvalidConsumption};
use pumpkin::command::tree::RawArgs;
use pumpkin::server::Server;
use pumpkin_protocol::java::client::play::{
    ArgumentType, CommandSuggestion, StringProtoArgBehavior, SuggestionProviders,
};
use pumpkin_util::text::TextComponent;

pub(crate) mod damager_command;
pub mod soup_kit_command;
// pub mod test_pdc_command;
// pub mod test_add_pdc_command;

pub struct DamagerArgumentConsumer;

impl GetClientSideArgParser for DamagerArgumentConsumer {
    fn get_client_side_parser(&self) -> ArgumentType<'_> {
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

        if DAMAGERS.iter().any(|d| d.name.eq_ignore_ascii_case(s)) {
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
        let suggestions = DAMAGERS
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
            Some(Arg::Simple(s)) => {
                if DAMAGERS.iter().any(|d| d.name.eq_ignore_ascii_case(s)) {
                    Ok(s.to_string().parse().unwrap())
                } else {
                    Err(CommandFailed(Box::new(TextComponent::text(
                        build_invalid_arg_msg(),
                    ))))
                }
            }
            _ => Err(InvalidConsumption(Some(name.to_string()))),
        }
    }
}
