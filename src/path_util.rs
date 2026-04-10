use std::borrow::Cow;
use std::path::{Component, Path, PathBuf};

/// Resolves target path relative to current working directory, in
/// case it's relative to the source path.
///
/// The target path is considered to be relative to the source path if
/// it's explicitly prefixed with the special directory expressions
/// such as `./`, `../` etc. Otherwise it's already considered
/// relative to the current working directory.
///
/// Example: Suppose source = ./docs/guidelines.md
///
/// If target = ../src/main.rs, result = src/main.rs
/// If target = ./arch.md, result = docs/arch.md
/// If target = README.md, result = README.md (no change)
///
/// Works similarly for files as well as dir paths.
///
pub fn resolve_target_path(source: &Path, target: &Path) -> PathBuf {
    let resolved = if is_explicit_relative(target) {
        // Relative to source path (use its parent if it's a file)
        let base = source.parent().unwrap_or(source);
        Cow::Owned(base.join(target))
    } else {
        // Already relative to current working directory
        Cow::Borrowed(target)
    };
    normalize_path(&resolved)
}

fn is_explicit_relative(path: &Path) -> bool {
    matches!(
        path.components().next(),
        Some(Component::CurDir | Component::ParentDir)
    )
}

/// Normalizes a relative path by getting rid of special directory
/// expressions such as `./`, `../` etc.
///
/// NOTE: Purely syntactic normalization (doesn't check if the file
/// actually exists)
fn normalize_path(path: &Path) -> PathBuf {
    let mut result = PathBuf::new();
    for comp in path.components() {
        match comp {
            Component::CurDir => {}
            Component::ParentDir => {
                result.pop();
            }
            other => result.push(other.as_os_str()),
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};

    use crate::path_util::resolve_target_path;

    #[test]
    fn test_resolve_target_path() {
        let result = resolve_target_path(
            &Path::new("./docs/guidelines.md"),
            &Path::new("../src/main.rs"),
        );
        assert_eq!(PathBuf::from("src/main.rs"), result);

        let result =
            resolve_target_path(&Path::new("./docs/guidelines.md"), &Path::new("./arch.md"));
        assert_eq!(PathBuf::from("docs/arch.md"), result);

        let result = resolve_target_path(
            &Path::new("./docs/guidelines.md"),
            &Path::new("../changelogs"),
        );
        assert_eq!(PathBuf::from("changelogs"), result);

        let result =
            resolve_target_path(&Path::new("./docs/guidelines.md"), &Path::new("README.md"));
        assert_eq!(PathBuf::from("README.md"), result);
    }
}
