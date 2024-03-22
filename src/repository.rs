use crate::error::RepositoryOperationError;
use reqwest::blocking::get;
use serde::Deserialize;
use std::{
    collections::HashSet,
    fs::{read_dir, File},
    io::copy,
    path::PathBuf,
};
use url::{ParseError, Url};

/// A Java Artifact
#[derive(Debug, PartialEq, Deserialize, Clone)]
pub struct Artifact {
    #[serde(rename = "groupId")]
    pub group_id: String,

    #[serde(rename = "artifactId")]
    pub artifact_id: String,

    pub version: String,
}

impl Artifact {
    pub fn new<T: Into<String>>(group_id: T, artifact_id: T, version: T) -> Self {
        Self {
            group_id: group_id.into(),
            artifact_id: artifact_id.into(),
            version: version.into(),
        }
    }
}

/// Represents a local repository. This structure can be used
/// for managing the local repository, creating, reading and
/// deleting artifacts.
pub struct Repository {
    base_path: PathBuf,
}

impl Default for Repository {
    fn default() -> Self {
        let home_directory = if cfg!(windows) {
            std::env::var("USERPROFILE").unwrap_or_default()
        } else {
            std::env::var("HOME").unwrap_or_default()
        };
        Self {
            base_path: PathBuf::from(home_directory).join("./repo"),
        }
    }
}

impl Repository {
    pub fn new<T: Into<PathBuf>>(base_path: T) -> Self {
        Self {
            base_path: base_path.into(),
        }
    }

    fn artifact_as_dirname(&self, artifact: &Artifact) -> PathBuf {
        self.base_path
            .join(&artifact.group_id)
            .join(&artifact.artifact_id)
    }

    pub fn artifact_jar_path(&self, artifact: &Artifact) -> PathBuf {
        let mut path = self.artifact_as_dirname(artifact).join(&artifact.version);
        path.set_extension(
            path.extension()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string()
                + ".jar",
        );
        path
    }

    pub fn artifact_pom_path(&self, artifact: &Artifact) -> PathBuf {
        let mut path = self.artifact_as_dirname(artifact).join(&artifact.version);
        path.set_extension(
            path.extension()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string()
                + ".pom",
        );
        path
    }

    /// Checks if the artifact exists.
    pub fn exists(&self, artifact: &Artifact) -> bool {
        self.artifact_pom_path(artifact).exists()
    }

    /// Writes the jar's content to its correspondent file in the repository.
    ///
    /// Sample location of an artifact's jar: `group_id/artifact_id/version.xml`
    pub fn save_artifact<T: AsRef<[u8]>>(
        &self,
        artifact: &Artifact,
        artifact_content: T,
    ) -> std::io::Result<PathBuf> {
        let artifact_dirname = self.artifact_as_dirname(artifact);
        let artifact_jar_path = self.artifact_jar_path(artifact);

        std::fs::create_dir_all(artifact_dirname)?;
        copy(
            &mut artifact_content.as_ref(),
            &mut File::create(&artifact_jar_path)?,
        )?;
        Ok(artifact_jar_path)
    }

    /// Writes the pom content to its correspondent file in the repository.
    ///
    /// Sample location of a xml: *`group_id/artifact_id/version.pom.xml*`
    pub fn save_pom<T: AsRef<[u8]>>(
        &self,
        artifact: &Artifact,
        artifact_content: T,
    ) -> std::io::Result<PathBuf> {
        let artifact_dirname = self.artifact_as_dirname(artifact);
        let artifact_pom_path = self.artifact_pom_path(artifact);

        std::fs::create_dir_all(artifact_dirname)?;
        copy(
            &mut artifact_content.as_ref(),
            &mut File::create(&artifact_pom_path)?,
        )?;
        Ok(artifact_pom_path)
    }

    /// Returns an immutable reference to the path where the repository is located at.
    pub fn base_path(&self) -> &PathBuf {
        &self.base_path
    }

    /// Returns a collection of stirngs
    pub fn get_artifact_available_versions(&self, artifact: &Artifact) -> Option<HashSet<String>> {
        Some(
            read_dir(self.artifact_as_dirname(artifact))
                .ok()?
                .into_iter()
                .filter_map(|element| element.ok())
                .map(|element| {
                    element
                        .path()
                        .file_stem()
                        .unwrap()
                        .to_string_lossy()
                        .to_string()
                })
                .collect::<HashSet<String>>(),
        )
    }

