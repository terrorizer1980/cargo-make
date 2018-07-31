//! # legacy
//!
//! Support legacy features.
//!

use dirs;
use std::env;
use std::fs::{copy, create_dir_all, remove_file};
use std::path::PathBuf;

fn get_legacy_cargo_make_home() -> Option<PathBuf> {
    match dirs::home_dir() {
        Some(directory) => Some(directory.join(".cargo-make")),
        None => None,
    }
}

pub(crate) fn get_cargo_make_home() -> Option<PathBuf> {
    match env::var("CARGO_MAKE_HOME") {
        Ok(directory) => Some(PathBuf::from(directory)),
        _ => get_legacy_cargo_make_home(),
    }
}

pub(crate) fn migrate(target_directory: PathBuf, file: &str) -> bool {
    debug!(
        "Legacy cargo-make target_directory: {:#?} file: {:#?} ",
        &target_directory, &file
    );
    return match get_legacy_cargo_make_home() {
        Some(directory) => {
            let legacy_file = directory.join(file);

            if legacy_file.exists() {
                let exists = if target_directory.exists() {
                    true
                } else {
                    match create_dir_all(&target_directory) {
                        Ok(_) => true,
                        _ => false,
                    }
                };

                if exists {
                    let target_file = target_directory.join(file);
                    info!("Legacy cargo-make file: {:#?} exists, target directory: {:#?} exists, copy to: {:#?}", &legacy_file, &target_directory, &target_file);

                    match copy(&legacy_file, &target_file) {
                        Ok(_) => {
                            info!("Delete legacy cargo-make file: {:#?}", &legacy_file);
                            remove_file(&legacy_file).unwrap_or(());
                            true
                        }
                        Err(error) => {
                            info!(
                                "Error while copying legacy file: {:#?} to: {:#?}, error: {:#?}",
                                &legacy_file, &target_file, &error
                            );
                            false
                        }
                    }
                } else {
                    false
                }
            } else {
                true
            }
        }
        None => true,
    };
}
