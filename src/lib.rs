use std::sync::Arc;

use pumpkin::plugin::{Context, EventPriority};
use pumpkin_api_macros::{plugin_impl, plugin_method};
use pumpkin_util::{
    PermissionLvl,
    permission::{Permission, PermissionDefault},
};

use crate::listeners::soup_rightclick::SoupRightClickHandler;

pub mod commands;
pub mod listeners;

async fn register_commands(context: &Context) -> Result<(), String> {
    let permission = Permission::new(
        "pumpkin-soup:command.soup",
        "Grants access to the /soup command.",
        PermissionDefault::Op(PermissionLvl::Four),
    );

    context.register_permission(permission).await?;

    context
        .register_command(
            commands::soup_kit_command::init_command_tree(),
            "pumpkinsoup:command.soup",
        )
        .await;

    Ok(())
}

async fn register_events(context: &Context) {
    context
        .register_event(Arc::new(SoupRightClickHandler), EventPriority::Lowest, true)
        .await;
}

#[plugin_method]
async fn on_load(&mut self, server: &Context) -> Result<(), String> {
    pumpkin::init_log!();

    register_commands(server).await?;
    register_events(server).await;

    log::info!("PumpkinSoup has been loaded.");
    Ok(())
}

#[plugin_impl]
pub struct Plugin {}

impl Plugin {
    pub fn new() -> Self {
        Plugin {}
    }
}

impl Default for Plugin {
    fn default() -> Self {
        Self::new()
    }
}
