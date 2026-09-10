use crate::notif::{self};
use crate::parse_yaml::DiscordConfig;
use bollard::Docker;
use bollard::query_parameters::ListImagesOptionsBuilder;
use chrono_tz::Tz;
use oci_client::secrets::RegistryAuth;
use oci_client::{ParseError, Reference};
use std::fmt;

#[derive(Debug)]
pub enum AccessError {
    DockerError(bollard::errors::Error),
    OciError(ParseError),
}

impl fmt::Display for AccessError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            // Both underlying errors already impl `Display`, so we defer to
            // their implementations.
            AccessError::DockerError(ref err) => write!(f, "Docker connectin error: {}", err),
            AccessError::OciError(ref err) => write!(f, "Remote repo error: {}", err),
        }
    }
}

impl From<bollard::errors::Error> for AccessError {
    fn from(err: bollard::errors::Error) -> AccessError {
        AccessError::DockerError(err)
    }
}

impl From<ParseError> for AccessError {
    fn from(err: ParseError) -> AccessError {
        AccessError::OciError(err)
    }
}

pub async fn compare_all_digest(
    tz: &Tz,
    maybe_discord_config: &Option<DiscordConfig>,
) -> Result<(), AccessError> {
    let docker = Docker::connect_with_local_defaults()?;
    let options = ListImagesOptionsBuilder::default().digests(true).build();
    let images = &docker.list_images(Some(options)).await?;

    let client = oci_client::Client::default();
    for image in images {
        let tags_list = &image.repo_tags;
        if !tags_list.is_empty() {
            let tag = &tags_list[0];
            let repo_digest_list = &image.repo_digests;
            let repo_digest = &repo_digest_list[0]; //need to think about the case that repo_digests is empty, normally wont happen
            let reference: &Reference = &tag.parse()?; //error here need to handle

            let remote_digest = match oci_client::Client::fetch_manifest_digest(
                &client,
                reference,
                &RegistryAuth::Anonymous,
            )
            .await
            {
                Ok(digest) => digest,
                Err(e) => {
                    eprintln!("{e:?}");
                    continue;
                }
            };

            let local_digest = repo_digest.split("@").collect::<Vec<_>>()[1];

            if local_digest != remote_digest {
                if let Some(discord_conf) = maybe_discord_config {
                    let created = notif::get_current_time(tz);
                    let discord_notif =
                        notif::DiscordNotif::new(tag, &created, &remote_digest, "hehe");
                    notif::DiscordNotif::send_discord_notif(
                        &discord_notif,
                        &discord_conf.webhook_url,
                    )
                    .await
                    .unwrap();
                } else {
                    println!("Need update on {tag:?}");
                }
            }
        }
    }
    Ok(())
}
