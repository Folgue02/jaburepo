use crate::repository::{RemoteRepository, Repository, Artifact};
use url::Url;
use std::path::PathBuf;

fn create_temp_repository() -> std::io::Result<Repository> {
    let tmp_dir = tempdir::TempDir::new("jaburepository")?.path().to_path_buf();
    Ok(Repository::new(tmp_dir))
}

#[test]
fn download_artifact() -> Result<(), Box<dyn std::error::Error>> {
    let repo = create_temp_repository()?;
    
    // Download artifacts from the maven repository.
    let remote_repo = RemoteRepository::default();
    //group: 'org.junit.jupiter', name: 'junit-jupiter-api', version: '5.10.2'

    let target_artifact = Artifact {
        group_id: "org.junit.jupiter".to_string(),
        artifact_id: "junit-jupiter-api".to_string(),
        version: "5.10.2".to_string()
    };

    let response = reqwest::blocking::get(remote_repo.jar_artifact_url(&target_artifact)?)?;

    repo.save_artifact(&target_artifact, response.bytes()?)?;


    let versions = repo.get_artifact_available_versions(&target_artifact).unwrap();
    assert_eq!(versions.get("5.10.2"), Some(&"5.10.2".to_string()));
    
    Ok(())
}

#[test]
fn jar_artifact_path_forming() {
    let repo = create_temp_repository().unwrap();
    let artifact = Artifact {
        group_id: "group".to_string(),
        artifact_id: "artifact".to_string(),
        version: "1.0.0".to_string()
    };

    let jar_path = repo.artifact_jar_path(&artifact);
    let expected = repo.base_path()
        .join(artifact.group_id)
        .join(artifact.artifact_id)
        .join(artifact.version + ".jar");

    assert_eq!(expected, jar_path);
}
