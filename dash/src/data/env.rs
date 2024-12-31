use chrono::Utc;
use uuid::Uuid;

/// Environment defines a handle for a specific instance of an application
/// that serves a distinct purpose.
///
/// Environments are bound to specific machines, where they receive a share of
/// hardware resources. This means we can define an application with multiple
/// environments where each environment exists on a different hardware system.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Env {
    pub id: Uuid,
    pub app: Uuid,

    pub handle: String,
    pub name: String,
    pub branch: String,
    pub subdomain: Option<String>,

    /// Application environment can be accessed at this url.
    /// TODO: addresses should be handled per project and optionally managed
    /// using the cloudflare integration
    pub address: String,

    pub status: Status,

    pub logs: Vec<String>,

    pub creation_time: chrono::DateTime<chrono::Utc>,
    pub last_edition_time: chrono::DateTime<chrono::Utc>,
    pub last_build_time: Option<chrono::DateTime<chrono::Utc>>,
}

impl Default for Env {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            app: Uuid::nil(),
            handle: "test".to_string(),
            name: "Testing".to_string(),
            branch: "master".to_string(),
            subdomain: None,
            address: "".to_string(),
            status: Status::default(),
            logs: Vec::new(),
            creation_time: Utc::now(),
            last_edition_time: Utc::now(),
            last_build_time: None,
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, strum::Display)]
pub enum Status {
    #[default]
    Unitialized,
    // TODO: provide ability to get build logs, perhaps reference log
    // collection item by id here
    Building,
    BuildFailed(String),
    RuntimeError(String),
    Running,
}
