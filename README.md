# Tagref

[![Build status](https://github.com/stepchowfun/tagref/workflows/Continuous%20integration/badge.svg?branch=main)](https://github.com/stepchowfun/tagref/actions?query=branch%3Amain)

Tagref helps you manage cross-references in your code. You can use it to help keep things in sync, document assumptions, maintain invariants, etc. Airbnb uses it for their front-end monorepo. You can use it too!

Tagref works with any programming language, and it respects your `.gitignore` file as well as other common filter files. It's recommended to set up Tagref as an automated continuous integration (CI) check. Tagref is fast and almost certainly won't be the bottleneck in your CI.

## What is it?

*Tagref* allows you to annotate your code with *tags* (in comments) which can be referenced from other parts of the codebase. For example, you might have a tag like this:

```python
# [tag:cities_nonempty] This function always returns a non-empty list.
def get_cities():
    return ['San Francisco', 'Tokyo']
```

Elsewhere, suppose you're writing some code which depends on that postcondition. You can make that clear by referencing the tag:

```python
cities = get_cities()

first_city = cities[0] # This is safe due to [ref:cities_nonempty].
```

Tagref ensures such references remain valid. If someone tries to delete or rename the tag, Tagref will complain. More precisely, it checks the following:

1. References actually point to tags. A tag cannot be deleted or renamed without updating the references that point to it.
2. Tags are unique. There is never any ambiguity about which tag is being referenced.

Note that, in the example above, Tagref won't ensure that the `get_cities` function actually returns a non-empty list. It isn't magic! It only checks the two conditions above.

## Usage

The easiest way to use Tagref is to run the `tagref` command with no arguments. It will recursively scan the working directory and check the two conditions described above. Here are the supported command-line options:

```
USAGE:
    tagref [SUBCOMMAND]

OPTIONS:
    -h, --help
            Prints help information

    -p, --path <PATH>...
            Adds the path of a directory to scan [default: .]

    -r, --ref-prefix <REF_PREFIX>
            Sets the prefix used for locating references [default: ref]

    -t, --tag-prefix <TAG_PREFIX>
            Sets the prefix used for locating tags [default: tag]

    -v, --version
            Prints version information

SUBCOMMANDS:
    check
            Checks all the tags and references (default)

    help
            Prints this message or the help of the given subcommand(s)

    list-refs
            Lists all the references

    list-tags
            Lists all the tags

    list-unused
            Lists the unreferenced tags
```

## Installation instructions

### Installation on macOS or Linux (AArch64 or x86-64)

If you're running macOS or Linux (AArch64 or x86-64), you can install Tagref with this command:

```sh
curl https://raw.githubusercontent.com/stepchowfun/tagref/main/install.sh -LSfs | sh
```

The same command can be used again to update to the latest version.

The installation script supports the following optional environment variables:

- `VERSION=x.y.z` (defaults to the latest version)
- `PREFIX=/path/to/install` (defaults to `/usr/local/bin`)

For example, the following will install Tagref into the working directory:

```sh
curl https://raw.githubusercontent.com/stepchowfun/tagref/main/install.sh -LSfs | PREFIX=. sh
```

If you prefer not to use this installation method, you can download the binary from the [releases page](https://github.com/stepchowfun/tagref/releases), make it executable (e.g., with `chmod`), and place it in some directory in your [`PATH`](https://en.wikipedia.org/wiki/PATH_\(variable\)) (e.g., `/usr/local/bin`).

### Installation on Windows (AArch64 or x86-64)

If you're running Windows (AArch64 or x86-64), download the latest binary from the [releases page](https://github.com/stepchowfun/tagref/releases) and rename it to `tagref` (or `tagref.exe` if you have file extensions visible). Create a directory called `Tagref` in your `%PROGRAMFILES%` directory (e.g., `C:\Program Files\Tagref`), and place the renamed binary in there. Then, in the "Advanced" tab of the "System Properties" section of Control Panel, click on "Environment Variables..." and add the full path to the new `Tagref` directory to the `PATH` variable under "System variables". Note that the `Program Files` directory might have a different name if Windows is configured for a language other than English.

To update an existing installation, simply replace the existing binary.

### Installation with Homebrew

If you have [Homebrew](https://brew.sh/), you can install Tagref as follows:

```sh
brew install tagref
```

You can update an existing installation with `brew upgrade tagref`.

### Installation with Cargo

If you have [Cargo](https://doc.rust-lang.org/cargo/), you can install Tagref as follows:

```sh
cargo install tagref
```

You can run that command with `--force` to update an existing installation.

## Acknowledgements

The idea for Tagref was inspired by [the GHC notes convention](https://ghc.haskell.org/trac/ghc/wiki/Commentary/CodingStyle#Commentsinthesourcecode). [This article](http://www.aosabook.org/en/ghc.html) has more insights into how the GHC developers manage their codebase.
