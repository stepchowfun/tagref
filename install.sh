#!/usr/bin/env sh

# Where the binary will be installed
PREFIX=/usr/local/bin

# Which version to download
RELEASE=v0.0.2

# Determine which binary to download.
FILENAME=''
if uname -a | grep -qi 'x86_64.*GNU/Linux'; then
  echo 'x86_64 GNU/Linux detected.'
  FILENAME=tagref-x86_64-unknown-linux-gnu
fi
if uname -a | grep -qi 'Darwin.*x86_64'; then
  echo 'macOS detected.'
  FILENAME=tagref-x86_64-apple-darwin
fi

# Fail if there is no pre-built binary for this platform.
if [ -z "$FILENAME" ]; then
  echo 'Unfortunately, there is no pre-built binary for this platform.' 1>&2
  exit 1
fi

# Download the binary.
if ! curl -LSf "https://github.com/stepchowfun/tagref/releases/download/$RELEASE/$FILENAME" -o "$PREFIX/tagref"; then
  echo 'There was an error downloading the binary.' 1>&2
  exit 1
fi

# Make it executable.
if ! chmod a+rx "$PREFIX/tagref"; then
  echo 'There was an error setting the permissions for the binary.' 1>&2
  exit 1
fi

# Let the user know it worked.
echo 'Tagref is now installed.'
