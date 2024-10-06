use saasbase::{
    db::{Collectable, Identifiable},
    ImageId, UserId,
};
use uuid::Uuid;

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
}

impl Default for UserData {
    fn default() -> Self {
        Self {
            id: Uuid::nil(),
            current_project: Uuid::nil(),
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
pub struct Project {
    pub id: Uuid,
    pub owner: UserId,

    pub name: String,
    pub acl: Vec<String>,
}

impl Default for Project {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            owner: Uuid::nil(),
            // TODO: use project namegen
            name: "newproj".to_string(),
            acl: vec![],
        }
    }
}

impl Collectable for Project {
    fn get_collection_name() -> &'static str {
        "project"
    }
}

impl Identifiable for Project {
    fn get_id(&self) -> Uuid {
        self.id
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Machine {
    pub id: Uuid,
    pub owner: UserId,
    pub project: Uuid,

    pub name: String,
    pub status: String,
}

impl Collectable for Machine {
    fn get_collection_name() -> &'static str {
        "machine"
    }
}

impl Identifiable for Machine {
    fn get_id(&self) -> Uuid {
        self.id
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Application {
    pub id: Uuid,
    pub owner: UserId,
    pub project: Uuid,

    pub name: String,
    pub avatar: ImageId,
}

impl Collectable for Application {
    fn get_collection_name() -> &'static str {
        "application"
    }
}

impl Identifiable for Application {
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

    // TODO: maybe allow single source to be used by multiple projects? At
    // least with github app installations there's no nice way of doing
    // single app install per project.
    pub project: Uuid,

    /// Id of the installation as provided by the user that installed the
    /// github app.
    pub installation_id: Option<u64>,
    // TODO: Source could be shared accross multiple projects.
    // pub projects: Vec<Uuid>,
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
