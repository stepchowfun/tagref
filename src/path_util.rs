use std::path::{Component, Path, PathBuf};

// Resolves `target_path` relative to `source_dir` if `target_path` begins with a `.` or `..`
// component. Otherwise, `target_path` is interpreted as being relative to the project root.
pub fn resolve_target_path(project_root: &Path, source_dir: &Path, target_path: &Path) -> PathBuf {
    if matches!(
        target_path.components().next(),
        Some(Component::CurDir | Component::ParentDir),
    ) {
        project_root.join(source_dir).join(target_path)
    } else {
        project_root.join(target_path)
    }
}

#[cfg(test)]
mod tests {
    use crate::path_util::resolve_target_path;
    use std::path::{Path, PathBuf};

    // Thanks to [ref:tagref_check], the following file and directory references will be checked
    // as a form of integration test:
    //
    // - [file:toast.yml]    // Relative to the project root
    // - [file:../toast.yml] // Relative to the directory containing this file
    // - [file:./main.rs]    // Relative to the directory containing this file
    // - [dir:src]           // Relative to the project root
    // - [dir:../src]        // Relative to the directory containing this file
    // - [dir:.]             // The directory containing this file

    #[test]
    fn resolve_target_path_parent_dir() {
        assert_eq!(
            resolve_target_path(
                Path::new("project"),
                Path::new("docs"),
                Path::new("../src/main.rs"),
            ),
            PathBuf::from("project/docs/../src/main.rs"),
        );
    }

    #[test]
    fn resolve_target_path_current_dir() {
        assert_eq!(
            resolve_target_path(
                Path::new("project"),
                Path::new("docs"),
                Path::new("./src/main.rs"),
            ),
            PathBuf::from("project/docs/./src/main.rs"),
        );
    }

    #[test]
    fn resolve_target_path_non_relative() {
        assert_eq!(
            resolve_target_path(
                Path::new("project"),
                Path::new("docs"),
                Path::new("src/main.rs"),
            ),
            PathBuf::from("project/src/main.rs"),
        );
    }
}
