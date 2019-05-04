use ignore::Walk;
use std::{fs::File, path::Path};

// This function visits each file in the given directory and calls the given
// callback with the path and the file. It skips files which cannot be read
// (e.g., due to lack of permissions). The number of files traversed is
// returned.
pub fn walk<T: FnMut(&Path, File) -> ()>(
  path: &Path,
  mut callback: T,
) -> usize {
  let mut files_scanned = 0;

  for result in Walk::new(path) {
    if let Ok(dir_entry) = result {
      // Here, `file_type()` should always return a `Some`. It could only
      // return `None` if the file represents STDIN, and that isn't the case
      // here.
      if dir_entry.file_type().unwrap().is_file() {
        let possible_file = File::open(dir_entry.path());
        if let Ok(file) = possible_file {
          callback(dir_entry.path(), file);
          files_scanned += 1;
        }
      }
    }
  }

  files_scanned
}
