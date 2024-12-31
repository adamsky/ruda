#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Github {
    pub app_id: String,
    pub app_key: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    #[serde(flatten)]
    pub base: saasbase::Config,

    /// Decides whether to start up a runner task as part of the dashboard
    /// process.
    pub local_runner: bool,

    /// Github app configuration. Allows acessing things like private repos.
    pub github: Option<Github>,
}
