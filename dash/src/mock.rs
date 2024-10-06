use uuid::Uuid;

use saasbase::{Database, UserId};

use crate::{
    config::Config,
    data::{self, Application, Machine, Project, Source, UserData},
    Result,
};

pub fn generate(config: &Config, db: &Database) -> Result<()> {
    let user = saasbase::mock::user(&config.base, db)?;
    println!("user id: {}", user.id);

    let project = Project {
        id: Uuid::new_v4(),
        owner: user.id,
        name: project_namegen().next().unwrap(),
        acl: vec![],
    };
    db.set(&project)?;
    let project = Project {
        id: Uuid::new_v4(),
        owner: user.id,
        name: project_namegen().next().unwrap(),
        acl: vec![],
    };
    db.set(&project)?;

    let app_user = UserData {
        id: user.id,
        current_project: project.id,
        ..Default::default()
    };
    db.set(&app_user)?;

    println!("just set app user: {}", app_user.id);

    let source = Source {
        id: Uuid::new_v4(),
        owner: user.id,
        project: project.id,
        installation_id: None,
    };
    db.set(&source)?;

    let application = Application {
        id: Uuid::new_v4(),
        owner: user.id,
        project: project.id,
        name: application_namegen().next().unwrap(),
        avatar: Uuid::new_v4(),
    };
    db.set(&application)?;

    let machine = Machine {
        id: Uuid::new_v4(),
        owner: user.id,
        project: project.id,
        name: machine_namegen().next().unwrap(),
        status: "OK".to_string(),
    };
    db.set(&machine)?;

    Ok(())
}

pub fn project_namegen<'a>() -> names::Generator<'a> {
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

pub fn machine_namegen<'a>() -> names::Generator<'a> {
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

pub fn application_namegen<'a>() -> names::Generator<'a> {
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
