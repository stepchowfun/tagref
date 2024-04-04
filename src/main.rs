mod count;
mod dir_references;
mod directive;
mod duplicates;
mod file_references;
mod tag_references;
mod walk;

use {
    atty::Stream,
    clap::{App, AppSettings, Arg, SubCommand},
    colored::Colorize,
    directive::compile_directive_regex,
    std::{
        collections::{HashMap, HashSet},
        io::BufReader,
        path::{Path, PathBuf},
        process::exit,
        sync::{Arc, Mutex},
    },
};

// The program version
const VERSION: &str = env!("CARGO_PKG_VERSION");

// Command-line option and subcommand names
const CHECK_SUBCOMMAND: &str = "check";
const LIST_TAGS_SUBCOMMAND: &str = "list-tags";
const LIST_REFS_SUBCOMMAND: &str = "list-refs";
const LIST_FILES_SUBCOMMAND: &str = "list-files";
const LIST_DIRS_SUBCOMMAND: &str = "list-dirs";
const LIST_UNUSED_SUBCOMMAND: &str = "list-unused";
const LIST_UNUSED_ERROR_OPTION: &str = "fail-if-any"; // [tag:fail_if_any]
const PATH_OPTION: &str = "path";
const TAG_SIGIL_OPTION: &str = "tag-sigil";
const REF_SIGIL_OPTION: &str = "ref-sigil";
const FILE_SIGIL_OPTION: &str = "file-sigil";
const DIR_SIGIL_OPTION: &str = "dir-sigil";

// This enum represents the subcommands.
enum Subcommand {
    Check,
    ListTags,
    ListRefs,
    ListFiles,
    ListDirs,
    ListUnused(bool), // [ref:fail_if_any]
}

// This struct represents the command-line arguments.
#[allow(clippy::struct_excessive_bools)]
pub struct Settings {
    paths: Vec<PathBuf>,
    tag_sigil: String,
    ref_sigil: String,
    file_sigil: String,
    dir_sigil: String,
    subcommand: Subcommand,
}

