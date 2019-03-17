mod citations;
mod count;
mod duplicates;
mod label;
mod walk;

use colored::Colorize;
use std::collections::HashMap;
use std::process;

const PATH_OPTION: &str = "path";
const CHECK_COMMAND: &str = "check";
const LIST_TAGS_COMMAND: &str = "list-tags";
const LIST_REFS_COMMAND: &str = "list-refs";

// Welcome to Tagref! The fun starts here.
fn main() {
  // Set up the command-line interface.
  let matches = clap::App::new("Tagref")
    .version("0.0.7")
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
    .arg(
      clap::Arg::with_name(PATH_OPTION)
        .short("p")
        .long(PATH_OPTION)
        .value_name("PATH")
        .help("Sets the path of the directory to scan")
        .takes_value(true),
    )
    .subcommand(
      clap::SubCommand::with_name(CHECK_COMMAND)
        .about("Check all the tags and references (default)"),
    )
    .subcommand(
      clap::SubCommand::with_name(LIST_TAGS_COMMAND)
        .about("List all the tags"),
    )
    .subcommand(
      clap::SubCommand::with_name(LIST_REFS_COMMAND)
        .about("List all the references"),
    )
    .get_matches();

  // Fetch the command-line arguments.
  let path = matches.value_of(PATH_OPTION).unwrap_or(".");

  // Decide what to do based on the subcommand.
  match matches.subcommand_name() {
    Some(LIST_TAGS_COMMAND) => {
      let _ = walk::walk(path, |file_path, contents| {
        for tag in label::parse(label::Type::Tag, file_path, contents) {
          println!("{}", tag);
        }
      });
    }

    Some(LIST_REFS_COMMAND) => {
      let _ = walk::walk(path, |file_path, contents| {
        for r#ref in label::parse(label::Type::Ref, file_path, contents) {
          println!("{}", r#ref);
        }
      });
    }

    Some(CHECK_COMMAND) | None => {
      // Parse all the tags into a HashMap. The values are vectors to allow for
      // duplicates. We will report duplicates later.
      let mut tags_map = HashMap::new();
      let _ = walk::walk(path, |file_path, contents| {
        for tag in label::parse(label::Type::Tag, file_path, contents) {
          tags_map
            .entry(tag.label.clone())
            .or_insert_with(Vec::new)
            .push(tag.clone());
        }
      });

      // Convert tags_map into a set and check for duplicates.
      match duplicates::check(&tags_map) {
        Ok(tags) => {
          // Parse all the references.
          let mut refs = Vec::new();
          let files_scanned = walk::walk(path, |file_path, contents| {
            refs.extend(label::parse(label::Type::Ref, file_path, contents));
          });

          // Check the references.
          if let Some(error) = citations::check(&tags, &refs) {
            eprintln!("{}", error.red());
            process::exit(1);
          } else {
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
        }
        Err(error) => {
          eprintln!("{}", error.red());
          process::exit(1);
        }
      };
    }

    Some(&_) => panic!(),
  }
}
