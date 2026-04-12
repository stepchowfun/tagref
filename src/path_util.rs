use std::path::{Component, Path, PathBuf};

// Resolves `target_path` relative to `source_dir` if `target_path` begins with a `.` or `..`
// component. Otherwise, `target_path` is interpreted as being relative to the current working
// directory and returned as is.
pub fn resolve_target_path(source_dir: &Path, target_path: &Path) -> PathBuf {
    if matches!(
        target_path.components().next(),
        Some(Component::CurDir | Component::ParentDir),
    ) {
        source_dir.join(target_path)
    } else {
        target_path.to_owned()
    }
}

#[cfg(test)]
mod tests {
    use crate::path_util::resolve_target_path;
    use std::path::{Path, PathBuf};

    // Thanks to [ref:tagref_check], the following file and direcotry references will be checked
    // as a form of integration test:
    //
    // - [file:toast.yml]    // Relative to current working directory (project root)
    // - [file:../toast.yml] // A file in the parent directory
    // - [file:./main.rs]    // A sibling file
    // - [dir:src]           // Relative to current working directory (project root)
    // - [dir:../src]        // A directory in the parent directory
    // - [dir:.]             // The directory containing this file

    #[test]
    fn resolve_target_path_parent_dir() {
        assert_eq!(
            resolve_target_path(Path::new("docs/guidelines.md"), Path::new("../src/main.rs")),
            PathBuf::from("docs/guidelines.md/../src/main.rs"),
        );
    }

    #[test]
    fn resolve_target_path_current_dir() {
        assert_eq!(
            resolve_target_path(Path::new("docs/guidelines.md"), Path::new("./src/main.rs")),
            PathBuf::from("docs/guidelines.md/./src/main.rs"),
        );
    }

    #[test]
    fn resolve_target_path_non_relative() {
        assert_eq!(
            resolve_target_path(Path::new("docs/guidelines.md"), Path::new("src/main.rs")),
            PathBuf::from("src/main.rs"),
        );
    }
}
