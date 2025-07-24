use once_cell::sync::OnceCell;
use pumpkin::plugin::Context;

static SERVER_CONTEXT: OnceCell<Context> = OnceCell::new();

pub fn set_context(ctx: Context) {
    if SERVER_CONTEXT.set(ctx.clone()).is_err() {
        log::warn!("Context was already set — ignoring");
    }
}

pub fn get_context() -> &'static Context {
    SERVER_CONTEXT.get().expect("Context not set")
}
