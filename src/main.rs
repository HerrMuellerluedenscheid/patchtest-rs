use anyhow::{Context, Ok, Result};
use checks::{ApplyPatch, CheckRepo, LintPatch, PatchError, Summary};
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

fn main() -> Result<()> {
    let args = Args::from_args();
    let mut errors: Vec<Result<String, PatchError>> = vec![];

    let patch_path = Path::new(&args.arg_patch_path);
    let patch = Patch::from_file(patch_path);
    errors.push(Summary::check(&patch));

    let repo = git_fetch(&args).context(format!("Failed when trying to fetch {}", args.arg_url))?;
    let error = ApplyPatch::apply(repo, &patch);
    errors.push(error);
    report_terminal(errors);

    Ok(())
}
