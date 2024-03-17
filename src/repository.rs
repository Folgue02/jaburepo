use std::{
    path::PathBuf,
    fs::{File, read_dir},
    collections::HashSet,
    io::copy
};
use serde::Deserialize;
use url::{Url, ParseError};

/// A Java Artifact
#[derive(Debug, PartialEq, Deserialize)]
pub struct Artifact {
    #[serde(rename = "groupId")]
    pub group_id: String,

    #[serde(rename = "artifactId")]
    pub artifact_id: String,

    pub version: String
}

/// Represents a local repository. This structure can be used
/// for managing the local repository, creating, reading and
/// deleting artifacts. 
pub struct Repository {
    base_path: PathBuf
}

impl Default for Repository {
    fn default() -> Self {
        Self {
            base_path: std::env::current_dir().unwrap().join("./repo")
        }
    }
}

impl Repository {
    pub fn new<T: Into<PathBuf>>(base_path: T) -> Self {
        Self {
            base_path: base_path.into()
        }
    }
    
    fn artifact_as_dirname(&self, artifact: &Artifact) -> PathBuf {
        self.base_path.join(&artifact.group_id)
            .join(&artifact.artifact_id)
    }

    pub fn artifact_jar_path(&self, artifact: &Artifact) -> PathBuf {
        let mut path = self.artifact_as_dirname(artifact)
            .join(&artifact.version);
        path.set_extension(
            path.extension()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string() + ".jar"
        );
        path
    }

    pub fn artifact_pom_path(&self, artifact: &Artifact) -> PathBuf {
        let mut path = self.artifact_as_dirname(artifact)
            .join(&artifact.version);
        path.set_extension(
            path.extension()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string() + ".xml"
        );
        path
    }

    /// Checks if the artifact exists.
    pub fn exists(&self, artifact: &Artifact) -> bool { 
        self.artifact_pom_path(artifact).exists()
    }

    /// Writes the jar's content to its correspondent file in the repository.
    ///
    /// Sample location of an artifact's jar: *`group_id/artifact_id/version.xml*`
    pub fn save_artifact<T: AsRef<[u8]>>(&self, artifact: &Artifact, artifact_content: T) -> std::io::Result<PathBuf> {
        let artifact_dirname = self.artifact_as_dirname(artifact);
        let artifact_jar_path = self.artifact_jar_path(artifact);

        std::fs::create_dir_all(artifact_dirname)?;
        copy(&mut artifact_content.as_ref(), &mut File::create(&artifact_jar_path)?)?;
        Ok(artifact_jar_path)
    }

    /// Writes the pom content to its correspondent file in the repository.
    ///
    /// Sample location of a xml: *`group_id/artifact_id/version.pom.xml*`
    pub fn save_pom<T: AsRef<[u8]>>(&self, artifact: &Artifact, artifact_content: T) -> std::io::Result<PathBuf> {
        let artifact_dirname = self.artifact_as_dirname(artifact);
        let artifact_pom_path = self.artifact_pom_path(artifact);

        std::fs::create_dir_all(artifact_dirname)?;
        copy(&mut artifact_content.as_ref(), &mut File::create(&artifact_pom_path)?)?;
        Ok(artifact_pom_path)
    }

    /// Returns an immutable reference to the path where the repository is located at.
    pub fn base_path(&self) -> &PathBuf {
        &self.base_path
    }

    /// Returns a collection of stirngs
    pub fn get_artifact_available_versions(&self, artifact: &Artifact) -> Option<HashSet<String>> {
        Some(
            read_dir(self.artifact_as_dirname(artifact)).ok()?.into_iter()
                .filter_map(|element| element.ok())
                .map(|element| element.path().file_stem().unwrap().to_string_lossy().to_string())
                .collect::<HashSet<String>>()
        )
    }
}

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
    /// Base URL of the artifact (*it doesn't contain the '.jar', '.xml' etc... extension
    /// of the file to download*).
    fn artifact_url(&self, artifact: &Artifact) -> Result<Url, ParseError> {
        //self.remote_url
        let segmented_group_id = artifact.group_id.split(".")
            .collect::<Vec<&str>>();
        let mut remote_url = self.remote_url.clone();
        
        remote_url = remote_url.join("maven2")?;
        // Join the segmented group_id into the 
        // same url.
        segmented_group_id.iter()
            .try_fold(remote_url, |url, segment| url.join(segment))
    }

    /// Generates the URL of the given artifact's jar. This method might fail
    /// if the passed artifact contains unexpected characters that might not
    /// be able to be represented in the URL.
    pub fn jar_artifact_url(&self, artifact: &Artifact) -> Result<Url, ParseError> {
        let base_artifact_url = self.artifact_url(artifact)?;
        Url::parse(&(base_artifact_url.to_string() + ".jar"))
    }

    /// Generates the URL of the given artifact's pom. This method might fail
    /// if the passed artifact contains unexpected characters that might not
    /// be able to be represented in the URL.
    pub fn pom_artifact_url(&self, artifact: &Artifact) -> Result<Url, ParseError> {
        let base_artifact_url = self.artifact_url(artifact)?;
        Url::parse(&(base_artifact_url.to_string() + ".xml"))
    }
}
