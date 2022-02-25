<!-- markdownlint-disable blanks-around-headings blanks-around-lists no-duplicate-heading -->

# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- next-header -->
## [Unreleased] - ReleaseDate

## [0.3.0] - 2022-01-11

### Changed
- [PR#5](https://github.com/EmbarkStudios/rymder/pull/5) bump tonic to `0.6` and prost to `0.9`
- [PR#5](https://github.com/EmbarkStudios/rymder/pull/5) generate bindings for agones 1.19

## [0.2.2] - 2021-09-03

### Fixed
- [PR#4](https://github.com/EmbarkStudios/rymder/pull/4) added the missing `PortAllocation`, `Creating`, `Starting`, `RequestReady`, and `Error` states.

## [0.2.1] - 2021-09-02
### Fixed
- [PR#3](https://github.com/EmbarkStudios/rymder/pull/3) added the missing `Scheduled` and `Unhealthy` states.

## [0.2.0] - 2021-09-01
### Added
- Added wrappers around the types exposed by `GameServer` to improve the type information provided, for example, all timestamps are now `chrono::DateTime<Utc>` and all durations are now `std::time::Duration` so that one doesn't need to look at the protobuf definitions or Agones documentation to figure what units are used etc.

### Changed
- Renamed `Sdk::new` to `Sdk::connect` and changed the internal behavior to take an overall `connect_timeout` that determines the maximum amount of time that can be spent both connecting to the Agones SDK server, as well as retrieving the initial `GameServer` state, removing the need to wrap `new` to retry connections while waiting for the SDK server to finish spinning up.

## [0.1.0] - 2021-08-23
### Added
- First pass implementation for [Agones 1.16.0](https://agones.dev/site/blog/2021/07/20/1.16.0-kubernetes-1.19-golang-1.15/).

<!-- next-url -->
[Unreleased]: https://github.com/EmbarkStudios/rymder/compare/0.3.0...HEAD
[0.3.0]: https://github.com/EmbarkStudios/rymder/compare/0.2.2...0.3.0
[0.2.2]: https://github.com/EmbarkStudios/rymder/compare/0.2.1...0.2.2
[0.2.1]: https://github.com/EmbarkStudios/rymder/compare/0.2.0...0.2.1
[0.2.0]: https://github.com/EmbarkStudios/rymder/compare/0.1.0...0.2.0
[0.1.0]: https://github.com/EmbarkStudios/rymder/releases/tag/0.1.0
