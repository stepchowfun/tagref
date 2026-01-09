# Tagref

[![Build status](https://github.com/stepchowfun/tagref/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/stepchowfun/tagref/actions?query=branch%3Amain)

![Welcome to Tagref.](https://raw.githubusercontent.com/stepchowfun/tagref/main/tagref.svg?sanitize=true)

*Tagref* helps you manage cross-references in your code. You can use it to help keep things in sync, document assumptions, maintain invariants, etc. [Airbnb](https://www.airbnb.com/), [Notion](https://www.notion.so/), and [Watershed](https://watershed.com/) use it to level up their code health. You can use it too!

Tagref works with any programming language, and it respects your `.gitignore` file as well as other common filter files. It's recommended to set up Tagref as an automated continuous integration (CI) check. Tagref is fast and almost certainly won't be the bottleneck in your CI.

## What is it?

Tagref allows you to annotate your code with *tags* (in comments) which can be *referenced* from other parts of the codebase.

Here's an example in Python:

```python
# [tag:polynomial_nonzero] This function never returns zero.
def polynomial(x):
    return x ** 2 + 1

def inverse_polynomial(x):
    return 1 / polynomial(x) # This is safe due to [ref:polynomial_nonzero].
```

To help you manage these tags and references, Tagref checks the following:

1. References actually point to tags. A tag cannot be deleted or renamed without updating the references that point to it.
2. Tags are unique. There is never any ambiguity about which tag is being referenced.

In the example above, Tagref doesn't guarantee that `polynomial` returns a nonzero number. It isn't magic! It only ensures that the `polynomial_nonzero` tag exists unambiguously. The programmer is still responsible for keeping the comments in sync with the code.

In addition to references to tags, Tagref also supports *file references* and *directory references*. A file reference guarantees that the given file exists. For example:

```python
# If you bump the version, be sure to update [file:CHANGELOG.md].
```

A directory reference guarantees that the given directory exists. For example:

```python
# This script will format the files in [dir:src].
```

File and directory paths are relative to the working directory, which is typically the root of the project or repository.

## Tag names

The name of a tag may consist of any UTF-8 text except the right square bracket `]`. Internal whitespace (as in `[tag:foo bar]`) is allowed, and surrounding whitespace (as in `[tag: baz ]`) is ignored. Tag names are case-sensitive, so `[tag:foo]` and `[tag:Foo]` are different tags.

You can use any naming convention you like. The Tagref authors prefer to use lowercase words separated by underscores `_`, like `[tag:important_note]`.

## Usage

The easiest way to use Tagref is to run the `tagref` command with no arguments. It will recursively scan the working directory and check all the tags and references. Here are the supported command-line options:

```
USAGE:
    tagref [SUBCOMMAND]

OPTIONS:
    -d, --dir-sigil <DIR_SIGIL>
            Sets the sigil used for directory references [default: dir]

    -f, --file-sigil <FILE_SIGIL>
            Sets the sigil used for file references [default: file]

    -h, --help
            Prints help information

    -p, --path <PATH>...
            Adds the path of a directory to scan [default: .]

    -r, --ref-sigil <REF_SIGIL>
            Sets the sigil used for tag references [default: ref]

    -t, --tag-sigil <TAG_SIGIL>
            Sets the sigil used for tags [default: tag]

    -v, --version
            Prints version information


SUBCOMMANDS:
    check
            Checks all the tags and references (default)

    help
            Prints this message or the help of the given subcommand(s)

    list-dirs
            Lists all the directory references

    list-files
            Lists all the file references

    list-refs
            Lists all the tag references

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

### Installation with pre-commit

If you use [pre-commit](https://pre-commit.com/), you can install Tagref by adding it to your `.pre-commit-config.yaml` as follows:

```yaml
repos:
- repo: https://github.com/stepchowfun/tagref
  rev: v1.10.0
  hooks:
  - id: tagref
```

If you happen to have Rust installed, make sure it's up-to-date since pre-commit will use it to install Tagref. If you don't already have Rust, pre-commit will install it for you.

## Editor integrations

- [tagref.el](https://github.com/vedang/tagref.el): An Emacs minor mode with tag/reference completion, xref-based navigation, and validation support.

## Acknowledgements

The idea for Tagref was inspired by the GHC *notes* system described in [this article](http://www.aosabook.org/en/ghc.html) (§5.6).
