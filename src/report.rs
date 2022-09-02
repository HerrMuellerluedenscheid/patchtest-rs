use crate::checks::PatchError;

/// print error report to console
pub fn report_terminal(results: Vec<Result<String, PatchError>>) {
    for result in results.iter() {
        match result {
            Ok(name) => println!("✅ {}", name),
            Err(error) => println!("❌ {} ({:?})", error, error),
        }
    }
}
