use anyhow::{Context, Ok, Result};
use checks::{ApplyPatch, CheckRepo, LintPatch, LintResult, Summary};

use config::{load_config, Config};
use git2::Repository;
use report::report_terminal;
use std::path::Path;
use structopt::StructOpt;
mod checks;
mod config;
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

    #[structopt(name = "config")]
    arg_config: Option<String>,
}

fn run_all_tests(patch: Patch, repo: &Repository) -> Vec<LintResult> {
    let mut results = vec![];
    results.push(Summary::check(&patch));
    results.push(ApplyPatch::apply(repo, &patch));
    results
}

fn main() -> Result<()> {
    let args = Args::from_args();
    let mut config = Config::default();
    let arg_config = &args.arg_config.clone().unwrap_or_default();
    if !arg_config.is_empty() {
        config = load_config(Path::new(&arg_config));
    }
    let repo = git_fetch(&args).context(format!("Failed when trying to fetch {}", args.arg_url))?;
    let patch = Patch::from_file(Path::new(&args.arg_patch_path));

    let results = run_all_tests(patch, &repo);

    report_terminal(results, &config);

    Ok(())
}
