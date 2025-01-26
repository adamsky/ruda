use std::fmt::Debug;

use askama::Template;
use axum::{
    extract::Query,
    response::{AppendHeaders, IntoResponse},
    Extension,
};
use octocrab::Octocrab;
use saasbase::axum::{askama::HtmlTemplate, extract, DbExt};

use crate::{data, Result};

#[derive(Clone, Debug, Serialize, Deserialize, Template)]
#[template(path = "partials/modals/new-app-modal-source-accounts.html")]
pub struct NewAppModalSourceAccounts {
    pub accounts: Vec<(String, String)>,
}

pub async fn new_app_modal_source_accounts(
    user: extract::User,
    Extension(db): DbExt,
    Extension(octocrab): Extension<Octocrab>,
) -> Result<impl IntoResponse> {
    let source_accounts = db
        .get_collection::<data::Source>()?
        .into_iter()
        .filter(|s| s.owner == user.id)
        .collect::<Vec<_>>();
    let mut accounts = vec![];

    for source_account in source_accounts {
        if let Some(installation_id) = source_account.installation_id {
            if let Ok(installation) = octocrab.apps().installation(installation_id.into()).await {
                accounts.push((
                    installation.account.avatar_url.to_string(),
                    installation.account.login,
                ));
            }
        }
    }

    Ok((
        AppendHeaders([("HX-Trigger-After-Settle", "sourceAccountsLoaded")]),
        HtmlTemplate(NewAppModalSourceAccounts { accounts }),
    ))
}

#[derive(Clone, Debug, Serialize, Deserialize, Template)]
#[template(path = "partials/modals/new-app-modal-source-repos.html")]
pub struct NewAppModalSourceRepos {
    pub repos: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReposQuery {
    pub account: String,
}

pub async fn new_app_modal_source_repos(
    user: extract::User,
    Query(query): Query<ReposQuery>,
    Extension(db): DbExt,
    Extension(octocrab): Extension<Octocrab>,
) -> Result<impl IntoResponse> {
    let source_accounts = db
        .get_collection::<data::Source>()?
        .into_iter()
        .filter(|s| s.owner == user.id)
        .collect::<Vec<_>>();
    let mut repos = vec![];

    for source_account in source_accounts {
        if let Some(installation_id) = source_account.installation_id {
            if let Ok(installation) = octocrab.apps().installation(installation_id.into()).await {
                if installation.account.login != query.account {
                    continue;
                }
            }

            let oc = octocrab.installation(installation_id.into());

            // TODO: support querying more than a single page, important for
            // users that have >100 repositories
            let installed_repos: octocrab::models::InstallationRepositories = oc
                .get(
                    format!("/installation/repositories?per_page={}&page={}", 100, 1),
                    None::<&()>,
                )
                .await?;
            repos.extend(installed_repos.repositories.into_iter().map(|r| r.name));
        }
    }

    println!("installation repos total count: {}", repos.len());

    Ok(HtmlTemplate(NewAppModalSourceRepos { repos }))
}
