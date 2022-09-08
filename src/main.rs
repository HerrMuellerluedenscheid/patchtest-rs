use crate::checks::run_all_tests;
use anyhow::{Context, Ok, Result};
use config::{load_config, print_config, Config};
use report::report_terminal;
use std::{path::Path, process::exit};
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

    #[structopt(long)]
    config: Option<String>,

    #[structopt(long)]
    print_config: bool,
}

fn main() -> Result<()> {
    let args = Args::from_args();

    if args.print_config {
        print_config();
        exit(0);
    };

    let mut config = Config::default();

    let arg_config = &args.config.clone().unwrap_or_default();
    if !arg_config.is_empty() {
        config = load_config(Path::new(&arg_config));
    }
    let repo = git_fetch(&args).context(format!("Failed when trying to fetch {}", args.arg_url))?;
    let patch = Patch::from_file(Path::new(&args.arg_patch_path));

    let results = run_all_tests(patch, &repo, &config);

    report_terminal(results, &config);

    Ok(())
}
