# Changelog

<!-- All notable changes to this project will be documented in this file. -->

<!-- The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), -->
<!-- and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html). -->

<!-- Template

## Unreleased

### Breaking changes

### Changed

### Added

### Fixed

### Removed

### Deprecated

### Performance

### Security

-->

## Unreleased

### Added

- `ReadParam` for `isize` and `usize`
- macros: `from` attr for parameters
- macros: support `Option` in `from` attr
- new `PositiveNumber` and `NegativeNumber` types
- take the ownership in `Filter` functions
- use the correct ffi function for `event_get_pubkey*`
- new `StaticCell` type
- new `StaticFilter` type
- new `ShortEventId` and `ShortPubKey` types

### Removed

- macros: filename from panic logs

## v0.0.1 - 2026/04/10

First release.

