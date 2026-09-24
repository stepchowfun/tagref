use std::path::{Component, Path, PathBuf};

// Resolves `target_path` relative to the project root if `target_path` begins with a root
// component (e.g., `/src/main.rs`). Otherwise, `target_path` is interpreted as being relative to
// `source_dir`.
pub fn resolve_target_path(project_root: &Path, source_dir: &Path, target_path: &Path) -> PathBuf {
    let mut components = target_path.components();
    if components.next() == Some(Component::RootDir) {
        project_root.join(components.as_path())
    } else {
        project_root.join(source_dir).join(target_path)
    }
}

#[cfg(test)]
mod tests {
    use crate::path_util::resolve_target_path;
    use std::path::{Path, PathBuf};

    // Thanks to [ref:tagref_check], the following file and directory references will be checked
    // as a form of integration test:
    //
    // - [file:/toast.yml]   // Relative to the project root
    // - [file:../toast.yml] // Relative to the directory containing this file
    // - [file:main.rs]      // Relative to the directory containing this file
    // - [file:./main.rs]    // Relative to the directory containing this file
    // - [dir:/src]          // Relative to the project root
    // - [dir:/]             // The project root
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
    fn resolve_target_path_relative() {
        assert_eq!(
            resolve_target_path(
                Path::new("project"),
                Path::new("docs"),
                Path::new("src/main.rs"),
            ),
            PathBuf::from("project/docs/src/main.rs"),
        );
    }

    #[test]
    fn resolve_target_path_root() {
        assert_eq!(
            resolve_target_path(
                Path::new("project"),
                Path::new("docs"),
                Path::new("/src/main.rs"),
            ),
            PathBuf::from("project/src/main.rs"),
        );
    }

    #[test]
    fn resolve_target_path_root_only() {
        assert_eq!(
            resolve_target_path(Path::new("project"), Path::new("docs"), Path::new("/")),
            PathBuf::from("project"),
        );
    }
}
