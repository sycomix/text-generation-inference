use std::error::Error;
use vergen::vergen;
use vergen::EmitBuilder;

fn main() -> Result<(), Box<dyn Error>> {
    // Try to get the git sha from the local git repository
    if EmitBuilder::builder()
        .configure(vergen::OutputFns::all())
        .build()
        .unwrap()
        .fail_on_stderr()
        .git_sha(false)
        .emit()
        .is_err()
    {
        // Unable to get the git sha
        if let Ok(sha) = std::env::var("GIT_SHA") {
            // Set it from an env var
            println!("cargo:rustc-env=VERGEN_GIT_SHA={sha}");
        }
    }

    // Check for additional dependencies
    if let Ok(label) = std::env::var("DOCKER_LABEL") {
        // Set it from an env var
        println!("cargo:rustc-env=DOCKER_LABEL={label}");
    }

    Ok(())
}
