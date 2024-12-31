use saasbase::{
    db::{Collectable, Identifiable},
    ImageId, UserId,
};
use uuid::Uuid;

pub mod app;
pub mod env;
pub mod key;
pub mod machine;
pub mod project;
pub mod storage;

pub use app::App;
pub use env::Env;
pub use key::Key;
pub use machine::Machine;
pub use project::Project;
pub use storage::Storage;

/// Application-specific user data. Uses the same user id as the base user
/// definition from `saasbase`.
///
/// # Centralized vs local session persistence
///
/// Our application is not concerned with storing user settings per session.
/// Any additional settings are stored in the main db and accessed directly
/// when building responses on the server.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserData {
    pub id: UserId,

    pub current_project: Uuid,

    // pub cloudflare: Cloudflare,
    // pub hetzner: Hetzner,
    pub github: Github,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Github {
    /// Verified github handle
    pub handle: Option<String>,
}

impl Default for UserData {
    fn default() -> Self {
        Self {
            id: Uuid::nil(),
            current_project: Uuid::nil(),
            github: Default::default(),
        }
    }
}

impl Collectable for UserData {
    fn get_collection_name() -> &'static str {
        "user_data"
    }
}

impl Identifiable for UserData {
    fn get_id(&self) -> Uuid {
        self.id
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Deployment {
    pub id: Uuid,
    pub owner: UserId,

    pub program: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Source {
    pub id: Uuid,

    /// Owner of the source is the user that performed the installation of the
    /// github app. As far as we are concerned, this means they showed up with
    /// the proper installation id that also matched their github infomation.
    pub owner: UserId,

    // Since with github app installations we can't do single install per
    // project, sources can be shared between multiple projects.
    pub projects: Vec<Uuid>,

    /// Id of the installation as provided by the user that installed the
    /// github app.
    pub installation_id: Option<u64>,

    pub installation_time: chrono::DateTime<chrono::Utc>,
}

impl Collectable for Source {
    fn get_collection_name() -> &'static str {
        "source"
    }
}

impl Identifiable for Source {
    fn get_id(&self) -> Uuid {
        self.id
    }
}
