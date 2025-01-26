use askama::Template;
use axum::{
    extract::{Path, Query},
    response::{IntoResponse, Redirect, Response},
    routing::get,
    Extension,
};
use octocrab::{models::Installation, Octocrab};
use saasbase::{
    axum::{askama::HtmlTemplate, ConfigExt, DbExt},
    Database, Router,
};
use uuid::Uuid;

use crate::{
    data,
    extract::{self, User},
    Error, Result,
};

use super::partial::{Head, Sidebar};

pub fn router() -> Router {
    Router::new()
        .route("/sources", get(list))
        .route("/source/:id", get(single))
        .route("/source/new", get(new))
        .route("/hook/source/github/install", get(install))
}

#[derive(Clone, Debug, Serialize, Deserialize, Template)]
#[template(path = "pages/source/new.html")]
pub struct New {
    head: Head,
    sidebar: Sidebar,
    user: saasbase::User,
    config: saasbase::Config,
}

pub async fn new(
    user: extract::User,
    Extension(config): ConfigExt,
    Extension(db): DbExt,
) -> Result<impl IntoResponse> {
    Ok("")
}

#[derive(Clone, Default, Debug)]
pub struct Source {
    id: Uuid,
    account_avatar_url: String,
    account_name: String,
    account_type: String,
    installation_id: String,
    installation_url: String,
    installation_created_at: String,
    installation_updated_at: String,
    repo_access: String,
}

impl Source {
    fn from_data_source(source: data::Source, install: Installation) -> Result<Self> {
        let source = Self {
            id: source.id,
            account_name: install.account.login,
            account_type: install.account.r#type,
            account_avatar_url: install.account.avatar_url.to_string(),
            installation_id: install.id.to_string(),
            installation_url: install
                .html_url
                .unwrap_or("https://github.com/apps/ruda-app/installations/new".to_string()),
            installation_created_at: install
                .created_at
                .map(|d| d.to_rfc2822())
                .unwrap_or("unknown".to_string()),
            installation_updated_at: install
                .updated_at
                .map(|d| d.to_rfc2822())
                .unwrap_or("unknown".to_string()),
            repo_access: install.repository_selection.unwrap_or("none".to_string()),
        };
        Ok(source)
    }
}

#[derive(Clone, Debug, Template)]
#[template(path = "pages/source/list.html")]
pub struct List {
    head: Head,
    sidebar: Sidebar,
    user: saasbase::User,
    config: saasbase::Config,

    sources: Vec<Source>,
}

pub async fn list(
    user: extract::User,
    Extension(config): ConfigExt,
    Extension(db): DbExt,
    Extension(octocrab): Extension<Octocrab>,
) -> Result<impl IntoResponse> {
    // preemptively recreate the sources from third-party provider
    recreate(&user, &db, octocrab.clone())
        .await
        .inspect_err(|e| log::warn!("{}", e.to_string()));

    let _sources = db
        .get_collection::<data::Source>()?
        .into_iter()
        .filter(|s| s.owner == user.base.id);
    let mut sources = vec![];

    for _source in _sources {
        if let Some(install) = _source.installation_id {
            if let Ok(install) = octocrab.apps().installation(install.into()).await {
                let source = Source::from_data_source(_source, install)?;
                sources.push(source);
            } else {
                // install id unrecognized by provider, remove it locally
                db.remove(&_source)?;
            }
        }
    }

    Ok(HtmlTemplate(List {
        head: Head {
            title: "Sources".to_string(),
            ..Default::default()
        },
        sidebar: Sidebar::at("Sources", user.base.id, &db)?,
        user: user.base,
        config: (*config).clone(),

        sources,
    }))
}

#[derive(Clone, Debug, Template)]
#[template(path = "pages/source/single.html")]
pub struct Single {
    head: Head,
    sidebar: Sidebar,
    user: saasbase::User,
    config: saasbase::Config,

    source: Source,
}

pub async fn single(
    Path(id): Path<Uuid>,
    user: extract::User,
    Extension(db): DbExt,
    Extension(config): ConfigExt,
    Extension(octocrab): Extension<Octocrab>,
) -> Result<impl IntoResponse> {
    let mut source_ = db.get::<data::Source>(id)?;
    // HACK
    let mut source = Source::default();
    if let Some(install) = source_.installation_id {
        if let Ok(install) = octocrab.apps().installation(install.into()).await {
            source = Source::from_data_source(source_, install)?;
        } else {
            db.remove(&source_)?;
        }
    }

    Ok(HtmlTemplate(Single {
        head: Head {
            title: format!("{} | Source", source.account_name),
            ..Default::default()
        },
        sidebar: Sidebar::at("Sources", user.base.id, &db)?,
        user: user.base,
        config: (*config).clone(),
        source,
    }))
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InstallParams {
    pub installation_id: u64,
    pub setup_action: String,
}

pub async fn install(
    user: extract::User,
    Query(params): Query<InstallParams>,
    Extension(db): DbExt,
    Extension(octocrab): Extension<Octocrab>,
) -> Result<Response> {
    // TODO: confirm the installation exists
    // let author = octocrab
    //     .installation(params.installation_id.into())
    //     .current()
    //     .user()
    //     .await?;
    // println!("{author:?}");

    if let Some(source) = db
        .get_collection::<data::Source>()?
        .into_iter()
        .find(|s| s.installation_id == Some(params.installation_id))
    {
        return Ok(Redirect::to("/sources").into_response());
    } else {
        db.set(&data::Source {
            id: Uuid::new_v4(),
            owner: user.base.id,
            projects: vec![user.data.current_project],
            installation_id: Some(params.installation_id),
            installation_time: chrono::Utc::now(),
        })?;
    }

    Ok(Redirect::to("/sources").into_response())
}

/// Recreates missing source items based on information from third-parties
/// (e.g. github).
// TODO: this should require properly connecting the account with github
// account, so then we can use the login string for grabbing installation id.
// Alternatively the email can be used, but it would have to match and get
// confirmed beforehand.
async fn recreate(user: &User, db: &Database, octocrab: Octocrab) -> Result<()> {
    let installs = octocrab.apps().installations().per_page(20).send().await?;
    let mut installation_id = None;
    println!("email: {}", user.base.email);
    for install in installs {
        println!("install email: {:?}", install.account.email);
        println!("install login: {}", install.account.login);
        if user
            .base
            .linked_accounts
            .contains(&saasbase::oauth::Link::Github {
                login: install.account.login,
            })
        {
            installation_id = Some(install.id);
            break;
        }
    }

    let installation_id = installation_id.ok_or(Error::Other(
        "no valid installations for user based on email match with github".to_string(),
    ))?;

    let sources = db.get_collection::<data::Source>()?;
    if sources
        .iter()
        .find(|s| s.installation_id == Some(*installation_id))
        .is_none()
    {
        db.set(&data::Source {
            id: Uuid::new_v4(),
            owner: user.base.id,
            projects: vec![user.data.current_project],
            installation_id: Some(*installation_id),
            installation_time: chrono::Utc::now(),
        })?;
    }

    Ok(())
}
