use std::{collections::HashMap, path::Path};

use git2::Diff;
use regex::Regex;

static COMMIT_HEADER: &str = r"From ((.|\n)*)\n\-{3}\n";

macro_rules! regex {
    ($re:expr) => {
        ::regex::bytes::Regex::new($re).unwrap()
    };
}

#[derive(Debug)]
pub struct Header {
    pub from: String,
    pub author: String,
    pub date: String,
    pub subject: String,
    pub summary: String,
    pub signatures: Vec<String>,
}

impl Header {
    fn from_string(header_string: &str) -> Self {
        let re = regex!(
            r"(?P<from>^From .+)\n(?P<author>From: .+)\n(?P<date>Date: .+)\n(?P<subject>Subject: .+)\n\n(?P<summary>.+\n\n)*(?P<signature>Signed-off-by: .+)\n"
        );
        let captures = re
            .captures(header_string.as_bytes())
            .expect("Failed parsing header");

        let fields = vec!["from", "author", "date", "subject", "summary", "signature"];
        let mut extractions: HashMap<&str, String> = HashMap::new();

        for name in fields.iter() {
            let value = match captures.name(name) {
                Some(field) => String::from_utf8(field.as_bytes().to_vec()).unwrap_or_default(),
                None => "".to_owned(),
            };

            extractions.insert(name, value);
        }
        Header {
            from: extractions["from"].clone(),
            author: extractions["author"].clone(),
            date: extractions["date"].clone(),
            subject: extractions["subject"].clone(),
            summary: extractions["summary"].clone(),
            signatures: vec![extractions["signature"].clone()],
        }
    }
}

pub struct Patch<'a> {
    pub header: Header,
    pub diff: Diff<'a>,
}

impl<'a> Patch<'a> {
    /// Load a patch from a file
    pub fn from_file(path: &Path) -> Patch<'a> {
        let buff = std::fs::read(path).unwrap();

        let diff = Diff::from_buffer(&buff).expect("failed loading diff");
        let re = Regex::new(COMMIT_HEADER).unwrap();
        let text = std::str::from_utf8(&buff).unwrap();

        let header_match = re.find(text).unwrap();
        let start = header_match.start();
        let end = header_match.end();
        let header = &text[start..end];
        let header = Header::from_string(header);
        Patch { header, diff }
    }
}