// Parse the command-line arguments.
#[allow(clippy::too_many_lines)]
fn settings() -> Settings {
    // Set up the command-line interface.
    let matches = App::new("Tagref")
        .version(VERSION)
        .version_short("v")
        .author("Stephan Boyer <stephan@stephanboyer.com>")
        .about(
            "\
             Tagref helps you maintain cross-references in your code.\n\
             \n\
             You can annotate your code with tags like [tag?:foo] and reference them like \
             [ref?:foo]. You can also reference files like [file:src/main.rs] and directories like \
             [dir:src].\n\
             \n\
             Tagref checks that tags are unique and that references are not dangling.\n\
             \n\
             For more information, visit https://github.com/stepchowfun/tagref.\
             "
            .replace('?', "")
            .trim(),
        )
        .setting(AppSettings::ColoredHelp)
        .setting(AppSettings::NextLineHelp)
        .setting(AppSettings::UnifiedHelpMessage)
        .setting(AppSettings::VersionlessSubcommands)
        .arg(
            Arg::with_name(PATH_OPTION)
                .value_name("PATH")
                .short("p")
                .long(PATH_OPTION)
                .help("Adds the path of a directory to scan")
                .default_value(".") // [tag:path_default]
                .multiple(true)
                .number_of_values(1),
        )
        .arg(
            Arg::with_name(TAG_SIGIL_OPTION)
                .value_name("TAG_SIGIL")
                .short("t")
                .long(TAG_SIGIL_OPTION)
                .help("Sets the sigil used for tags")
                .default_value("tag"), // [tag:tag_sigil_default]
        )
        .arg(
            Arg::with_name(REF_SIGIL_OPTION)
                .value_name("REF_SIGIL")
                .short("r")
                .long(REF_SIGIL_OPTION)
                .help("Sets the sigil used for tag references")
                .default_value("ref"), // [tag:ref_sigil_default]
        )
        .arg(
            Arg::with_name(FILE_SIGIL_OPTION)
                .value_name("FILE_SIGIL")
                .short("f")
                .long(FILE_SIGIL_OPTION)
                .help("Sets the sigil used for file references")
                .default_value("file"), // [tag:file_sigil_default]
        )
        .arg(
            Arg::with_name(DIR_SIGIL_OPTION)
                .value_name("DIR_SIGIL")
                .short("d")
                .long(DIR_SIGIL_OPTION)
                .help("Sets the sigil used for directory references")
                .default_value("dir"), // [tag:dir_sigil_default]
        )
        .subcommand(
            SubCommand::with_name(CHECK_SUBCOMMAND)
                .about("Checks all the tags and references (default)"),
        )
        .subcommand(SubCommand::with_name(LIST_TAGS_SUBCOMMAND).about("Lists all the tags"))
        .subcommand(
            SubCommand::with_name(LIST_REFS_SUBCOMMAND).about("Lists all the tag references"),
        )
        .subcommand(
            SubCommand::with_name(LIST_FILES_SUBCOMMAND).about("Lists all the file references"),
        )
        .subcommand(
            SubCommand::with_name(LIST_DIRS_SUBCOMMAND).about("Lists all the directory references"),
        )
        .subcommand(
            SubCommand::with_name(LIST_UNUSED_SUBCOMMAND)
                .about("Lists the unreferenced tags")
                .arg(
                    Arg::with_name(LIST_UNUSED_ERROR_OPTION)
                        .long(LIST_UNUSED_ERROR_OPTION)
                        .help("Exits with an error status code if any tags are unreferenced"),
                ),
        )
        .get_matches();

    // Determine which paths to scan. The `unwrap` is safe due to [ref:path_default].
    let paths = matches
        .values_of(PATH_OPTION)
        .unwrap()
        .map(|path| Path::new(path).to_owned())
        .collect::<Vec<_>>();

    // Determine the tag sigil. The `unwrap` is safe due to [ref:tag_sigil_default].
    let tag_sigil = matches.value_of(TAG_SIGIL_OPTION).unwrap().to_owned();

    // Determine the ref sigil. The `unwrap` is safe due to [ref:ref_sigil_default].
    let ref_sigil = matches.value_of(REF_SIGIL_OPTION).unwrap().to_owned();

    // Determine the file sigil. The `unwrap` is safe due to [ref:file_sigil_default].
    let file_sigil = matches.value_of(FILE_SIGIL_OPTION).unwrap().to_owned();

    // Determine the directory sigil. The `unwrap` is safe due to [ref:dir_sigil_default].
    let dir_sigil = matches.value_of(DIR_SIGIL_OPTION).unwrap().to_owned();

    // Determine the subcommand.
    let subcommand = match matches.subcommand_name() {
        Some(CHECK_SUBCOMMAND) | None => Subcommand::Check,
        Some(LIST_TAGS_SUBCOMMAND) => Subcommand::ListTags,
        Some(LIST_REFS_SUBCOMMAND) => Subcommand::ListRefs,
        Some(LIST_FILES_SUBCOMMAND) => Subcommand::ListFiles,
        Some(LIST_DIRS_SUBCOMMAND) => Subcommand::ListDirs,
        Some(LIST_UNUSED_SUBCOMMAND) => Subcommand::ListUnused(
            matches
                .subcommand
                .unwrap() // Safe because we're _in_ a subcommand
                .matches
                .is_present(LIST_UNUSED_ERROR_OPTION),
        ),
        Some(&_) => panic!("Unimplemented subcommand."),
    };

    // Return the command-line options.
    Settings {
        paths,
        tag_sigil,
        ref_sigil,
        file_sigil,
        dir_sigil,
        subcommand,
    }
}

