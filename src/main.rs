mod citations;
mod count;
mod duplicates;
mod label;
mod walk;

use atty::Stream;
use clap::{App, AppSettings, Arg, SubCommand};
use colored::Colorize;
use std::{collections::HashMap, io::BufReader, path::Path, process::exit};

// The program version
const VERSION: &str = "0.0.8";

// Command-line option and subcommand names
const CHECK_COMMAND: &str = "check";
const LIST_REFS_COMMAND: &str = "list-refs";
const LIST_TAGS_COMMAND: &str = "list-tags";
const LIST_UNUSED_COMMAND: &str = "list-unused";
const PATH_OPTION: &str = "path";

// Program entrypoint
fn entry() -> Result<(), String> {
  // Determine whether to print colored output.
  colored::control::set_override(atty::is(Stream::Stdout));

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
    .arg(
      Arg::with_name(PATH_OPTION)
        .short("p")
        .long(PATH_OPTION)
        .value_name("PATH")
        .help("Sets the path of the directory to scan")
        .takes_value(true),
    )
    .subcommand(
      SubCommand::with_name(CHECK_COMMAND)
        .about("Check all the tags and references (default)"),
    )
    .subcommand(
      SubCommand::with_name(LIST_TAGS_COMMAND).about("List all the tags"),
    )
    .subcommand(
      SubCommand::with_name(LIST_UNUSED_COMMAND)
        .about("List the unreferenced tags"),
    )
    .subcommand(
      SubCommand::with_name(LIST_REFS_COMMAND)
        .about("List all the references"),
    )
    .get_matches();

  // Fetch the command-line arguments.
  let path =
    Path::new(&matches.value_of(PATH_OPTION).unwrap_or(".")).to_owned();

  // Decide what to do based on the subcommand.
  match matches.subcommand_name() {
    Some(LIST_TAGS_COMMAND) => {
      // Parse and print all the tags.
      let _ = walk::walk(&path, |file_path, file| {
        for tag in
          label::parse(label::Type::Tag, file_path, BufReader::new(file))
        {
          println!("{}", tag);
        }
      });
    }

    Some(LIST_REFS_COMMAND) => {
      // Parse and print all the references.
      let _ = walk::walk(&path, |file_path, file| {
        for r#ref in
          label::parse(label::Type::Ref, file_path, BufReader::new(file))
        {
          println!("{}", r#ref);
        }
      });
    }

    Some(LIST_UNUSED_COMMAND) => {
      // Parse all the tags into a `HashMap`. The values are `Vec`s to allow
      // for duplicates.
      let mut tags_map = HashMap::new();
      let _ = walk::walk(&path, |file_path, file| {
        for tag in
          label::parse(label::Type::Tag, file_path, BufReader::new(file))
        {
          tags_map
            .entry(tag.label.clone())
            .or_insert_with(Vec::new)
            .push(tag.clone());
        }
      });

      // Remove all the referenced tags.
      let _ = walk::walk(&path, |file_path, file| {
        for r#ref in
          label::parse(label::Type::Ref, file_path, BufReader::new(file))
        {
          tags_map.remove(&r#ref.label);
        }
      });

      // Print the remaining tags.
      for tags in tags_map.values() {
        for tag in tags {
          println!("{}", tag);
        }
      }
    }

    Some(CHECK_COMMAND) | None => {
      // Parse all the tags into a `HashMap`. The values are `Vec`s to allow
      // for duplicates. We'll report duplicates later.
      let mut tags_map = HashMap::new();
      let _ = walk::walk(&path, |file_path, file| {
        for tag in
          label::parse(label::Type::Tag, file_path, BufReader::new(file))
        {
          tags_map
            .entry(tag.label.clone())
            .or_insert_with(Vec::new)
            .push(tag.clone());
        }
      });

      // Convert tags_map into a set and check for duplicates.
      let tags = duplicates::check(&tags_map)?;

      // Parse all the references.
      let mut refs = Vec::new();
      let files_scanned = walk::walk(&path, |file_path, file| {
        refs.extend(label::parse(
          label::Type::Ref,
          file_path,
          BufReader::new(file),
        ));
      });

      // Check the references.
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
