use saasbase::{
    db::{Collectable, Identifiable},
    UserId,
};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Machine {
    pub id: Uuid,
    pub owner: UserId,
    pub project: Uuid,

    pub kind: Kind,

    pub name: String,
    pub status: Status,

    /// Unique code for authorizing incoming connections from machine
    pub secret: Uuid,

    pub address: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, strum::Display)]
pub enum Kind {
    /// Managed machine is one handled by the system, e.g. based on an
    /// integration with a third-party service. Using an external API this kind
    /// of machine will be automatically added/removed based on external
    /// context.
    Managed,
    /// Unmanaged machine is one solely handled by the user.
    Unmanaged,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, strum::Display)]
pub enum Status {
    Disconnected,
    Connected,
}

impl Default for Machine {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            owner: Uuid::nil(),
            project: Uuid::nil(),

            kind: Kind::Unmanaged,

            name: namegen().next().unwrap(),
            status: Status::Disconnected,

            secret: Uuid::new_v4(),
            address: "localhost".to_string(),
        }
    }
}

impl Collectable for Machine {
    fn get_collection_name() -> &'static str {
        "machines"
    }
}

impl Identifiable for Machine {
    fn get_id(&self) -> Uuid {
        self.id
    }
}

pub fn namegen<'a>() -> names::Generator<'a> {
    names::Generator::new(
        &[
            "busy",
            "dilligent",
            "energetic",
            "zealous",
            "nimble",
            "earnest",
            "resolute",
            "vigorous",
        ],
        &[
            "automaton",
            "widget",
            "engine",
            "instrument",
            "gadget",
            "robot",
            "workhorse",
            "mechanism",
            "thingamabob",
        ],
        names::Name::Numbered,
    )
}
