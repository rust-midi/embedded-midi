# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
...

### Changed
...

## [0.1.2] - 2021-11-24
Dependencies update

### Fixes
- Update dependencies

## [0.1.1] - 2021-04-16
Dependency update

### Fixes
- Update dependencies

## [0.1.0] - 2020-11-20
Bugfix release, with thanks for David Stalnaker for contributions 

### Added
- Implement running status for midi-out
- Implement write for all messages

### Fixes
- Use github actions for CI
- Various bugfixes


## [0.0.2] - 2020-07-06

### Added
- Receive and parse all messages except system exclusive
- Parse running state messages

### Changed
- Move midi parsing to separate module
- Rename MidiEvent to MidiMessage

## [0.0.1] - 2020-06-17

### Added
- Basic representation of midi events
- Receive and parse note-on and note-off messages
- Send note-on and note-off messages
- Basic examples

[unreleased]: https://github.com/mendelt/embedded-midi/compare/0.1.1...HEAD
[0.1.1]: https://github.com/mendelt/embedded-midi/releases/tag/0.1.1
[0.1.0]: https://github.com/mendelt/embedded-midi/releases/tag/0.1.0
[0.0.2]: https://github.com/mendelt/embedded-midi/releases/tag/0.0.2
[0.0.1]: https://github.com/mendelt/embedded-midi/releases/tag/0.0.1
