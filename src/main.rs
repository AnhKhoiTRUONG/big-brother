mod access;
mod config;
use anyhow::Context;
use bollard::Docker;
use bollard::query_parameters::ListImagesOptionsBuilder;

#[tokio::main]
async fn main() {
    let config = config::Config::init();

    if let Ok(key_docker) = access::get_access_token_dockerhub(&config).await {
        let docker = Docker::connect_with_local_defaults().unwrap();
        let options = ListImagesOptionsBuilder::default().digests(true).build();
        let images = &docker.list_images(Some(options)).await.unwrap();

        for image in images {
            let tags = &image.repo_tags;
            if !tags.is_empty() {
                let call1 = access::ApiCall::parse(&tags[0]);
                if call1.namespace != "ghcr.io" {
                    if let Ok(remote_digest) = access::get_disgest(&call1, &key_docker)
                        .await
                        .context("Failed to retreate the remote digest")
                    {
                        let repo_digests = &image.repo_digests;
                        let local_digest = repo_digests[0].split("@").collect::<Vec<_>>()[1];
                        if remote_digest != local_digest {
                            println!(
                                "{} vs {}\nNeed update on {:?}",
                                remote_digest, local_digest, tags
                            );
                        }
                    }
                }
            }
        }
    }
}
