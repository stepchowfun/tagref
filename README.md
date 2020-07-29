# Tagref

[![Build status](https://github.com/stepchowfun/tagref/workflows/Continuous%20integration/badge.svg?branch=master)](https://github.com/stepchowfun/tagref/actions?query=branch%3Amaster)

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

### Easy installation

If you are running macOS or a GNU-based Linux on an x86-64 CPU, you can install Tagref with this command:

```sh
curl https://raw.githubusercontent.com/stepchowfun/tagref/master/install.sh -LSfs | sh
```

The same command can be used again to update Tagref to the latest version.

**NOTE:** Piping `curl` to `sh` is dangerous since the server might be compromised. If you're concerned about this, you can download and inspect the installation script or choose one of the other installation methods.

#### Customizing the installation

The installation script supports the following environment variables:

- `VERSION=x.y.z` (defaults to the latest version)
- `PREFIX=/path/to/install` (defaults to `/usr/local/bin`)

For example, the following will install Tagref into the working directory:

```sh
curl https://raw.githubusercontent.com/stepchowfun/tagref/master/install.sh -LSfs | PREFIX=. sh
```

### Manual installation

The [releases page](https://github.com/stepchowfun/tagref/releases) has precompiled binaries for macOS or Linux systems running on an x86-64 CPU. You can download one of them and place it in a directory listed in your [`PATH`](https://en.wikipedia.org/wiki/PATH_\(variable\)).

### Installation with Cargo

If you have [Cargo](https://doc.rust-lang.org/cargo/), you can install Tagref as follows:

```sh
cargo install tagref
```

You can run that command with `--force` to update an existing installation.

## Usage

The easiest way to use Tagref is to run the `tagref` command with no arguments. It will scan the working directory and check the two conditions described above. Here are the supported command-line options:

```
USAGE:
    tagref [SUBCOMMAND]

OPTIONS:
    -h, --help                       Prints help information
    -p, --path <PATH>...             Adds the path of a directory to scan [default: .]
    -r, --ref-prefix <REF_PREFIX>    Sets the prefix used for locating references [default: ref]
    -t, --tag-prefix <TAG_PREFIX>    Sets the prefix used for locating tags [default: tag]
    -v, --version                    Prints version information

SUBCOMMANDS:
    check          Check all the tags and references (default)
    help           Prints this message or the help of the given subcommand(s)
    list-refs      List all the references
    list-tags      List all the tags
    list-unused    List the unreferenced tags
```

## Acknowledgements

The idea for Tagref was inspired by [the GHC notes convention](https://ghc.haskell.org/trac/ghc/wiki/Commentary/CodingStyle#Commentsinthesourcecode). [This article](http://www.aosabook.org/en/ghc.html) has more insights into how the GHC developers manage their codebase.
