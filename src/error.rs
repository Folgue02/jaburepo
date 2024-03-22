use crate::repository::Artifact;
use url::ParseError;

/// An error while performing an operation on the local
/// repository. This can also include operations such fetching a
/// remote artifact and saving it to the local repository.
#[derive(Debug)]
pub enum RepositoryOperationError {
    /// Represents an error while fetching an artifact.
    /// It also contains the error of the request.
    GetError(reqwest::Error),

    /// The name of the artifact cannot be converted into
    /// a URL.
    InvalidArtifactName(ParseError),

    /// An error caused when interacting with the local
    /// repository.
    IoError(std::io::Error),

    /// Malformed XML being parsed will result in this error.
    ///
    /// ***NOTE***: There are many chances this error is given when 
    /// trying to parse the pom of an artifact.
    SerdeXmlParsingError(serde_xml_rs::Error),
}

impl std::fmt::Display for RepositoryOperationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self) // TODO
    }
}

impl From<ParseError> for RepositoryOperationError {
    fn from(value: ParseError) -> Self {
        Self::InvalidArtifactName(value)
    }
}

impl From<std::io::Error> for RepositoryOperationError {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value)
    }
}

impl From<serde_xml_rs::Error> for RepositoryOperationError {
    fn from(value: serde_xml_rs::Error) -> Self {
        Self::SerdeXmlParsingError(value)
    }
}

impl From<reqwest::Error> for RepositoryOperationError {
    fn from(value: reqwest::Error) -> Self {
        Self::GetError(value)
    }
}

impl std::error::Error for RepositoryOperationError {}
