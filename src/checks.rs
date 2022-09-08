use git2::{ApplyOptions, Repository};
use serde::{Deserialize, Serialize};

use crate::patch::Patch;
use thiserror::Error;

#[derive(Debug)]
pub struct TestMetaInfo {
    pub name: String,
}

/// PatchError enumerates all possible errors returned by this library.
#[derive(Error, Debug)]
pub enum PatchError {
    /// Patch cannot be applied
    #[error("Patch cannot be applied. meta: {source}")]
    ApplyError { source: git2::Error },

    /// Header field
    #[error("Header field is missing")]
    HeaderFieldError { message: String },
}

pub struct LintResult {
    pub meta_info: TestMetaInfo,
    pub test_result: Result<(), PatchError>,
}

/// Validations on a cloned repository
pub(crate) trait CheckRepo {
    fn apply(repo: &Repository, patch: &Patch) -> LintResult;
}

/// Validations of a patch
pub(crate) trait LintPatch {
    fn check(patch: &Patch) -> LintResult;
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Level {
    Error,
    Warning,
    Skip,
}

pub fn icon(level: &Level) -> &'static str {
    match level {
        Level::Skip => "",
        Level::Error => "❌",
        Level::Warning => "⚠",
    }
}

#[derive(Serialize, Deserialize)]
pub struct ApplyPatch {}

#[derive(Serialize, Deserialize)]
pub struct Summary {}

impl CheckRepo for ApplyPatch {
    /// Applies the patch and wrappes failures in a PatchError
    fn apply(repo: &Repository, patch: &Patch) -> LintResult {
        let location = git2::ApplyLocation::WorkDir;
        let meta_info = TestMetaInfo {
            name: "apply patch".to_owned(),
        };
        let options = &mut ApplyOptions::new();
        let result = repo
            .apply(&patch.diff, location, Some(options))
            .map_err(|source| PatchError::ApplyError { source });

        LintResult {
            meta_info,
            test_result: result,
        }
    }
}

impl LintPatch for Summary {
    /// Validate the message summary
    fn check(patch: &Patch) -> LintResult {
        let meta_info = TestMetaInfo {
            name: "summary".to_owned(),
        };
        let mut result = Ok(());
        if patch.header.summary.is_empty() {
            result = Err(PatchError::HeaderFieldError {
                message: "summary is empty".to_owned(),
            });
        }

        LintResult {
            meta_info,
            test_result: result,
        }
    }
}

#[test]
fn test_summary() {
    use std::path::Path;

    fn is_error_patch_file(path_str: &str) {
        let patch = Patch::from_file(Path::new(path_str));
        assert!(Summary::check(&patch).test_result.is_err());
    }

    fn is_ok_patch_file(path_str: &str) {
        let patch = Patch::from_file(Path::new(path_str));
        assert!(Summary::check(&patch).test_result.is_ok());
    }

    is_error_patch_file("tests/files/CommitMessage.test_commit_message_presence.fail");
    is_ok_patch_file("tests/files/CommitMessage.test_commit_message_presence.pass");
}
