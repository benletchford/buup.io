use std::process::Command;

fn main() {
    // Get the Git hash
    let output = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output();

    let git_hash = match output {
        Ok(o) if o.status.success() => String::from_utf8(o.stdout)
            .unwrap_or_else(|_| "unknown".to_string())
            .trim()
            .to_string(),
        _ => "unknown".to_string(), // Fallback if git command fails or not in a repo
    };

    // Set the environment variable for the crate
    println!("cargo:rustc-env=BUUP_WEB_GIT_HASH={}", git_hash);

    // Rerun build script if HEAD changes
    println!("cargo:rerun-if-changed=.git/HEAD");

    // Rerun build script if the ref HEAD points to changes (e.g., commit on master)
    // Try to read the ref HEAD points to
    if let Ok(ref_path) = std::fs::read_to_string(".git/HEAD") {
        if let Some(ref_target) = ref_path.split_whitespace().nth(1) {
            let full_ref_path = format!(".git/{}", ref_target);
            println!("cargo:rerun-if-changed={}", full_ref_path);
        }
    }
}
