use crate::notif::{self};
use crate::parse_yaml::DiscordConfig;
use bollard::Docker;
use bollard::query_parameters::ListImagesOptionsBuilder;
use chrono_tz::Tz;
use discord_webhook2::error::DiscordWebhookError;
use docker_image::DockerImage;
use oci_client::secrets::RegistryAuth;
use oci_client::{ParseError, Reference};
use std::fmt::{self};

//Maybe put the error handle in another file and use thiserror crate
//but now im kinda happy with it
#[derive(Debug)]
pub enum AccessError {
    Docker(bollard::errors::Error),
    Oci(ParseError),
    Discord(DiscordWebhookError),
}

impl fmt::Display for AccessError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AccessError::Docker(ref err) => write!(f, "Docker connectin error: {}", err),
            AccessError::Oci(ref err) => write!(f, "Remote repo error: {}", err),
            AccessError::Discord(ref err) => write!(f, "Discord Webhook Error: {}", err),
        }
    }
}

impl From<bollard::errors::Error> for AccessError {
    fn from(err: bollard::errors::Error) -> AccessError {
        AccessError::Docker(err)
    }
}

impl From<ParseError> for AccessError {
    fn from(err: ParseError) -> AccessError {
        AccessError::Oci(err)
    }
}

impl From<DiscordWebhookError> for AccessError {
    fn from(err: DiscordWebhookError) -> AccessError {
        AccessError::Discord(err)
    }
}

pub fn get_hublink(image: &DockerImage) -> String {
    let mut hublink = String::from("https://");

    match image.registry {
        Some(_) => hublink.push_str("github.com/"),
        None => hublink.push_str("hub.docker.com/r/"),
    }

    hublink.push_str(image.name.as_str());

    hublink
}

pub async fn compare_all_digest(
    tz: &Tz,
    maybe_discord_config: &Option<DiscordConfig>,
) -> Result<(), AccessError> {
    let docker = Docker::connect_with_local_defaults()?;
    let options = ListImagesOptionsBuilder::default().digests(true).build();

    //seems like unwrap is reasonable here because when i can't connect to docker it need to be stop
    let images = &docker.list_images(Some(options)).await.unwrap();

    let client = oci_client::Client::default();
    for image in images {
        let tags_list = &image.repo_tags;
        if !tags_list.is_empty() {
            let tag = &tags_list[0];
            let repo_digest_list = &image.repo_digests;

            //need to think about the case that repo_digests is empty, normally wont happen
            let repo_digest = &repo_digest_list[0];
            let reference: &Reference = &tag.parse()?;

            // Can I handle the error like this? Should i make a general thing?
            let remote_digest = match oci_client::Client::fetch_manifest_digest(
                &client,
                reference,
                &RegistryAuth::Anonymous,
            )
            .await
            {
                Ok(digest) => digest,
                Err(e) => {
                    eprintln!("Your remote repo link seems not right {e}");
                    continue;
                }
            };

            let local_digest = repo_digest.split("@").collect::<Vec<_>>()[1];

            if local_digest != remote_digest {
                if let Some(discord_conf) = maybe_discord_config {
                    let created = notif::get_current_time(tz);
                    let hublink_string = get_hublink(&DockerImage::parse(tag).unwrap());
                    let hublink = &hublink_string.as_str();

                    let discord_notif =
                        notif::DiscordNotif::new(tag, &created, &remote_digest, hublink);
                    notif::DiscordNotif::send_discord_notif(
                        &discord_notif,
                        &discord_conf.webhook_url,
                    )
                    .await?;
                } else {
                    println!("Need update on {tag:?}");
                }
            }
        }
    }
    Ok(())
}
