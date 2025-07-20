use std::str::FromStr;
use crate::config::DAMAGERS;

pub mod damager_state;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Damager {
    pub name: String,
    pub damage: i32,
    pub delay: u32,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseDamagerError;

impl FromStr for Damager {
    type Err = ParseDamagerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        DAMAGERS
            .iter()
            .find(|d| d.name.eq_ignore_ascii_case(s))
            .map(|d| d.clone())
            .ok_or(ParseDamagerError)
    }
}
