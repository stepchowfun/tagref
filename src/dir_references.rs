use crate::{directive::Directive, path_util::resolve_target_path};
use std::{fs::metadata, path::Path};

// This function checks that directory references actually point to directories. It returns a vector
// of error strings.
pub fn check(refs: &[Directive]) -> Vec<String> {
    let mut errors = Vec::<String>::new();

    for dir in refs {
        let dir_path = resolve_target_path(&dir.path, Path::new(&dir.label));
        match metadata(&dir_path) {
            Ok(metadata) => {
                if !metadata.is_dir() {
                    errors.push(format!("{dir} does not point to a directory."));
                }
            }
            Err(error) => {
                let error_string = error.to_string();
                errors.push(format!("Error when validating {dir}: {error_string}"));
            }
        }
    }

    errors
}
