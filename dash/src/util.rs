use saasbase::{Database, User, UserId};

use crate::{
    data::{Project, UserData},
    Result,
};

/// Creates user along with all necessary starting items.
pub fn create_user(db: &Database) -> Result<User> {
    let user = User::new(&db)?;
    db.set(&user)?;

    let mut project = Project::default();
    project.owner = user.id;
    db.set(&project)?;

    let app_user = UserData {
        id: user.id,
        current_project: project.id,
        ..Default::default()
    };
    db.set(&app_user)?;

    Ok(user)
}
