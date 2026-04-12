use crate::{directive::Directive, path_util::resolve_target_path};
use std::{fs::metadata, path::Path};

// This function checks that file references actually point to files. It returns a vector of error
// strings.
pub fn check(refs: &[Directive]) -> Vec<String> {
    let mut errors = Vec::<String>::new();

    for file in refs {
        // The `unwrap` is safe because `file.path` should always exist in some parent directory.
        match metadata(resolve_target_path(
            file.path.parent().unwrap(),
            Path::new(&file.label),
        )) {
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