    /// Recursive saves the specified artifact, as well as
    /// its dependencies in the local repository, using the remote
    /// repository given.
    ///
    /// # Parameters
    ///
    /// * artifact - Artifact to save.
    /// * remote_repository - The remote repository from where to
    /// download the artifacts.
    /// * action_per_download - Action that gets called before every download,
    /// being passed the pom's url as first parameter, and the jar's url as the
    /// second parameter.
    ///
    /// # See
    /// * [`Self::save_from_remote`]
    ///
    pub fn recursive_save_from_remote<T>(
        &self,
        artifact: &Artifact,
        remote_repository: &RemoteRepository,
        action_per_download: T,
    ) -> crate::RepositoryOperationResult<()>
    where
        T: Fn(String, String) -> (),
    {
        let mut artifact_list: Vec<Artifact> = vec![artifact.clone()];
        while let Some(dep) = artifact_list.pop() {
            self.save_from_remote(&dep, remote_repository, &action_per_download)?;

            let artifact_pom = std::fs::read_to_string(self.artifact_pom_path(artifact))?;
            artifact_list.append(&mut crate::utils::dependencies_in_pom(artifact_pom)?);
            artifact_list = artifact_list
                .into_iter()
                .filter(|a| !self.exists(a))
                .collect();
        }

        Ok(())
    }

    /// Saves a given artifact to the local repository, downloading it from the
    /// specified remote repository. Before doing so, the `action_per_download` function
    /// gets called, if there's one.
    ///
    /// # Parameters
    ///
    /// * artifact - Artifact to save.
    /// * remote_repository - The remote repository from where to
    /// download the artifacts.
    /// * action_per_download - Action that gets called before every download,
    /// being passed the pom's url as first parameter, and the jar's url as the
    /// second parameter.
    ///
    /// # See
    /// * [`Artifact::recursive_save_from_remote`]
    pub fn save_from_remote<T>(
        &self,
        artifact: &Artifact,
        remote_repository: &RemoteRepository,
        action_per_download: &T,
    ) -> Result<(), RepositoryOperationError>
    where
        T: Fn(String, String) -> (),
    {
        // TODO: Check if the artifact already exists in the local
        // repository.
        let pom_url = remote_repository.pom_artifact_url(artifact)?.to_string();
        let jar_url = remote_repository.jar_artifact_url(artifact)?.to_string();

        action_per_download(pom_url.to_string(), jar_url.to_string());

        let pom_response = reqwest::blocking::get(pom_url)?;
        let jar_response = reqwest::blocking::get(jar_url)?;

        self.save_artifact(artifact, jar_response.bytes()?)?;
        self.save_pom(artifact, pom_response.bytes()?)?;
        Ok(())
    }
}

/// Represents a remote repository. This struct is used to
/// fetch artifacts from the mentioned remote repository.
///
/// * Local repository: [`crate::repository::Repository`]
pub struct RemoteRepository {
    pub remote_url: Url,
}

// https://repo1.maven.org/maven2/org/junit/jupiter/junit-jupiter-api/5.10.2/junit-jupiter-api-5.10.2.jar

impl Default for RemoteRepository {
    fn default() -> Self {
        Self {
            remote_url: Url::parse("https://repo1.maven.org/").unwrap(),
        }
    }
}

impl RemoteRepository {
    /// Base URL of the artifact (*it doesn't contain the '.jar', '.xml' etc... extension
    /// of the file to download*).
    pub fn artifact_url(&self, artifact: &Artifact) -> Result<Url, ParseError> {
        //self.remote_url
        let segmented_group_id = artifact.group_id.split(".");
        let mut remote_url = self.remote_url.clone();

        remote_url = remote_url.join("maven2")?;
        // Join the segmented group_id into the
        // same url.

        segmented_group_id.for_each(|segment| {
            remote_url.path_segments_mut().unwrap().push(segment);
        });

        remote_url
            .path_segments_mut()
            .unwrap()
            .push(artifact.artifact_id.as_str())
            .push(artifact.version.as_str())
            .push(format!("{}-{}", artifact.artifact_id, artifact.version).as_str());

        Ok(remote_url)
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
        Url::parse(&(base_artifact_url.to_string() + ".pom"))
    }
}
