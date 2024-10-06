use serde::de::DeserializeOwned;

pub static CONFIG_FILE: &'static str = "ruda.toml";

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Config {
    pub name: String,
    pub version: String,

    #[cfg(feature = "runner")]
    #[serde(flatten)]
    pub runner: crate::runner::Config,
}

// NOTE: perhaps move to runner module?
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Runner {
    pub test: String,
}

pub fn load<T: DeserializeOwned + Default>() -> anyhow::Result<T> {
    load_from(CONFIG_FILE)
}

/// Loads application config from toml file at the given path.
pub fn load_from<T: DeserializeOwned + Default>(path: impl AsRef<str>) -> anyhow::Result<T> {
    let config = config::Config::builder()
        .add_source(config::File::with_name(path.as_ref()).required(false))
        .add_source(config::File::with_name(&format!("secret.{}", path.as_ref())).required(false))
        .add_source(
            config::Environment::default()
                .separator("__")
                .prefix_separator("__"),
        )
        .build()?;

    let config: T = config.try_deserialize().unwrap_or_default();

    Ok(config)
}
