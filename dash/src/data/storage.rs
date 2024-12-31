use saasbase::{
    db::{Collectable, Identifiable},
    UserId,
};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Storage {
    pub id: Uuid,
    pub owner: UserId,
    pub project: Uuid,

    pub name: String,
    pub kind: Kind,
}

impl Default for Storage {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            owner: Uuid::nil(),
            project: Uuid::nil(),
            // TODO: use namegen
            name: "default-storage".to_string(),
            kind: Kind::S3,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Kind {
    S3,
    StorageBox,
}

impl Collectable for Storage {
    fn get_collection_name() -> &'static str {
        "storages"
    }
}

impl Identifiable for Storage {
    fn get_id(&self) -> Uuid {
        self.id
    }
}

impl Storage {
    pub fn free_storage(&self) -> String {
        unimplemented!()
    }
}
