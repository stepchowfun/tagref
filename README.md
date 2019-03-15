# Tagref

[![Build Status](https://travis-ci.org/stepchowfun/tagref.svg?branch=master)](https://travis-ci.org/stepchowfun/tagref)

*Tagref* helps you refer to other locations in your codebase. For example, you might create a tag like this:

```ruby
# This method always returns a non-empty list. [tag:wibble_nonempty]
def wibble(x)
  ...
end
```

Elsewhere, suppose you're writing some code which depends on that postcondition. You can make this clear by referencing the tag:

```ruby
flobs = wibble(wobble)

return flobs[0] # This is safe due to [ref:wibble_nonempty].
```

Tagref ensures such references remain valid. If someone tries to delete or rename the tag (e.g., because they want to change what `wibble` does), Tagref will complain. More precisely, it checks the following:

1. References actually point to tags. A tag cannot be deleted without updating the references that point to it.
2. Tags are distinct. There is never any ambiguity about which tag is being referenced.

The syntax is `[tag:label]` for tags and `[ref:label]` for references. Tagref works with any programming language, and it respects your `.gitignore` file as well as other common filter files. It's recommended to set up Tagref as an automated continuous integration check. Tagref is fast and probably won't be the bottleneck in your CI.

## Installation

### Default installation

If you are running macOS or a GNU-based Linux on an x86-64 CPU, you can install Tagref with this command:

```sh
curl -LSfs https://raw.githubusercontent.com/stepchowfun/tagref/master/install.sh | sudo sh
```

The same command can be used to update Tagref to the latest version.

### Custom installation

The installation script supports the following environment variables:

- `VERSION=x.y.z` (defaults to the latest version)
- `PREFIX=/path/to/install` (defaults to `/usr/local/bin`)

For example, the following will install Tagref into the current directory:

```sh
curl -LSfs https://raw.githubusercontent.com/stepchowfun/tagref/master/install.sh | PREFIX=. sh
```

### Installation on other platforms

If there is no pre-built binary available for your platform, you can build and install Tagref with [Cargo](https://doc.rust-lang.org/book/second-edition/ch14-04-installing-binaries.html):

```sh
cargo install tagref
```

Then you can update Tagref to the latest version using the `--force` flag:

```sh
cargo install tagref --force
```

## Usage

The easiest way to use Tagref is to run the `tagref` command with no arguments. It will scan the working directory and check the two conditions described above. Here are the supported command-line options:

```
USAGE:
    tagref [OPTIONS] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -p, --path <PATH>    Sets the path of the directory to scan

SUBCOMMANDS:
    check        Check all the tags and references (default)
    help         Prints this message or the help of the given subcommand(s)
    list-refs    List all the references
    list-tags    List all the tags
```

## Acknowledgements

The idea for Tagref was inspired by [the GHC notes convention](https://ghc.haskell.org/trac/ghc/wiki/Commentary/CodingStyle#Commentsinthesourcecode). GHC is one of the most maintainable codebases for its size. [This article](http://www.aosabook.org/en/ghc.html) has more insights into how the GHC developers manage that codebase.
