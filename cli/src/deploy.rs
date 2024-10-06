use std::{fs::File, io::Read};

use anyhow::Result;
use clap::ArgMatches;
use tokio_util::sync::CancellationToken;

use ruda::api;

use crate::util;

pub fn cmd() -> clap::Command {
    use clap::{Arg, Command};
    Command::new("deploy")
        .about("Deploy an application")
        .display_order(30)
        .arg(Arg::new("name").value_name("APP_NAME"))
        .arg(Arg::new("bin").value_name("path to app binary"))
}

/// Deploys an application.
pub async fn run(matches: &ArgMatches, cancellation: CancellationToken) -> Result<()> {
    let token = util::retrieve_token().await?;
    println!("got token: {:?}", token);

    let app_bin = matches
        .get_one::<String>("bin")
        .expect("app bin not provided");

    let mut bin = Vec::new();
    let mut file = File::open(app_bin).unwrap();
    file.read_to_end(&mut bin).unwrap();

    // authenticate to get the access token
    let query = api::DeployQuery {
        name: "testbin".to_string(),
        address: "localhost:8001".to_string(),
    };

    println!("sending request...");

    let client = reqwest::Client::new();

    let request = client
        // .post("https://ruda.app/api/deploy")
        .post("http://127.0.0.1:8000/api/deploy")
        .query(&query)
        .body(bin)
        // .timeout(Duration::from_secs(1))
        .bearer_auth(token)
        .fetch_mode_no_cors()
        .build()
        .unwrap();

    let response = client.execute(request).await?.status();

    println!("response: {:?}", response);

    cancellation.cancel();
    Ok(())
}
