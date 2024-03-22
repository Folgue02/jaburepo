pub mod repository;
pub mod utils;
pub mod error;

#[cfg(test)]
mod tests;

pub type RepositoryOperationResult<T> = Result<T, error::RepositoryOperationError>;
