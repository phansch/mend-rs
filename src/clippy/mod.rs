use std::path::Path;
use std::process::Command;

/// Run clippy in the specified path
///
/// Returns the stderr of the clippy command which includes the JSON we care about.
pub fn run(path: &Path) -> String {
    let output = Command::new("cargo")
        .env("RUSTFLAGS", "-Z unstable-options --error-format=json")
        .arg("+nightly")
        .arg("clippy")
        .current_dir(path)
        .output()
        .expect(&format!("Running clippy failed in path: {:?}", path));

    String::from_utf8(output.stderr).expect("Could not read stderr")
}
