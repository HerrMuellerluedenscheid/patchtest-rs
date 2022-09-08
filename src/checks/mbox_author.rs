use crate::patch::Patch;
use regex::Regex;
use serde::{Deserialize, Serialize};

use super::{LintResult, PatchError, TestMetaInfo};

#[derive(Serialize, Deserialize, Default)]
pub struct InvalidAuthors {
    pub regular_expressions: Vec<String>,
}

pub fn test_author_valid(patch: &Patch, invalid: &InvalidAuthors) -> LintResult {
    let result: Vec<Regex> = invalid
        .regular_expressions
        .iter()
        .map(|f| Regex::new(f).unwrap_or_else(|_| panic!("failed parsing regex: {}", f)))
        .filter(|re| re.is_match(&patch.header.author))
        .collect();

    let test_result = match result.is_empty() {
        true => Ok(()),
        false => Err(PatchError::AuthorError { matches: result }),
    };

    LintResult {
        meta_info: TestMetaInfo {
            name: "valid author".to_owned(),
        },
        test_result,
    }
}

#[test]
fn test_author() {
    use std::path::Path;

    fn is_error_patch_file(patch_path: &str) {
        let invalid = InvalidAuthors {
            regular_expressions: vec!["example".to_owned()],
        };
        let patch = Patch::from_file(Path::new(patch_path));
        assert!(test_author_valid(&patch, &invalid).test_result.is_err());
    }

    fn is_ok_patch_file(patch_path: &str) {
        let invalid = InvalidAuthors {
            regular_expressions: vec!["example".to_owned()],
        };
        let patch = Patch::from_file(Path::new(patch_path));
        assert!(test_author_valid(&patch, &invalid).test_result.is_ok());
    }

    is_error_patch_file("tests/files/Author.test_author_valid.1.fail");
    is_ok_patch_file("tests/files/Author.test_author_valid.1.pass");
    is_error_patch_file("tests/files/Author.test_author_valid.2.fail");
    is_ok_patch_file("tests/files/Author.test_author_valid.2.pass");
}
