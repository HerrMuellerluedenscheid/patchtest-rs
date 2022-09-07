use crate::{checks::icon, LintResult};

/// print error report to console
pub fn report_terminal(lint_results: &[LintResult]) {
    for lint_result in lint_results.iter() {
        let result = &lint_result.result;
        let icon = icon(&lint_result.level);
        match result {
            Ok(name) => println!("✅ {}", name),
            Err(error) => println!("{} {} ({:?})", icon, error, error),
        }
    }
}
