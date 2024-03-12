use std::{
    path::PathBuf,
    fs::File,
    io::copy
};

/// A Java Artifact
pub struct Artifact {
    pub group_id: String,
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
    fn artifact_as_dirname(&self, artifact: &Artifact) -> PathBuf {
        self.base_path.join(&artifact.group_id)
            .join(&artifact.artifact_id)
    }

    fn artifact_jar_path(&self, artifact: &Artifact) -> PathBuf {
        let mut path = self.artifact_as_dirname(artifact)
            .join(&artifact.version);
        path.set_extension("jar");
        path
    }

    fn artifact_pom_path(&self, artifact: &Artifact) -> PathBuf {
        let mut path = self.artifact_as_dirname(artifact)
            .join(&artifact.version);
        path.set_extension("pom.xml");
        path
    }

    /// Checks if the artifact exists.
    pub fn exists(&self, artifact: &Artifact) -> bool { 
        self.artifact_pom_path(artifact).exists()
    }

    /// Writes the jar's content to its correspondent file in the repository.
    ///
    /// Sample location of an artifact's jar: *`group_id/artifact_id/version.pom.xml*`
    pub fn save_artifact(&self, artifact: &Artifact, artifact_content: Vec<u8>) -> std::io::Result<PathBuf> {
        let artifact_dirname = self.artifact_as_dirname(artifact);
        let artifact_jar_path = self.artifact_jar_path(artifact);
        std::fs::create_dir_all(artifact_dirname)?;
        copy(&mut artifact_content.as_slice(), &mut File::create(self.artifact_jar_path(artifact))?)?;
        Ok(artifact_jar_path)
    }

    /// Writes the pom content to its correspondent file in the repository.
    ///
    /// Sample location of a pom.xml: *`group_id/artifact_id/version.pom.xml*`
    pub fn save_pom(&self, artifact: &Artifact, artifact_content: Vec<u8>) -> std::io::Result<PathBuf> {
        let artifact_dirname = self.artifact_as_dirname(artifact);
        let artifact_pom_path = self.artifact_pom_path(artifact);
        std::fs::create_dir_all(artifact_dirname)?;
        copy(&mut artifact_content.as_slice(), &mut File::create(self.artifact_jar_path(artifact))?)?;
        Ok(artifact_pom_path)
    }

    /// Returns an immutable reference to the path where the repository is located at.
    pub fn base_path(&self) -> &PathBuf {
        &self.base_path
    }

    pub fn get_artifact_available_versions(&self) -> Option<Vec<String>> {
        let read_dir = match std::fs::read_dir(&self.base_path) {
            Ok(rd) => rd,
            Err(_) => return None
        };
        todo!()
    }
}
