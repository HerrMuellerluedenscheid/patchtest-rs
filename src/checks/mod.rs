use git2::{ApplyOptions, Repository};
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::{config::Config, patch::Patch};
use thiserror::Error;

use self::mbox_author::test_author_valid;

pub mod mbox_author;

#[derive(Debug)]
pub struct TestMetaInfo {
    pub name: String,
}

pub fn run_all_tests(patch: Patch, repo: &Repository, config: &Config) -> Vec<LintResult> {
    vec![
        check_summary(&patch),
        apply_patch(repo, &patch),
        test_author_valid(&patch, &config.invalid_authors),
    ]
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

    #[error("Found an invalid author")]
    AuthorError { matches: Vec<Regex> },
}

pub struct LintResult {
    pub meta_info: TestMetaInfo,
    pub test_result: Result<(), PatchError>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Level {
    Error,
    Warning,
    Skip,
}

/// Applies the patch and wrappes failures in a PatchError
pub fn apply_patch(repo: &Repository, patch: &Patch) -> LintResult {
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

/// Validate the message summary
pub fn check_summary(patch: &Patch) -> LintResult {
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

#[test]
fn test_summary() {
    use std::path::Path;

    fn is_error_patch_file(path_str: &str) {
        let patch = Patch::from_file(Path::new(path_str));
        assert!(check_summary(&patch).test_result.is_err());
    }

    fn is_ok_patch_file(path_str: &str) {
        let patch = Patch::from_file(Path::new(path_str));
        assert!(check_summary(&patch).test_result.is_ok());
    }

    is_error_patch_file("tests/files/CommitMessage.test_commit_message_presence.fail");
    is_ok_patch_file("tests/files/CommitMessage.test_commit_message_presence.pass");
}
