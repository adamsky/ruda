use uuid::Uuid;

use saasbase::{Database, UserId};

use crate::{
    config::Config,
    data::{self, App, Machine, Project, Source, UserData},
    Result,
};

pub fn generate(config: &Config, db: &Database) -> Result<()> {
    for n in 0..1 {
        let _ = user(config, db)?;
    }

    log::info!("users in db: {}", db.len::<saasbase::User>()?);
    log::info!("apps in db: {}", db.len::<data::App>()?);

    Ok(())
}

pub fn user(config: &Config, db: &Database) -> Result<()> {
    let user = saasbase::mock::user(&config.base, db)?;

    let project = Project {
        id: Uuid::new_v4(),
        owner: user.id,
        acl: vec![],
        ..Default::default()
    };
    db.set(&project)?;
    let project = Project {
        id: Uuid::new_v4(),
        owner: user.id,
        acl: vec![],
        ..Default::default()
    };
    db.set(&project)?;

    let app_user = UserData {
        id: user.id,
        current_project: project.id,
        ..Default::default()
    };
    db.set(&app_user)?;

    let source = Source {
        id: Uuid::new_v4(),
        owner: user.id,
        projects: vec![project.id],
        installation_id: None,
        installation_time: chrono::Utc::now(),
    };
    db.set(&source)?;

    let application = App {
        id: Uuid::new_v4(),
        owner: user.id,
        project: project.id,
        avatar: Uuid::new_v4(),
        source_url: "github.com/app/source".to_string(),
        ..Default::default()
    };
    db.set(&application)?;

    let machine = Machine {
        id: Uuid::new_v4(),
        owner: user.id,
        project: project.id,
        kind: data::machine::Kind::Unmanaged,
        status: data::machine::Status::Disconnected,
        secret: Uuid::new_v4(),
        address: "localhost".to_string(),
        ..Default::default()
    };
    db.set(&machine)?;

    Ok(())
}
