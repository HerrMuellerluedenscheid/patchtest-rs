use crate::{
    checks::{icon, LintResult},
    config::Config,
};

/// print error report to console
pub fn report_terminal(lint_results: Vec<LintResult>, config: &Config) {
    for lint_result in lint_results.iter() {
        match &lint_result.test_result {
            Ok(()) => println!("✅ {}", &lint_result.meta_info.name),
            Err(error) => {
                let icon = icon(config.get_error_level(&lint_result.meta_info.name));
                println!("{} {} ({:?})", icon, error, error)
            }
        }
    }
}
