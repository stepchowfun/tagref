use ignore::Walk;
use std::fs::File;
use std::io::prelude::Read;

// This function visits each file in the given directory and calls the given
// callback with the path and the contents of the file. The number of files
// scanned is returned.
pub fn walk<T: FnMut(&str, &str) -> ()>(path: &str, mut callback: T) -> usize {
  let mut files_scanned: usize = 0;

  for result in Walk::new(path) {
    if let Ok(dir_entry) = result {
      // Here, file_type() should always return a Some. It could only return
      // None if the file represents STDIN, and that isn't the case here.
      if dir_entry.file_type().unwrap().is_file() {
        let possible_file = File::open(dir_entry.path());
        if let Ok(mut file) = possible_file {
          let mut contents = String::new();
          if file.read_to_string(&mut contents).is_ok() {
            files_scanned += 1;
            callback(
              &dir_entry.path().to_string_lossy().into_owned(),
              &contents
            );
          }
        }
      }
    }
  }

  files_scanned
}
