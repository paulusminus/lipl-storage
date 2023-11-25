use std::{
    path::Path,
    process::{exit, Command},
};

const FEATURES: [&str; 3] = ["fs", "postgres", "redis"];
const EXECUTABLE: &str = "cargo";

/// Change to workspace root.
///
/// Assumed this xtask is located in `[WORKSPACE]/crates/xtask-build-man`.
fn cwd_to_workspace_root() -> std::io::Result<()> {
    let pkg_root = std::env!("CARGO_MANIFEST_DIR");
    let ws_root = Path::new(pkg_root).join("..").join("..");
    std::env::set_current_dir(ws_root)
}

fn main() {
    cwd_to_workspace_root().expect("Cannot change working directory to workspace root");
    for feature in FEATURES {
        let bin_name = String::from("lipl-storage-") + feature;
        let status = Command::new(EXECUTABLE)
            .args([
                "build",
                "--release",
                "--bin",
                bin_name.as_str(),
                "-p",
                "lipl-storage-server",
                "--features",
                feature,
            ])
            .status()
            .unwrap_or_else(|_| panic!("Failed to run build for feature {feature}"));

        if !status.success() {
            eprintln!(
                "Failed to build bin for feature {feature}: Status {}",
                status
            );
            if let Some(code) = status.code() {
                exit(code);
            }
        }
    }
}
