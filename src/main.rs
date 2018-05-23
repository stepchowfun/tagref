extern crate clap;
extern crate colored;
extern crate ignore;
extern crate regex;

mod citations;
mod count;
mod duplicates;
mod label;
mod walk;

use colored::Colorize;
use std::collections::HashMap;
use std::process;

// Welcome to Tagref! The fun starts here.
fn main() {
  // Set up the command-line interface.
  let matches = clap::App::new("Tagref")
    .version("0.0.3")
    .author("Stephan Boyer <stephan@stephanboyer.com>")
    .about(
      " \
      Tagref helps you refer to other locations in your codebase. It checks \
      the following:\n\n1. References actually point to tags. \n2. Tags are \
      distinct.\n\nThe syntax for tags is [tag:?label?] and the syntax for \
      references is [ref:?label?]. For more information, visit \
      https://github.com/stepchowfun/tagref. \
      ".replace("?", "").trim() // The '?'s are to avoid tag conflicts.
      )
    .arg(
      clap::Arg::with_name("path")
        .short("p")
        .long("path")
        .value_name("PATH")
        .help("Sets the path of the directory to scan")
        .takes_value(true)
      )
    .arg(
      clap::Arg::with_name("list-tags")
        .short("n")
        .long("list-tags")
        .help("Lists all the tags")
      )
    .arg(
      clap::Arg::with_name("list-references")
        .short("r")
        .long("list-references")
        .help("Lists all the references")
      )
    .get_matches();

  // Fetch the command-line arguments.
  let dir = matches.value_of("path").unwrap_or(".");
  let list_tags = matches.is_present("list-tags");
  let list_references = matches.is_present("list-references");
  let check_references = !list_tags && !list_references;

  // Parse all the tags into a HashMap. The values are vectors to allow for
  // duplicates. We will report duplicates later.
  let mut tags_map = HashMap::new();
  let _ = walk::walk(dir, |path, contents| {
    for tag in label::parse(label::LabelType::Tag, path, contents) {
      tags_map.entry(
        tag.label.clone()
      ).or_insert(
        Vec::new()
      ).push(
        tag.clone()
      );
    }
  });

  // Convert tags_map into a set and check for duplicates.
  match duplicates::check(&tags_map) {
    Ok(tags) => {
      // Handle the --list-tags flag if necessary.
      if list_tags {
        for (_, tag) in &tags {
          println!("{}", tag);
        }
      }

      // Parse all the references.
      let mut references = Vec::new();
      let files_scanned = walk::walk(dir, |path, contents| {
        // We know that the regex is well-formed, so we are justified in
        // unwrapping.
        references.extend(label::parse(label::LabelType::Ref, path, contents));
      });

      // Handle the --list-references flag if necessary.
      if list_references {
        for reference in &references {
          println!("{}", reference);
        }
      }

      // Check the references if necessary.
      if check_references {
        match citations::check(&tags, &references) {
          Some(error) => {
            eprintln!("{}", error.red());
            process::exit(1);
          },
          None => {
            println!(
              "{}",
              format!(
                "{} and {} validated in {}.",
                count::count(tags.len().into(), "tag"),
                count::count(references.len().into(), "reference"),
                count::count(files_scanned, "file")
              ).green()
            );
          },
        };
      }
    },
    Err(error) => {
      eprintln!("{}", error.red());
      process::exit(1);
    },
  };
}
