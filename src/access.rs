use bollard::Docker;
use bollard::query_parameters::ListImagesOptionsBuilder;
use oci_client::secrets::RegistryAuth;
use oci_client::{ParseError, Reference};

pub async fn compare_all_digest() -> Result<(), String> {
    let docker = Docker::connect_with_local_defaults().map_err(|e| e.to_string())?;
    let options = ListImagesOptionsBuilder::default().digests(true).build();
    let images = &docker
        .list_images(Some(options))
        .await
        .map_err(|e| e.to_string())?;

    let client = oci_client::Client::default();
    for image in images {
        let tags_list = &image.repo_tags;
        if !tags_list.is_empty() {
            let tag = &tags_list[0];
            let repo_digest_list = &image.repo_digests;
            let repo_digest = &repo_digest_list[0]; //need to think about the case that
            //repo_digests is empty, normally wont happen
            let reference: &Reference = &tag.parse().map_err(|e: ParseError| e.to_string())?; //error here need to handle

            let remote_digest = oci_client::Client::fetch_manifest_digest(
                &client,
                reference,
                &RegistryAuth::Anonymous,
            )
            .await
            .map_err(|e| e.to_string())?;

            let local_digest = repo_digest.split("@").collect::<Vec<_>>()[1];

            if local_digest != remote_digest {
                println!("Need update on {tag:?}");
            }
        }
    }
    Ok(())
}
