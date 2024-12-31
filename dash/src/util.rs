use saasbase::{Database, User, UserId};

use crate::{
    data::{Project, UserData},
    Result,
};

/// Creates user along with all necessary starting items.
pub fn create_user(db: &Database) -> Result<User> {
    let user = User::new(&db)?;
    db.set(&user)?;

    init_user(user.id, db)?;

    Ok(user)
}

/// Creates application-specific data for an existing base user.
pub fn init_user(user_id: UserId, db: &Database) -> Result<UserData> {
    let mut project = Project::default();
    project.owner = user_id;
    db.set(&project)?;

    let user_data = UserData {
        id: user_id,
        current_project: project.id,
        ..Default::default()
    };
    db.set(&user_data)?;

    Ok(user_data)
}
