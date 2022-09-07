use anyhow::{Context, Ok, Result};
use checks::{print_config, ApplyPatch, CheckRepo, Level, LintPatch, PatchError, Summary};
use report::report_terminal;
use std::path::Path;
use structopt::StructOpt;
mod checks;
mod fetch;
mod patch;
mod report;
use fetch::git_fetch;
use patch::Patch;

#[derive(StructOpt)]
pub struct Args {
    #[structopt(name = "url")]
    arg_url: String,
    #[structopt(name = "path")]
    arg_path: String,
    #[structopt(name = "patch")]
    arg_patch_path: String,
}

pub struct LintResult {
    result: Result<String, PatchError>,
    level: Level,
}

fn main() -> Result<()> {
    let args = Args::from_args();
    let mut results = vec![];

    let patch_path = Path::new(&args.arg_patch_path);
    let patch = Patch::from_file(patch_path);
    let summary = Summary {
        level: Level::Warning,
    };
    let result = LintResult {
        result: Summary::check(&patch),
        level: summary.level,
    };
    results.push(result);

    let repo = git_fetch(&args).context(format!("Failed when trying to fetch {}", args.arg_url))?;
    let result = LintResult {
        result: ApplyPatch::apply(&repo, &patch),
        level: Level::Warning,
    };
    results.push(result);
    report_terminal(&results);

    print_config();
    Ok(())
}
