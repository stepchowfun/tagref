mod citations;
mod count;
mod duplicates;
mod label;
mod walk;

use atty::Stream;
use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use colored::Colorize;
use std::{
    collections::HashMap,
    io::BufReader,
    path::{Path, PathBuf},
    process::exit,
    sync::{Arc, Mutex},
};

#[macro_use]
extern crate lazy_static;

// The program version
const VERSION: &str = env!("CARGO_PKG_VERSION");

// Command-line option and subcommand names
const CHECK_COMMAND: &str = "check";
const LIST_REFS_COMMAND: &str = "list-refs";
const LIST_TAGS_COMMAND: &str = "list-tags";
const LIST_UNUSED_COMMAND: &str = "list-unused";
const PATH_OPTION: &str = "path";

// Parse the command-line optins.
fn settings<'a>() -> (ArgMatches<'a>, Vec<PathBuf>) {
    // Set up the command-line interface.
    let matches = App::new("Tagref")
        .version(VERSION)
        .version_short("v")
        .author("Stephan Boyer <stephan@stephanboyer.com>")
        .about(
            " \
             Tagref helps you refer to other locations in your codebase. It checks \
             the following:\n\n1. References actually point to tags. \n2. Tags are \
             distinct.\n\nThe syntax for tags is [tag:?label?] and the syntax for \
             references is [ref:?label?]. For more information, visit \
             https://github.com/stepchowfun/tagref. \
             "
            .replace("?", "") // The '?'s are to avoid tag conflicts.
            .trim(),
        )
        .setting(AppSettings::ColoredHelp)
        .setting(AppSettings::UnifiedHelpMessage)
        .setting(AppSettings::VersionlessSubcommands)
        .arg(
            Arg::with_name(PATH_OPTION)
                .short("p")
                .long(PATH_OPTION)
                .value_name("PATH")
                .help("Adds the path of a directory to scan")
                .takes_value(true)
                .number_of_values(1)
                .multiple(true),
        )
        .subcommand(
            SubCommand::with_name(CHECK_COMMAND)
                .about("Check all the tags and references (default)"),
        )
        .subcommand(SubCommand::with_name(LIST_TAGS_COMMAND).about("List all the tags"))
        .subcommand(SubCommand::with_name(LIST_UNUSED_COMMAND).about("List the unreferenced tags"))
        .subcommand(SubCommand::with_name(LIST_REFS_COMMAND).about("List all the references"))
        .get_matches();

    // Determine which paths to scan.
    let paths = matches
        .values_of(PATH_OPTION)
        .map_or_else(
            || vec![".".to_owned()],
            |paths| {
                paths
                    .map(std::borrow::ToOwned::to_owned)
                    .collect::<Vec<_>>()
            },
        )
        .iter()
        .map(|path| Path::new(path).to_owned())
        .collect::<Vec<_>>();

    // Return the command-line options.
    (matches, paths)
}

// Program entrypoint
#[allow(clippy::too_many_lines)]
fn entry() -> Result<(), String> {
    // Determine whether to print colored output.
    colored::control::set_override(atty::is(Stream::Stdout));

    // Parse the command-line options.
    let (matches, paths) = settings();

    // Decide what to do based on the subcommand.
    match matches.subcommand_name() {
        Some(LIST_TAGS_COMMAND) => {
            // Parse and print all the tags.
            let mutex = Arc::new(Mutex::new(()));
            let _ = walk::walk(&paths, move |file_path, file| {
                for tag in label::parse(label::Type::Tag, file_path, BufReader::new(file)) {
                    let _ = mutex.lock().unwrap(); // Safe assuming no poisoning
                    println!("{}", tag);
                }
            });
        }

        Some(LIST_REFS_COMMAND) => {
            // Parse and print all the references.
            let mutex = Arc::new(Mutex::new(()));
            let _ = walk::walk(&paths, move |file_path, file| {
                for r#ref in label::parse(label::Type::Ref, file_path, BufReader::new(file)) {
                    let _ = mutex.lock().unwrap(); // Safe assuming no poisoning
                    println!("{}", r#ref);
                }
            });
        }

        Some(LIST_UNUSED_COMMAND) => {
            // Store the tags into a `HashMap`. The values are `Vec`s to allow for duplicates.
            let tags_map = Arc::new(Mutex::new(HashMap::new()));

            // Parse all the tags.
            let tags_map_add_tags = tags_map.clone();
            let _ = walk::walk(&paths, move |file_path, file| {
                for tag in label::parse(label::Type::Tag, file_path, BufReader::new(file)) {
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
                for r#ref in label::parse(label::Type::Ref, file_path, BufReader::new(file)) {
                    tags_map_remove_tags
                        .lock()
                        .unwrap() // Safe assuming no poisoning
                        .remove(&r#ref.label);
                }
            });

            // Print the remaining tags. The `unwrap` is safe assuming no poisoning.
            for tags in tags_map.lock().unwrap().values() {
                for tag in tags {
                    println!("{}", tag);
                }
            }
        }

        Some(CHECK_COMMAND) | None => {
            // Parse all the tags into a `HashMap`. The values are `Vec`s to allow for duplicates.
            // We'll report duplicates later.
            let tags = Arc::new(Mutex::new(HashMap::new()));
            let tags_clone = tags.clone();
            let _ = walk::walk(&paths, move |file_path, file| {
                for tag in label::parse(label::Type::Tag, file_path, BufReader::new(file)) {
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
                let results = label::parse(label::Type::Ref, file_path, BufReader::new(file));
                if !results.is_empty() {
                    refs_clone
                        .lock()
                        .unwrap() // Safe assuming no poisoning
                        .extend(results);
                }
            });

            // Check the references. The `unwrap` is safe assuming no poisoning.
            let refs = refs.lock().unwrap();
            citations::check(&tags, &refs)?;
            println!(
                "{}",
                format!(
                    "{} and {} validated in {}.",
                    count::count(tags.len(), "tag"),
                    count::count(refs.len(), "reference"),
                    count::count(files_scanned, "file")
                )
                .green()
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
