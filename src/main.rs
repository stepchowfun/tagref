#![deny(clippy::all, clippy::pedantic, warnings)]

mod count;
mod duplicates;
mod label;
mod references;
mod walk;

use {
    atty::Stream,
    clap::{App, AppSettings, Arg, ArgMatches, SubCommand},
    colonnade::Colonnade,
    colored::Colorize,
    regex::{escape, Regex},
    std::{
        cmp::{max, min},
        collections::HashMap,
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
const LIST_REFS_SUBCOMMAND: &str = "list-refs";
const LIST_TAGS_SUBCOMMAND: &str = "list-tags";
const LIST_UNUSED_SUBCOMMAND: &str = "list-unused";
const PATH_OPTION: &str = "path";
const TAG_PREFIX_OPTION: &str = "tag-prefix";
const REF_PREFIX_OPTION: &str = "ref-prefix";

// Parse the command-line optins.
fn settings<'a>() -> (ArgMatches<'a>, Vec<PathBuf>, String, String) {
    // Set up the command-line interface.
    let matches = App::new("Tagref")
        .version(VERSION)
        .version_short("v")
        .author("Stephan Boyer <stephan@stephanboyer.com>")
        .about(
            " \
             Tagref helps you maintain cross-references in your code. It checks \
             the following:\n\n1. References actually point to tags. \n2. Tags are \
             unique.\n\nThe syntax for tags is [tag:?label?] and the syntax for \
             references is [ref:?label?]. For more information, visit \
             https://github.com/stepchowfun/tagref. \
             "
            .replace('?', "") // The '?'s are to avoid tag conflicts.
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
                .multiple(true),
        )
        .arg(
            Arg::with_name(TAG_PREFIX_OPTION)
                .value_name("TAG_PREFIX")
                .short("t")
                .long(TAG_PREFIX_OPTION)
                .help("Sets the prefix used for locating tags")
                .default_value("tag"), // [tag:tag_prefix_default]
        )
        .arg(
            Arg::with_name(REF_PREFIX_OPTION)
                .value_name("REF_PREFIX")
                .short("r")
                .long(REF_PREFIX_OPTION)
                .help("Sets the prefix used for locating references")
                .default_value("ref"), // [tag:ref_prefix_default]
        )
        .subcommand(
            SubCommand::with_name(CHECK_SUBCOMMAND)
                .about("Checks all the tags and references (default)"),
        )
        .subcommand(SubCommand::with_name(LIST_TAGS_SUBCOMMAND).about("Lists all the tags"))
        .subcommand(
            SubCommand::with_name(LIST_UNUSED_SUBCOMMAND).about("Lists the unreferenced tags"),
        )
        .subcommand(SubCommand::with_name(LIST_REFS_SUBCOMMAND).about("Lists all the references"))
        .get_matches();

    // Determine which paths to scan. The `unwrap` is safe due to [ref:path_default].
    let paths = matches
        .values_of(PATH_OPTION)
        .unwrap()
        .map(|path| Path::new(path).to_owned())
        .collect::<Vec<_>>();

    // Determine the ref prefix. The `unwrap` is safe due to [ref:ref_prefix_default].
    let ref_prefix = matches.value_of(REF_PREFIX_OPTION).unwrap().to_owned();

    // Determine the tag prefix. The `unwrap` is safe due to [ref:tag_prefix_default].
    let tag_prefix = matches.value_of(TAG_PREFIX_OPTION).unwrap().to_owned();

    // Return the command-line options.
    (matches, paths, tag_prefix, ref_prefix)
}

// Print data in two tabulated columns.
fn print_tabulated(data: Vec<[String; 3]>) {
    // Set the viewport width to the terminal width if STDOUT is a TTY, or 80 otherwise. We ensure
    // the width is at least 3 to ensure there is enough room for 3 columns with a 1-space margin
    // between them [tag:min_terminal_width].
    let terminal_width = max(
        term_size::dimensions_stdout().map_or(80, |(width, _height)| width),
        4,
    );

    // Create a 3-column table. The `unwrap` is safe due to [ref:min_terminal_width].
    let mut colonnade = Colonnade::new(3, terminal_width).unwrap();

    colonnade.columns[1]
        .max_width(min(
            data.iter().map(|arr| arr[1].len()).max().unwrap_or(0),
            45,
        ))
        .unwrap();

    // The `unwrap` is safe by manual inspection of the types of errors that can be thrown.
    for line in colonnade.tabulate(data).unwrap() {
        println!("{line}");
    }
}

// Program entrypoint
#[allow(clippy::too_many_lines)]
fn entry() -> Result<(), String> {
    // Determine whether to print colored output.
    colored::control::set_override(atty::is(Stream::Stdout));

    // Parse the command-line options.
    let (matches, paths, tag_prefix, ref_prefix) = settings();

    // Compile the regular expressions in advance.
    let tag_regex: Regex = Regex::new(&format!(
        "(?i)\\[\\s*{}\\s*:\\s*([^\\]\\s]*)\\s*\\]\\s*[.,:]*\\s*(.+$)?",
        escape(&tag_prefix),
    ))
    .unwrap();
    let ref_regex: Regex = Regex::new(&format!(
        "(?i)\\[\\s*{}\\s*:\\s*([^\\]\\s]*)\\s*\\]\\s*[.,:]*\\s*(.+$)?",
        escape(&ref_prefix),
    ))
    .unwrap();

    // Decide what to do based on the subcommand.
    match matches.subcommand_name() {
        Some(LIST_TAGS_SUBCOMMAND) => {
            // Parse and print all the tags.
            let tags = Arc::new(Mutex::new(vec![]));
            let tags_clone = tags.clone();
            let _ = walk::walk(&paths, move |file_path, file| {
                let tags = tags_clone.clone();
                let mut results = vec![];
                for tag in label::parse(
                    &tag_regex,
                    &ref_regex,
                    label::Type::Tag,
                    file_path,
                    BufReader::new(file),
                ) {
                    results.push([
                        tag.label,
                        tag.description,
                        format!("{}:{}", tag.path.display(), tag.line_number),
                    ]);
                }
                tags.lock().unwrap().extend(results.drain(..)); // Safe assuming no poisoning
            });

            // Safe assuming no poisoning
            let mut tags: Vec<[String; 3]> = std::mem::take(&mut tags.lock().unwrap());

            tags.sort();
            print_tabulated(tags);
        }

        Some(LIST_REFS_SUBCOMMAND) => {
            // Parse and print all the references.
            let references = Arc::new(Mutex::new(vec![]));
            let references_clone = references.clone();
            let _ = walk::walk(&paths, move |file_path, file| {
                let references = references_clone.clone();
                let mut results = vec![];
                for r#ref in label::parse(
                    &tag_regex,
                    &ref_regex,
                    label::Type::Ref,
                    file_path,
                    BufReader::new(file),
                ) {
                    results.push([
                        r#ref.label,
                        r#ref.description,
                        format!("{}:{}", r#ref.path.display(), r#ref.line_number),
                    ]);
                }
                references.lock().unwrap().extend(results.drain(..)); // Safe assuming no poisoning
            });

            // Safe assuming no poisoning
            let mut references: Vec<[String; 3]> = std::mem::take(&mut references.lock().unwrap());

            references.sort();
            print_tabulated(references);
        }

        Some(LIST_UNUSED_SUBCOMMAND) => {
            // Store the tags into a `HashMap`. The values are `Vec`s to allow for duplicates.
            let tags_map = Arc::new(Mutex::new(HashMap::new()));

            // Parse all the tags.
            let tags_map_add_tags = tags_map.clone();
            let tag_regex_clone = tag_regex.clone();
            let ref_regex_clone = tag_regex.clone();
            let _ = walk::walk(&paths, move |file_path, file| {
                for tag in label::parse(
                    &tag_regex_clone,
                    &ref_regex_clone,
                    label::Type::Tag,
                    file_path,
                    BufReader::new(file),
                ) {
                    tags_map_add_tags
                        .lock()
                        .unwrap() // Safe assuming no poisoning
                        .entry(tag.label.clone())
                        .or_insert_with(Vec::new)
                        .push(tag.clone());
                }
            });

            // Remove all the referenced tags.
            let tags_map_remove_tags = tags_map.clone();
            let _ = walk::walk(&paths, move |file_path, file| {
                for r#ref in label::parse(
                    &tag_regex,
                    &ref_regex,
                    label::Type::Ref,
                    file_path,
                    BufReader::new(file),
                ) {
                    tags_map_remove_tags
                        .lock()
                        .unwrap() // Safe assuming no poisoning
                        .remove(&r#ref.label);
                }
            });

            // Print the remaining tags. The `unwrap` is safe assuming no poisoning.
            let mut references: Vec<[String; 3]> = tags_map
                .lock()
                .unwrap()
                .values()
                .flat_map(|tags| {
                    tags.iter().map(|tag| {
                        [
                            tag.label.clone(),
                            tag.description.clone(),
                            format!("{}:{}", tag.path.display(), tag.line_number),
                        ]
                    })
                })
                .collect();
            references.sort();
            print_tabulated(references);
        }

        Some(CHECK_SUBCOMMAND) | None => {
            // Parse all the tags into a `HashMap`. The values are `Vec`s to allow for duplicates.
            // We'll report duplicates later.
            let tags = Arc::new(Mutex::new(HashMap::new()));
            let tags_clone = tags.clone();
            let tag_regex_clone = tag_regex.clone();
            let ref_regex_clone = tag_regex.clone();
            let _ = walk::walk(&paths, move |file_path, file| {
                for tag in label::parse(
                    &tag_regex_clone,
                    &ref_regex_clone,
                    label::Type::Tag,
                    file_path,
                    BufReader::new(file),
                ) {
                    tags_clone
                        .lock()
                        .unwrap() // Safe assuming no poisoning
                        .entry(tag.label.clone())
                        .or_insert_with(Vec::new)
                        .push(tag.clone());
                }
            });

            // Convert tags_map into a set and check for duplicates. The `unwrap` is safe assuming
            // no poisoning.
            let tags = duplicates::check(&tags.lock().unwrap())?;

            // Parse all the references.
            let refs = Arc::new(Mutex::new(Vec::new()));
            let refs_clone = refs.clone();
            let files_scanned = walk::walk(&paths, move |file_path, file| {
                let results = label::parse(
                    &tag_regex,
                    &ref_regex,
                    label::Type::Ref,
                    file_path,
                    BufReader::new(file),
                );
                if !results.is_empty() {
                    refs_clone
                        .lock()
                        .unwrap() // Safe assuming no poisoning
                        .extend(results);
                }
            });

            // Check the references. The `unwrap` is safe assuming no poisoning.
            let refs = refs.lock().unwrap();
            references::check(&tags, &refs)?;
            println!(
                "{}",
                format!(
                    "{} and {} validated in {}.",
                    count::count(tags.len(), "tag"),
                    count::count(refs.len(), "reference"),
                    count::count(files_scanned, "file"),
                )
                .green(),
            );
        }

        Some(&_) => panic!(),
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
