use git2::{ApplyOptions, Repository};
use serde::{Deserialize, Serialize};

use crate::patch::Patch;
use thiserror::Error;

/// PatchError enumerates all possible errors returned by this library.
#[derive(Error, Debug)]
pub enum PatchError {
    /// Patch cannot be applied
    #[error("Patch cannot be applied")]
    ApplyError { source: git2::Error },

    /// Header field
    #[error("Header field is missing")]
    HeaderFieldError { message: String },
}

/// Validations on a cloned repository
pub(crate) trait CheckRepo {
    fn apply(repo: &Repository, patch: &Patch) -> Result<String, PatchError>;
}

/// Validations of a patch
pub(crate) trait LintPatch {
    fn check(patch: &Patch) -> Result<String, PatchError>;
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Level {
    Warning,
    Error,
}

pub fn icon(level: &Level) -> &'static str {
    match level {
        Level::Error => "❌",
        Level::Warning => "⚠",
    }
}

#[derive(Serialize, Deserialize)]
pub struct ApplyPatch {
    pub level: Level,
}

#[derive(Serialize, Deserialize)]
pub struct Summary {
    pub level: Level,
}

#[derive(Serialize, Deserialize)]
struct Config<T, U>
where
    T: LintPatch,
    U: CheckRepo,
{
    lints: Vec<T>,
    checks: Vec<U>,
}

pub fn print_config() {
    let config = Config {
        lints: vec![Summary {
            level: Level::Error,
        }],
        checks: vec![ApplyPatch {
            level: Level::Error,
        }],
    };
    println!("config {:?}", serde_yaml::to_string(&config));
}

impl CheckRepo for ApplyPatch {
    /// Applies the patch and wrappes failures in a PatchError
    fn apply(repo: &Repository, patch: &Patch) -> Result<String, PatchError> {
        let location = git2::ApplyLocation::WorkDir;
        let options = &mut ApplyOptions::new();
        repo.apply(&patch.diff, location, Some(options))
            .map_err(|source| PatchError::ApplyError { source })?;
        Ok("apply patch".to_owned())
    }
}

impl LintPatch for Summary {
    /// Validate the message summary
    fn check(patch: &Patch) -> Result<String, PatchError> {
        if patch.header.summary.is_empty() {
            return Err(PatchError::HeaderFieldError {
                message: "summary is empty".to_owned(),
            });
        };
        Ok("summary present".to_owned())
    }
}

#[test]
fn test_summary() {
    use std::path::Path;
    fn is_error_patch_file(path_str: &str) {
        let patch = Patch::from_file(Path::new(path_str));
        assert!(Summary::check(&patch).is_err());
    }

    fn is_ok_patch_file(path_str: &str) {
        let patch = Patch::from_file(Path::new(path_str));
        assert!(Summary::check(&patch).is_ok());
    }

    is_error_patch_file("tests/files/CommitMessage.test_commit_message_presence.fail");
    is_ok_patch_file("tests/files/CommitMessage.test_commit_message_presence.pass");
}
