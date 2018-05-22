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

If you are running macOS or a GNU-based Linux on an x86-64 CPU, the following will install Tagref to `/usr/local/bin`:

```sh
curl -LSfs https://raw.githubusercontent.com/stepchowfun/tagref/master/install.sh | sudo sh
```

If you want to install to a different location, you can download a binary from the [releases page](https://github.com/stepchowfun/tagref/releases) and put it anywhere on your `$PATH`. If there is no pre-built binary available for your platform, you can build and install it with [Cargo](https://doc.rust-lang.org/book/second-edition/ch14-04-installing-binaries.html).

## Usage

The easiest way to use Tagref is to run the `tagref` command with no arguments. It will scan the working directory and check the two conditions described above. Here are the supported command-line options:

```
USAGE:
    tagref [FLAGS] [OPTIONS]

FLAGS:
    -h, --help               Prints help information
    -r, --list-references    Lists all the references
    -n, --list-tags          Lists all the tags
    -V, --version            Prints version information

OPTIONS:
    -p, --path <PATH>    Sets the path of the directory to scan
```

## Acknowledgements

The idea for Tagref was inspired by [the GHC notes convention](https://ghc.haskell.org/trac/ghc/wiki/Commentary/CodingStyle#Commentsinthesourcecode). GHC is one of the most maintainable codebases for its size. [This article](http://www.aosabook.org/en/ghc.html) has more insights into how the GHC developers manage that codebase.
