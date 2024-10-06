#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Github {
    pub app_id: String,
    pub app_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(flatten)]
    pub base: saasbase::Config,

    /// Github app configuration. Allows acessing things like private repos.
    pub github: Github,
}
