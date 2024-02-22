# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.9.1] - 2024-02-21

### Fixed
- Fixed a typo in the help text.

## [1.9.0] - 2024-02-21

### Added
- Tagref now supports file and directory references.

### Changed
- The `--tag-prefix` and `--ref-prefix` options have been renamed to `--tag-sigil` and `--ref-sigil`, respectively.
- Tagref now reports all errors rather than just a subset of them.

## [1.8.5] - 2024-02-16

### Added
- Tagref now provides a [pre-commit](https://pre-commit.com/) hook configuration.

## [1.8.4] - 2023-06-18

### Added
- Tagref supports a new platform: Windows on AArch64.

## [1.8.3] - 2023-06-02

### Added
- Tagref supports a new platform: musl Linux on AArch64.

## [1.8.2] - 2023-05-22

### Added
- Tagref supports a new platform: GNU Linux on AArch64.

## [1.8.1] - 2023-05-11

### Added
- Tagref supports a new platform: macOS on Apple silicon.

## [1.8.0] - 2023-05-10

### Added
- Added the `--fail-if-any` flag to the `list-unused` subcommand.

## [1.7.0] - 2023-04-03

### Changed
- The `list-tags`, `list-refs`, and `list-unused` subcommands now have their original behavior, the same as in v1.5.0. The fancy two-column output was too buggy.

## [1.6.1] - 2023-04-03

### Fixed
- The `list-tags`, `list-refs`, and `list-unused` subcommands now render their output with the correct width.

## [1.6.0] - 2023-03-15

### Changed
- The `list-tags`, `list-refs`, and `list-unused` subcommands now print their output in a nice columnated format thanks to Manos Pitsidianakis.

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
