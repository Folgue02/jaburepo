use crate::repository::Artifact;
use url::{Url, ParseError};

/// Represents a remote repository. This struct is used to 
/// fetch artifacts from the mentioned remote repository.
///
/// * Local repository: [`crate::repository::Repository`]
pub struct RemoteRepository {
    pub remote_url: Url
}

// https://repo1.maven.org/maven2/org/junit/jupiter/junit-jupiter-api/5.10.2/junit-jupiter-api-5.10.2.jar

impl Default for RemoteRepository {
    fn default() -> Self {
        Self {
            remote_url: Url::parse("https://repo1.maven.org/").unwrap()
        }
    }
}

impl RemoteRepository {
    /// Base URL of the artifact (*it doesn't contain the '.jar', '.pom' etc... extension
    /// of the file to download*).
    fn artifact_url(&self, artifact: &Artifact) -> Result<Url, ParseError> {
        //self.remote_url
        let segmented_group_id = artifact.group_id.split(".")
            .collect::<Vec<&str>>();
        let mut remote_url = self.remote_url.clone();
        
        remote_url = remote_url.join("maven2")?;
        segmented_group_id.iter()
            .try_fold(remote_url, |url, segment| url.join(segment))
    }

    /// Generates the URL of the given artifact's jar.
    pub fn jar_artifact_url(&self, artifact: &Artifact) -> Result<Url, ParseError> {
        let base_artifact_url = self.artifact_url(artifact)?;
        Url::parse(&(base_artifact_url.to_string() + ".jar"))
    }

    /// Generates the URL of the given artifact's pom.
    pub fn pom_artifact_url(&self, artifact: &Artifact) -> Result<Url, ParseError> {
        let base_artifact_url = self.artifact_url(artifact)?;
        Url::parse(&(base_artifact_url.to_string() + ".pom.xml"))
    }
}
