use log::warn;
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::path::Path;
use std::sync::LazyLock;
use std::{env, fs};

pub mod damagers;
pub use damagers::{DAMAGERS, Damager, DamagerConfig};

const CONFIG_ROOT_FOLDER: &str = "plugins/pumpkinsoup/";

pub static DAMAGER_CONFIG: LazyLock<DamagerConfig> = LazyLock::new(|| {
    let exec_dir = env::current_dir().unwrap();
    DamagerConfig::load(&exec_dir)
});

impl LoadConfiguration for DamagerConfig {
    fn get_path() -> &'static Path {
        Path::new("config.toml")
    }

    fn validate(&self) {}
}

trait LoadConfiguration {
    fn load(exec_dir: &Path) -> Self
    where
        Self: Sized + Default + Serialize + DeserializeOwned,
    {
        let config_dir = exec_dir.join(CONFIG_ROOT_FOLDER);
        if !config_dir.exists() {
            log::debug!("creating new config root folder");
            fs::create_dir(&config_dir).expect("Failed to create config root folder");
        }
        let path = config_dir.join(Self::get_path());

        let config = if path.exists() {
            let file_content = fs::read_to_string(&path)
                .unwrap_or_else(|_| panic!("Couldn't read configuration file at {:?}", &path));

            toml::from_str(&file_content).unwrap_or_else(|err| {
                panic!(
                    "Couldn't parse config at {:?}. Reason: {}. This is probably caused by a config update; just delete the old config and start Pumpkin again",
                    &path,
                    err.message()
                )
            })
        } else {
            let content = Self::default();

            if let Err(err) = fs::write(&path, toml::to_string(&content).unwrap()) {
                warn!(
                    "Couldn't write default config to {:?}. Reason: {}. This is probably caused by a config update; just delete the old config and start Pumpkin again",
                    &path, err
                );
            }

            content
        };

        config.validate();
        config
    }

    fn get_path() -> &'static Path;

    fn validate(&self);
}
