use crate::config::{self, Config};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct AcessTokenBody<'a> {
    pub identifier: &'a str,
    pub secret: &'a str,
}

#[derive(Deserialize, Debug)]
pub struct AcessToken {
    pub access_token: String,
}

// #[derive(Deserialize, Debug)]
// pub struct RepoImages {
//     pub digest: String,
// }

#[derive(Deserialize)]
pub struct RepoTag {
    // pub images: Vec<RepoImages>,
    pub digest: String,
}

// this struct is to take the namespaces, repo, server and tags
// We gonna check if its ghrc or docker hub later as well
#[derive(Deserialize, Debug)]
pub struct ApiCall<'a> {
    pub namespace: &'a str,
    pub repo: &'a str,
    pub tag: &'a str,
}

// Parse the repo tag into information that we need
impl<'a> ApiCall<'a> {
    pub fn parse(input: &'a str) -> Self {
        let (image_part, tag) = input.split_once(':').unwrap_or((input, "latest"));

        let (namespace, repo) = image_part
            .split_once('/')
            .unwrap_or(("library", image_part));

        Self {
            namespace,
            repo,
            tag,
        }
    }
}

//Return access token
pub async fn get_access_token_dockerhub(config: &config::Config) -> Result<String> {
    let content = format!(
        "{{\"identifier\": \"{}\", \"secret\": \"{}\"}}",
        Config::identifier(config),
        Config::secret(config)
    );

    let content_str = content.as_str();

    let content_json: AcessTokenBody = serde_json::from_str(content_str)
        .context("Failed to parse the identifier and secret to JSON")?;

    let client = reqwest::Client::new();
    let res = client
        .post("https://hub.docker.com/v2/auth/token")
        .json(&content_json)
        .send()
        .await
        .context("Error when create access token")?;

    let data = res
        .json::<AcessToken>()
        .await
        .context("Failed to parse the JSON token from Dokcer")?;

    Ok(data.access_token)
}

// key is from the get_access_token_dockerhub
// Return digest
pub async fn get_disgest<'a>(repo: &ApiCall<'a>, key: &String) -> Result<String> {
    let client = reqwest::Client::new();

    let res = client
        .get(format!(
            "https://hub.docker.com/v2/namespaces/{}/repositories/{}/tags/{}",
            repo.namespace, repo.repo, repo.tag
        ))
        .bearer_auth(key)
        .send()
        .await
        .context("Error when create access token")?;

    let data = res
        .json::<RepoTag>()
        .await
        .context("Failed to parse the JSON token from Dokcer")?;

    Ok(data.digest)
}
