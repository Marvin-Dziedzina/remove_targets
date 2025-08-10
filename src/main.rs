use std::{
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use clap::{Arg, command, value_parser};
use log::{debug, error, info, warn};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

const CARGO_TOML_NAME: &str = "Cargo.toml";

fn main() {
    env_logger::init();

    let matches = command!()
        .arg(
            Arg::new("path")
                .value_parser(value_parser!(PathBuf))
                .default_value("."),
        )
        .get_matches();

    let path = matches
        .get_one::<PathBuf>("path")
        .expect("Failed to get path")
        .to_path_buf();

    debug!("Search Root Path: {}", path.display());

    // Check if `path` exists.
    if !fs::exists(&path).expect("Failed to check if path exists") {
        panic!("Path does not exist");
    };

    hunt_target_dirs(&path);
}

fn hunt_target_dirs(start: &Path) {
    // Check if a `Cargo.toml` is present in the current dir. We can skip `cargo clean` if its no rust crate.
    let cargo_toml_exists = match fs::exists(start.join(CARGO_TOML_NAME)) {
        Ok(result) => result,
        Err(e) => {
            warn!(
                "Failed to check for `Cargo.toml` in `{}`: {}",
                start.display(),
                e
            );

            false
        }
    };

    debug!("`Cargo.toml` exists: {}", cargo_toml_exists);

    if cargo_toml_exists {
        // Run `cargo clean`
        match Command::new("cargo")
            .arg("clean")
            .current_dir(start)
            .status()
        {
            Ok(status) => {
                if status.success() {
                    info!("Cleaned {}", start.display());
                } else {
                    warn!(
                        "Cargo failed to clean `{}`: {:?}",
                        start.display(),
                        status.code()
                    );
                }
            }
            Err(e) => {
                warn!(
                    "Failed to run `cargo clean` at `{}`: {}",
                    start.display(),
                    e
                );
            }
        };
    }

    // Search all dirs in `start` for directories and search them.
    let dirs: Vec<_> = match fs::read_dir(start) {
        Ok(dirs) => dirs.collect(),
        Err(e) => {
            error!("Failed to read directories in `{}`: {}", start.display(), e);
            return;
        }
    };

    dirs.par_iter().for_each(|dir| {
        let dir = match dir {
            Ok(dir) => dir,
            Err(e) => {
                error!("Failed to get directory: {}", e);
                return;
            }
        };

        debug!("Found {}", dir.file_name().display());

        match dir.file_type() {
            Ok(file_type) => {
                if file_type.is_dir() && dir.file_name() != OsStr::new("target") {
                    debug!("Searching {}", dir.file_name().display());

                    hunt_target_dirs(&dir.path());

                    debug!("Done searching {}", dir.file_name().display());
                };
            }
            Err(e) => {
                error!("Failed to get file type: {}", e);
                return;
            }
        };
    });
}
