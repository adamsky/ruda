use saasbase::{
    db::{Collectable, Identifiable},
    UserId,
};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Key {
    pub id: Uuid,
    pub owner: UserId,
    pub project: Uuid,

    pub name: String,
    pub kind: Kind,
}

impl Default for Key {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            owner: Uuid::nil(),
            project: Uuid::nil(),
            name: namegen().next().unwrap(),
            kind: Kind::S3,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Kind {
    S3,
    StorageBox,
}

impl Collectable for Key {
    fn get_collection_name() -> &'static str {
        "keys"
    }
}

impl Identifiable for Key {
    fn get_id(&self) -> Uuid {
        self.id
    }
}

pub fn namegen<'a>() -> names::Generator<'a> {
    names::Generator::new(
        &["unbreakable", "quantum", "vital"],
        &["safe", "key", "pass", "opener", "device"],
        names::Name::Numbered,
    )
}
