//! Set gitattributes via patterns and key-value pairs.

pub use git2::{Error, Repository};

/// A trait which provides methods for settings attributes in a Git repository.
pub trait SetAttr {
    /// Set attributes in the repository's root `.gitattributes` file.
    fn set_attr(&self, pattern: &str, attributes: &[&str]) -> Result<(), Error>;
}

impl SetAttr for Repository {
    fn set_attr(&self, pattern: &str, attributes: &[&str]) -> Result<(), Error> {
        todo!()
    }
}
