use pumpkin_api_macros::{plugin_impl, plugin_method};
use pumpkin::plugin::Context;

pub mod listeners;

#[plugin_method] 
async fn on_load(&mut self, server: &Context) -> Result<(), String> {
    pumpkin::init_log!(); 

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