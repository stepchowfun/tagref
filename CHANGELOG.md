# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.5.0] - 2021-06-20

### Added
- Tagref now supports two new platforms: (1) Windows, and (2) Linux without glibc.

## [1.4.1] - 2020-12-08

### Changed
- Tagref now always respects `.gitignore`. Previously, Tagref would only respect `.gitignore` if there was a `.git` directory.

## [1.4.0] - 2020-12-08

Nothing changed in this release.

## [1.3.3] - 2020-10-23

### Changed
- Tagref now always skips two hidden directories: `.git` and `.hg`.

## [1.3.2] - 2020-10-23

### Changed
- Tagref no longer skips hidden files and directories.

## [1.3.1] - 2020-07-30

### Changed
- Tagref is now faster because it now compiles the regular expressions once, rather than once per file.

## [1.3.0] - 2020-07-30

### Added
- Added support for `--ref-prefix` and `--tag-prefix`.

## [1.2.1] - 2019-11-26

### Removed
- Removed the useless `--version` flag for subcommands.

## [1.2.0] - 2019-05-22

### Added
- Every release from this point forward will include checksums of the precompiled binaries.

## [1.1.0] - 2019-05-06

### Added
- Added support for scanning multiple paths.

## [1.0.0] - 2019-05-06

### Added
- Commitment to stability.
