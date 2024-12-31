use chrono::Utc;
use saasbase::{
    db::{Collectable, Identifiable},
    Database, ImageId, UserId,
};
use uuid::Uuid;

use anyhow::Result;

use crate::data;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct App {
    pub id: Uuid,
    pub owner: UserId,
    pub project: Uuid,

    pub name: String,
    pub avatar: ImageId,
    pub domain: String,

    pub machine: Option<Uuid>,

    pub source_url: String,

    pub envs: Vec<data::Env>,

    pub creation_time: chrono::DateTime<chrono::Utc>,
    pub last_edition_time: chrono::DateTime<chrono::Utc>,
}

impl App {
    pub fn new(db: &Database) -> Result<Self> {
        let mut app = Self::default();
        app.load_image(db)?;
        Ok(app)
    }

    pub fn load_image(&mut self, db: &Database) -> Result<()> {
        self.avatar = saasbase::user::new_avatar_image(db)?;
        Ok(())
    }
}

impl Default for App {
    fn default() -> Self {
        App {
            id: Uuid::new_v4(),
            owner: Uuid::nil(),
            project: Uuid::nil(),

            name: namegen().next().unwrap(),
            avatar: Uuid::nil(),
            domain: "default.app".to_string(),

            machine: None,

            source_url: "".to_string(),

            envs: vec![
                data::Env {
                    handle: "prod".to_string(),
                    name: "Production".to_string(),
                    branch: "prod".to_string(),
                    subdomain: None,
                    ..Default::default()
                },
                data::Env {
                    handle: "test".to_string(),
                    name: "Testing".to_string(),
                    branch: "master".to_string(),
                    subdomain: None,
                    ..Default::default()
                },
            ],

            creation_time: Utc::now(),
            last_edition_time: Utc::now(),
        }
    }
}

impl Collectable for App {
    fn get_collection_name() -> &'static str {
        "applications"
    }
}

impl Identifiable for App {
    fn get_id(&self) -> Uuid {
        self.id
    }
}

pub fn namegen<'a>() -> names::Generator<'a> {
    names::Generator::new(
        &[
            "clever",
            "sharp",
            "astute",
            "able",
            "perceptive",
            "apt",
            "acute",
            "bright",
        ],
        &[
            "implementation",
            "execution",
            "practice",
            "exercise",
            "effort",
            "labour",
            "program",
        ],
        names::Name::Numbered,
    )
}