// Program entrypoint
#[allow(clippy::too_many_lines)]
fn entry() -> Result<(), String> {
    // Determine whether to print colored output.
    colored::control::set_override(atty::is(Stream::Stdout));

    // Parse the command-line options.
    let settings = settings();

    // Compile the regular expressions in advance.
    let tag_regex = compile_directive_regex(&settings.tag_sigil);
    let ref_regex = compile_directive_regex(&settings.ref_sigil);
    let file_regex = compile_directive_regex(&settings.file_sigil);
    let dir_regex = compile_directive_regex(&settings.dir_sigil);

    // Parse all the tags and references.
    let tags = Arc::new(Mutex::new(HashMap::new()));
    let refs = Arc::new(Mutex::new(Vec::new()));
    let files = Arc::new(Mutex::new(Vec::new()));
    let dirs = Arc::new(Mutex::new(Vec::new()));
    let tags_clone = tags.clone();
    let refs_clone = refs.clone();
    let files_clone = files.clone();
    let dirs_clone = dirs.clone();
    let tag_regex_clone = tag_regex.clone();
    let ref_regex_clone = ref_regex.clone();
    let file_regex_clone = file_regex.clone();
    let dir_regex_clone = dir_regex.clone();
    let files_scanned = walk::walk(&settings.paths, move |file_path, file| {
        let directives = directive::parse(
            &tag_regex_clone,
            &ref_regex_clone,
            &file_regex_clone,
            &dir_regex_clone,
            file_path,
            BufReader::new(file),
        );
        for tag in directives.tags {
            tags_clone
                .lock()
                .unwrap() // Safe assuming no poisoning
                .entry(tag.label.clone())
                .or_insert_with(Vec::new)
                .push(tag.clone());
        }
        refs_clone.lock().unwrap().extend(directives.refs); // Safe assuming no poisoning
        files_clone.lock().unwrap().extend(directives.files); // Safe assuming no poisoning
        dirs_clone.lock().unwrap().extend(directives.dirs); // Safe assuming no poisoning
    });

    // Decide what to do based on the subcommand.
    match settings.subcommand {
        Subcommand::Check => {
            // Errors will be accumulated in this vector.
            let mut errors = Vec::<String>::new();

            // Convert the `tags` map into a set and check for duplicates. The `unwrap` is safe
            // assuming no poisoning.
            errors.extend(duplicates::check(&tags.lock().unwrap()));

            // Check the tag references. The `unwrap`s are safe assuming no poisoning.
            let tags = tags
                .lock()
                .unwrap()
                .keys()
                .cloned()
                .collect::<HashSet<String>>();
            let refs = refs.lock().unwrap();
            errors.extend(tag_references::check(&tags, &refs));

            // Check the file references. The `unwrap` is safe assuming no poisoning.
            errors.extend(file_references::check(&files.lock().unwrap()));

            // Check the directory references. The `unwrap` is safe assuming no poisoning.
            errors.extend(dir_references::check(&dirs.lock().unwrap()));

            // Check for any errors and report the result.
            if errors.is_empty() {
                println!(
                    "{}",
                    format!(
                        "{}, {}, {}, and {} validated in {}.",
                        count::count(tags.len(), "tag"),
                        count::count(refs.len(), "tag reference"),
                        // The `unwrap` is safe assuming no poisoning.
                        count::count(files.lock().unwrap().len(), "file reference"),
                        // The `unwrap` is safe assuming no poisoning.
                        count::count(dirs.lock().unwrap().len(), "directory reference"),
                        count::count(files_scanned, "file"),
                    )
                    .green(),
                );
            } else {
                return Err(errors.join("\n\n"));
            }
        }

        Subcommand::ListTags => {
            // Print all the tags. The `unwrap` is safe assuming no poisoning.
            for dupes in tags.lock().unwrap().values() {
                for dupe in dupes {
                    println!("{dupe}");
                }
            }
        }

        Subcommand::ListRefs => {
            // Print all the tag references. The `unwrap` is safe assuming no poisoning.
            for r#ref in refs.lock().unwrap().iter() {
                println!("{ref}");
            }
        }

        Subcommand::ListFiles => {
            // Print all the file references. The `unwrap` is safe assuming no poisoning.
            for file in files.lock().unwrap().iter() {
                println!("{file}");
            }
        }

        Subcommand::ListDirs => {
            // Print all the directory references. The `unwrap` is safe assuming no poisoning.
            for dir in dirs.lock().unwrap().iter() {
                println!("{dir}");
            }
        }

        Subcommand::ListUnused(error_flag_set) => {
            // Remove all the referenced tags. The `unwrap` is safe assuming no poisoning.
            for r#ref in refs.lock().unwrap().iter() {
                tags.lock()
                    .unwrap() // Safe assuming no poisoning
                    .remove(&r#ref.label);
            }

            // Print the remaining tags. The `unwrap` is safe assuming no poisoning.
            for dupes in tags.lock().unwrap().values() {
                for dupe in dupes {
                    println!("{dupe}");
                }
            }

            // Error out if the error flag has been passed and there are unused tags.
            // The `unwrap` is safe assuming no poisoning.
            if error_flag_set && !tags.lock().unwrap().is_empty() {
                return Err(format!(
                    "Found unused tags while using --{LIST_UNUSED_ERROR_OPTION}",
                ));
            }
        }
    }

    // Everything succeeded.
    Ok(())
}

// Let the fun begin!
fn main() {
    // Jump to the entrypoint and handle any resulting errors.
    if let Err(e) = entry() {
        eprintln!("{}", e.red());
        exit(1);
    }
}
