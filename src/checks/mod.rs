use git2::{ApplyOptions, Repository};
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::{config::Config, patch::Patch};
use thiserror::Error;

use self::{commit_message::check_commit_message, mbox_author::test_author_valid};

pub mod commit_message;
pub mod mbox_author;

#[derive(Debug)]
pub struct TestMetaInfo {
    pub name: String,
}

pub fn run_all_tests(patch: Patch, repo: &Repository, config: &Config) -> [LintResult; 3] {
    [
        check_commit_message(&patch),
        apply_patch(repo, &patch),
        test_author_valid(&patch, &config.invalid_authors),
    ]
}

/// PatchError enumerates all possible errors returned by this library.
#[derive(Error, Debug)]
pub enum PatchError {
    /// Patch cannot be applied
    #[error("Patch cannot be applied. meta: {source}")]
    Apply { source: git2::Error },

    /// Header field
    #[error("Header field is missing")]
    HeaderField { message: String },

    #[error("Found an invalid author")]
    Author { matches: Vec<Regex> },
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
        .map_err(|source| PatchError::Apply { source });

    LintResult {
        meta_info,
        test_result: result,
    }
}
