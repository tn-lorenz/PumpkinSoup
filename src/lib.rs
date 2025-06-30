use crate::listeners::soup_rightclick::SoupRightClickHandler;
use once_cell::sync::Lazy;
use pumpkin::plugin::{Context, EventPriority};
use pumpkin_api_macros::{plugin_impl, plugin_method};
use pumpkin_util::{
    PermissionLvl,
    permission::{Permission, PermissionDefault},
};
use std::sync::Arc;
use tokio::runtime::Runtime;

pub mod commands;
mod damager_state;
pub mod listeners;
mod util;

const PLUGIN_NAME: &str = env!("CARGO_PKG_NAME");

async fn register_commands(context: &Context) -> Result<(), String> {
    let soup_kit_permission = Permission::new(
        &format!("{PLUGIN_NAME}:command.soup"),
        "Grants access to the /soup command.",
        PermissionDefault::Op(PermissionLvl::Four),
    );

    context.register_permission(soup_kit_permission).await?;

    context
        .register_command(
            commands::soup_kit_command::init_command_tree(),
            &format!("{PLUGIN_NAME}:command.soup"),
        )
        .await;

    let damager_cmd_permission = Permission::new(
        &format!("{PLUGIN_NAME}:command.damager"),
        "Grants access to the /damager command.",
        PermissionDefault::Op(PermissionLvl::Four),
    );

    context.register_permission(damager_cmd_permission).await?;

    context
        .register_command(
            commands::damager_command::init_command_tree(),
            &format!("{PLUGIN_NAME}:command.damager"),
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

pub static TOKIO_RUNTIME: Lazy<Runtime> =
    Lazy::new(|| Runtime::new().expect("Failed to create global Tokio Runtime"));
