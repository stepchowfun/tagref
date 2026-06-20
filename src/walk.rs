use ignore::{
    WalkBuilder, WalkState,
    overrides::{Override, OverrideBuilder},
};
use std::{
    fs::File,
    path::Path,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
};

// This function visits each file in the given directory and calls the given callback with the path
// and the file. It skips files which cannot be read (e.g., due to lack of permissions). It also
// skips over symlinks. The number of files traversed is returned.
pub fn walk<T: 'static + Clone + Send + FnMut(&Path, File)>(
    path: &Path,
    current_dir: &Path,
    ignore_rules: &[String],
    callback: T,
) -> Result<usize, String> {
    // Keep track of the number of files traversed, and allow multiple threads to update it.
    let files_scanned = Arc::new(AtomicUsize::new(0));
    let overrides = build_overrides(current_dir, ignore_rules)?;

    // Traverse the filesystem in parallel.
    WalkBuilder::new(path)
        .current_dir(current_dir)
        .hidden(false)
        .require_git(false)
        .parents(false)
        .overrides(overrides)
        .build_parallel()
        .run(|| {
            // These clones will be moved into the closure below, and that closure will be sent
            // to a new thread.
            let mut callback = callback.clone();
            let files_scanned = files_scanned.clone();

            // This closure will be sent to a new thread.
            Box::new(move |result| {
                // Proceed if we have access to the path.
                if let Ok(dir_entry) = result {
                    // Here, `file_type()` should always return a `Some`. It could only return
                    // `None` if the file represents STDIN, and that isn't the case here.
                    if dir_entry.file_type().unwrap().is_file() {
                        // Try to open the file.
                        let possible_file = File::open(dir_entry.path());
                        if let Ok(file) = possible_file {
                            // Process the file and increment the counter.
                            callback(dir_entry.path(), file);
                            files_scanned.fetch_add(1, Ordering::SeqCst);
                        }
                    }
                }

                // Don't stop...believing!
                WalkState::Continue
            })
        });

    // Return the number of files traversed.
    Ok(files_scanned.load(Ordering::SeqCst))
}

// Builds high-precedence ignore overrides for the walker
fn build_overrides(current_dir: &Path, ignore_rules: &[String]) -> Result<Override, String> {
    let mut override_builder = OverrideBuilder::new(current_dir);
    override_builder
        .add("!.git/")
        .unwrap() // Safe by manual inspection
        .add("!.hg/")
        .unwrap(); // Safe by manual inspection
    for ignore_rule in ignore_rules {
        override_builder
            .add(&format!("!{ignore_rule}"))
            .map_err(|error| format!("Error parsing ignore rule `{ignore_rule}`: {error}"))?;
    }
    override_builder
        .build()
        .map_err(|error| format!("Error building ignore rules: {error}"))
}
