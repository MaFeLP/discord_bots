use std::process::Command;

fn main() {
    // Get the git hash of the current project.
    let git_hash = {
        match Command::new("git").args(&["rev-parse", "--short", "HEAD"]).output() {
            Ok(output) => {
                match String::from_utf8(output.stdout) {
                    Ok(s) => s,
                    Err(_) => String::new()
                }
            }
            Err(_) => String::new()
        }
    };
    println!("cargo:rustc-env=GIT_HASH={}", git_hash);
}