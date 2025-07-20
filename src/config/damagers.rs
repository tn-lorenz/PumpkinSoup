use crate::damager::Damager;
use dashmap::DashSet;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub static DAMAGERS: Lazy<DashSet<Damager>> = Lazy::new(DashSet::new);

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DamagerSettings {
    pub damage: f32,
    pub delay: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DamagerConfig {
    pub damagers: BTreeMap<String, DamagerSettings>,
}

impl Default for DamagerConfig {
    fn default() -> Self {
        let mut map = BTreeMap::new();
        map.insert(
            "easy".to_string(),
            DamagerSettings {
                damage: 4.0,
                delay: 500,
            },
        );
        map.insert(
            "medium".to_string(),
            DamagerSettings {
                damage: 5.0,
                delay: 500,
            },
        );
        map.insert(
            "hard".to_string(),
            DamagerSettings {
                damage: 7.0,
                delay: 500,
            },
        );
        map.insert(
            "extreme".to_string(),
            DamagerSettings {
                damage: 9.0,
                delay: 500,
            },
        );
        map.insert(
            "calamity".to_string(),
            DamagerSettings {
                damage: 12.0,
                delay: 500,
            },
        );
        map.insert(
            "zero_tick".to_string(),
            DamagerSettings {
                damage: 1.0,
                delay: 20,
            },
        );

        Self { damagers: map }
    }
}
