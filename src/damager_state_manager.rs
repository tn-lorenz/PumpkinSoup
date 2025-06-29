use dashmap::DashSet;
use once_cell::sync::Lazy;
use uuid::Uuid;

pub static ACTIVE_UUIDS: Lazy<DashSet<Uuid>> = Lazy::new(DashSet::new);
