use askama::Template;
use axum::{extract::Path, response::IntoResponse, routing::get, Extension, Form};
use http_body_util::BodyExt;
use tokio::io::AsyncWriteExt;
use url::Url;
use uuid::Uuid;

use saasbase::{
    axum::{askama::HtmlTemplate, ConfigExt, DbExt},
    Router,
};

use crate::{
    dash::partial,
    data::{self, app},
    extract, realtime, Error, Result,
};

pub fn router() -> Router {
    Router::new()
        .route("/app/:id/env/:handle", get(env).post(env_post))
        .route("/app/:id/env/:handle/deploy", get(env_deploy))
        // this will be repeatedly called from the browser
        // .route("/app/:id/env/:handle/status", get(env_status))
        .route("/app/:id/env/:handle/status", get(env_status))
}

#[derive(Clone, Debug, Serialize, Deserialize, Template)]
#[template(path = "pages/app/env.html")]
pub struct Environment {
    head: partial::Head,
    sidebar: partial::Sidebar,
    user: saasbase::User,
    config: saasbase::Config,

    env: data::Env,
}

pub async fn env(
    user: extract::User,
    Path((app, env)): Path<(Uuid, String)>,
    Extension(config): ConfigExt,
    Extension(db): DbExt,
    Extension(github): Extension<octocrab::Octocrab>,
) -> Result<impl IntoResponse> {
    let app = db.get::<data::App>(app)?;
    let env = app
        .envs
        .iter()
        .find(|e| e.handle == env)
        .ok_or(Error::Other("env not found".to_string()))?;
    Ok(HtmlTemplate(Environment {
        head: partial::Head {
            title: format!("{} | {}", env.name, app.name),
            ..Default::default()
        },
        sidebar: todo!(),
        user: todo!(),
        config: todo!(),
        env: todo!(),
    }))
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct EnvForm {
    handle: String,
    name: String,
    triggers: String,
}

pub async fn env_post(
    user: extract::User,
    Path((app, env)): Path<(Uuid, String)>,
    Extension(config): ConfigExt,
    Extension(db): DbExt,
    Form(env_form): Form<EnvForm>,
) -> Result<impl IntoResponse> {
    Ok(())
}

/// Returns yet unread logs from the chosen environment.
pub async fn env_logs(
    user: extract::User,
    Path((app, env)): Path<(Uuid, String)>,
    Extension(config): ConfigExt,
    Extension(db): saasbase::axum::DbExt,
    // Extension(realtime): Extension<realtime::Handle>,
) -> Result<impl IntoResponse> {
    Ok(())
}

/// Returns yet unread logs from the chosen environment.
pub async fn env_status(
    user: extract::User,
    Path((app, env)): Path<(Uuid, String)>,
    Extension(config): ConfigExt,
    Extension(db): saasbase::axum::DbExt,
    // Extension(realtime): Extension<realtime::Handle>,
) -> Result<impl IntoResponse> {
    Ok(())
}

pub async fn env_deploy(
    user: extract::User,
    Path((app, env)): Path<(Uuid, String)>,
    Extension(config): ConfigExt,
    Extension(db): DbExt,
    Extension(octocrab): Extension<octocrab::Octocrab>,
    Extension(realtime): Extension<realtime::Handle>,
) -> Result<impl IntoResponse> {
    log::debug!("deploy app env");

    let app = db.get::<data::App>(app)?;
    let env = app
        .envs
        .into_iter()
        .find(|e| e.handle == env)
        .ok_or(Error::Other("bad env handle".to_string()))?;

    // TODO: support deploying from plain git url
    // let source_url = app
    //     .source_url
    //     .parse::<Url>()
    //     .map_err(|e| Error::Other(e.to_string()))?;
    // let mut source_url_segments = source_url.path_segments().unwrap();
    // let repo_owner = source_url_segments.next().unwrap();
    // let repo_name = source_url_segments.next().unwrap();

    // get the code

    // first check if the repo is public

    // if not, check user's code repos
    let sources: Vec<data::Source> = db
        .get_collection::<data::Source>()?
        .into_iter()
        .filter(|s| s.owner == user.base.id)
        .collect();
    log::trace!("sources: {sources:?}");

    // let mut source = None;
    for src in sources {
        let installation_id = match src.installation_id {
            Some(id) => id,
            None => continue,
        };

        if let Ok(installation) = octocrab.apps().installation(installation_id.into()).await {
            if installation.account.login != app.source_account {
                continue;
            }
        }

        let installation = octocrab.installation(installation_id.into());

        let repo = installation
            .repos(&app.source_account, &app.source_repo)
            .download_tarball(env.branch.clone())
            .await;

        println!("repo ok? {}", repo.is_ok());

        if repo.is_err() {
            continue;
        }

        if let Ok(mut repo) = repo {
            let mut body = repo.body_mut();

            let mut file = tokio::fs::File::create("test.tar.gz").await?;

            while let Some(next) = body.frame().await {
                let frame = next?;
                // only write data frames to file
                if let Some(chunk) = frame.data_ref() {
                    if chunk.len() < 300 {
                        if core::str::from_utf8(chunk).unwrap() == "404: Not Found" {
                            return Err(Error::Other(format!(
                                "unable to download repo tarball at the speficied address (check branch name)"
                            )));
                        }
                    }
                    file.write_all(chunk).await?;
                }
            }
            file.flush().await?;
        }
    }

    // if let Some((source, repo)) = source {
    //     let installation = github.installation(source.installation_id.unwrap().into());
    //     let content = installation
    //         .repos(&repo.owner.as_ref().unwrap().login, &repo.name)
    //         .get_content()
    //         .send()
    //         .await?;
    //     content.items.iter().inspect(|i| println!("{}", i.path));
    // } else {
    //     log::warn!("app source unavailable: {}", app.source_url);
    //     return Err(anyhow::Error::msg("source unavailable").into());
    // }

    // Execute the deploy request through the realtime interface.
    let resp = realtime
        .exec
        .execute((app.id, realtime::Request::Deploy { env: env.handle }))
        .await?;

    println!("machine response: {:?}", resp);

    Ok(())
}
