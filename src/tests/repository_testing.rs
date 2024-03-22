use crate::repository::{Artifact, RemoteRepository, Repository};
use std::path::PathBuf;
use url::Url;

fn sample_artifact() -> Artifact {
    Artifact::new("org.junit.jupiter", "junit-jupiter-api", "5.10.2")
}

fn create_temp_repository() -> std::io::Result<Repository> {
    let tmp_dir = tempdir::TempDir::new("jaburepository")?
        .path()
        .to_path_buf();
    Ok(Repository::new(tmp_dir))
}

#[test]
fn download_artifact() -> Result<(), Box<dyn std::error::Error>> {
    let repo = create_temp_repository()?;

    // Download artifacts from the maven repository.
    let remote_repo = RemoteRepository::default();

    let target_artifact = sample_artifact();
    let response = reqwest::blocking::get(remote_repo.jar_artifact_url(&target_artifact)?)?;

    repo.save_artifact(&target_artifact, response.bytes()?)?;

    let versions = repo
        .get_artifact_available_versions(&target_artifact)
        .unwrap();
    assert_eq!(versions.get("5.10.2"), Some(&"5.10.2".to_string()));

    Ok(())
}

#[test]
fn jar_artifact_path_forming() {
    let repo = create_temp_repository().unwrap();
    let artifact = Artifact {
        group_id: "group".to_string(),
        artifact_id: "artifact".to_string(),
        version: "1.0.0".to_string(),
    };

    let jar_path = repo.artifact_jar_path(&artifact);
    let expected = repo
        .base_path()
        .join(artifact.group_id)
        .join(artifact.artifact_id)
        .join(artifact.version + ".jar");

    assert_eq!(expected, jar_path);
}

#[test]
fn base_artifact_url() {
    let remote_repository = RemoteRepository::default();
    let expected = "https://repo1.maven.org/maven2/org/junit/jupiter/junit-jupiter-api/5.10.2/junit-jupiter-api-5.10.2";

    assert_eq!(
        expected,
        remote_repository
            .artifact_url(&sample_artifact())
            .unwrap()
            .as_str()
    )
}

#[test]
fn save_from_remote_test() {
    let repo = create_temp_repository().unwrap();
    let remote_repository = RemoteRepository::default();
    let target_artifact = sample_artifact();

    repo.save_from_remote(&target_artifact, &remote_repository, &|_, _| {})
        .unwrap();
}

#[test]
fn recursive_save_from_remote_test() {
    let repo = create_temp_repository().unwrap();
    let remote_repository = RemoteRepository::default();
    let target_artifact = sample_artifact();

    repo.recursive_save_from_remote(&target_artifact, &remote_repository, |pom_url, jar_url| {
        println!("Downloading {pom_url} (pom) and {jar_url} (jar_url)");
    })
    .unwrap();
}
