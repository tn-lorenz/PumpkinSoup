use once_cell::sync::OnceCell;
use pumpkin::plugin::Context;

#[allow(dead_code)]
static SERVER_CONTEXT: OnceCell<Context> = OnceCell::new();

#[allow(dead_code)]
pub fn set_context(ctx: Context) {
    if SERVER_CONTEXT.set(ctx.clone()).is_err() {
        log::warn!("Context was already set — ignoring");
    }
}

#[allow(dead_code)]
pub fn get_context() -> &'static Context {
    SERVER_CONTEXT.get().expect("Context not set")
}
