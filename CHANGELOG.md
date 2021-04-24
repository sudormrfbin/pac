# Changelog

<!-- Based on https://keepachangelog.com/en/1.0.0/ -->

## Unreleased

## Added

- `--rev` flag which replaces `--branch` et. al for install command (uses revision strings)


## 0.1.2

### Added

- Checkout a different branch, tag or commit upon plugin install (`--branch`, `--tag`, `--commit` arguments for install command)

## 0.1.1

### Added
- Colored help
- Checkout the default branch when installing plugins (`master` may not be the default)

## 0.1.0

### Added
- This changelog ;)
- `--as` option for install command to install package under a different name

### Removed
- Per plugin configuration and related features have been removed, use vimrc for plugin configuration
- Local plugin support removed, use manual symlinks instead

### Changed
- Complete package names where applicable for fish shell
- Fail visibly for invalid package names when uninstalling
