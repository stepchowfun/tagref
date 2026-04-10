use crate::{directive::Directive, path_util::resolve_target_path};
use std::{fs::metadata, path::Path};

// This function checks that file references actually point to files. It returns a vector of error
// strings.
pub fn check(refs: &[Directive]) -> Vec<String> {
    let mut errors = Vec::<String>::new();

    for file in refs {
        let file_path = resolve_target_path(&file.path, Path::new(&file.label));
        match metadata(&file_path) {
            Ok(metadata) => {
                if !metadata.is_file() {
                    errors.push(format!("{file} does not point to a file."));
                }
            }
            Err(error) => {
                let error_string = error.to_string();
                errors.push(format!("Error when validating {file}: {error_string}"));
            }
        }
    }

    errors
}
