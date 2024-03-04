use {crate::directive::Directive, std::fs::metadata};

// This function checks that directory references actually point to files. It returns a vector of
// error strings.
pub fn check(refs: &[Directive]) -> Vec<String> {
    let mut errors = Vec::<String>::new();

    for dir in refs {
        match metadata(&dir.label) {
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
