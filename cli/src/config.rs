#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Config {
    #[serde(flatten)]
    pub base: ruda::config::Config,

    pub login: Login,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Login {
    pub key: Option<String>,
    pub credentials: Option<(String, String)>,
}
