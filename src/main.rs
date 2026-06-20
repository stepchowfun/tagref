mod config;
mod count;
mod dir_references;
mod directive;
mod duplicates;
mod file_references;
mod path_util;
mod tag_references;
mod walk;

use clap::{ArgAction, Args, Parser, Subcommand as ClapSubcommand};
use colored::Colorize;
use directive::compile_directive_regex;
use std::{
    collections::{HashMap, HashSet},
    io::{self, BufReader, IsTerminal},
    path::PathBuf,
    process::exit,
    sync::{Arc, Mutex},
};

// This struct represents the command-line arguments.
#[derive(Parser)]
#[command(
    about = concat!(
        env!("CARGO_PKG_DESCRIPTION"),
        "\n\n",
        "You can annotate your code with tags like [tag:foo] and reference them like [ref:foo]. ",
        "You can also reference files like [file:src/main.rs] and directories like [dir:src]. ",
        "Tagref checks that tags are unique and that references are not dangling.\n\n",
        "More information can be found at: ",
        env!("CARGO_PKG_HOMEPAGE"),
    ),
    version,
    disable_version_flag = true
)]
struct Cli {
    #[arg(short, long, help = "Print version", action = ArgAction::Version)]
    _version: Option<bool>,

    #[arg(
        short,
        long,
        value_name = "CONFIG",
        help = "Use a config file instead of searching for tagref.yml"
    )]
    config: Option<PathBuf>,

    #[command(subcommand)]
    command: Option<Subcommand>,
}

#[derive(Args)]
struct ListUnusedArgs {
    #[arg(
        long,
        help = "Exit with an error status code if any tags are unreferenced"
    )]
    fail_if_any: bool,
}

#[derive(ClapSubcommand)]
enum Subcommand {
    #[command(about = "Check all the tags and references (default)")]
    Check,

    #[command(about = "List all the tags")]
    ListTags,

    #[command(about = "List all the tag references")]
    ListRefs,

    #[command(about = "List all the file references")]
    ListFiles,

    #[command(about = "List all the directory references")]
    ListDirs,

    #[command(about = "List the unreferenced tags")]
    ListUnused(ListUnusedArgs),
}

// Program entrypoint
#[allow(clippy::too_many_lines)]
fn entry() -> Result<(), String> {
    // Determine whether to print colored output.
    colored::control::set_override(io::stdout().is_terminal());

    // Parse the command-line options.
    let cli = Cli::parse();

    // Load the configuration.
    let config = config::load(cli.config.as_deref())?;

    // Compile the regular expressions in advance.
    let tag_regex = compile_directive_regex(&config.tag_sigil);
    let ref_regex = compile_directive_regex(&config.ref_sigil);
    let file_regex = compile_directive_regex(&config.file_sigil);
    let dir_regex = compile_directive_regex(&config.dir_sigil);

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
    let project_root = config.project_root;
    let ignore_rules = config.ignore_rules;
    let project_root_clone = project_root.clone();
    let files_scanned = walk::walk(&project_root, &ignore_rules, move |file_path, file| {
        // The `unwrap` is safe because the walker is rooted at `project_root_clone`.
        let relative_file_path = file_path.strip_prefix(&project_root_clone).unwrap();
        let directives = directive::parse(
            &tag_regex_clone,
            &ref_regex_clone,
            &file_regex_clone,
            &dir_regex_clone,
            relative_file_path,
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
    })?;

    // Decide what to do based on the subcommand.
    match cli.command.unwrap_or(Subcommand::Check) {
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
            errors.extend(file_references::check(
                &project_root,
                &files.lock().unwrap(),
            ));

            // Check the directory references. The `unwrap` is safe assuming no poisoning.
            errors.extend(dir_references::check(&project_root, &dirs.lock().unwrap()));

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

        Subcommand::ListUnused(args) => {
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
            if args.fail_if_any && !tags.lock().unwrap().is_empty() {
                return Err("Found unused tags while using --fail-if-any".to_owned());
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

#[cfg(test)]
mod tests {
    use super::Cli;
    use clap::CommandFactory;

    #[test]
    fn verify_cli() {
        Cli::command().debug_assert();
    }
}
