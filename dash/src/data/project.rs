use saasbase::{
    db::{Collectable, Identifiable},
    UserId,
};
use uuid::Uuid;

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
            name: namegen().next().unwrap(),
            acl: vec![],
        }
    }
}

impl Collectable for Project {
    fn get_collection_name() -> &'static str {
        "projects"
    }
}

impl Identifiable for Project {
    fn get_id(&self) -> Uuid {
        self.id
    }
}

pub fn namegen<'a>() -> names::Generator<'a> {
    names::Generator::new(
        &[
            "remarkable",
            "monumental",
            "great",
            "massive",
            "worthy",
            "huge",
            "laudable",
            "ambitious",
        ],
        &[
            "scheme",
            "undertaking",
            "plan",
            "outline",
            "design",
            "campaign",
            "operation",
            "endeavour",
            "effort",
        ],
        names::Name::Numbered,
    )
}
