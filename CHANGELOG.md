<!-- markdownlint-disable blanks-around-headings blanks-around-lists no-duplicate-heading -->

# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- next-header -->
## [Unreleased] - ReleaseDate
## [0.6.0] - 2023-01-05
## Changed
- [PR#17](https://github.com/EmbarkStudios/rymder/pull/17) updated tonic from 0.7 -> 0.8
- [PR#18](https://github.com/EmbarkStudios/rymder/pull/18) updated to agones 1.28.0

## [0.5.0] - 2022-05-11
## Changed
- [PR#16](https://github.com/EmbarkStudios/rymder/pull/16) updated to agones 1.21.0.
- [PR#16](https://github.com/EmbarkStudios/rymder/pull/16) updated to tonic from 0.6 to 0.7

## Added
- [PR#15](https://github.com/EmbarkStudios/rymder/pull/15) added `Clone` impls to various public structs. Thanks [@mvlabat](https://github.com/mvlabat)!

## [0.4.0] - 2022-03-03
### Changed
- [PR#5](https://github.com/EmbarkStudios/rymder/pull/14) replaced `chrono` with `time` due to `chrono` being unmaintained.

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
[Unreleased]: https://github.com/EmbarkStudios/rymder/compare/0.6.0...HEAD
[0.6.0]: https://github.com/EmbarkStudios/rymder/compare/0.5.0...0.6.0
[0.5.0]: https://github.com/EmbarkStudios/rymder/compare/0.4.0...0.5.0
[0.4.0]: https://github.com/EmbarkStudios/rymder/compare/0.3.0...0.4.0
[0.3.0]: https://github.com/EmbarkStudios/rymder/compare/0.2.2...0.3.0
[0.2.2]: https://github.com/EmbarkStudios/rymder/compare/0.2.1...0.2.2
[0.2.1]: https://github.com/EmbarkStudios/rymder/compare/0.2.0...0.2.1
[0.2.0]: https://github.com/EmbarkStudios/rymder/compare/0.1.0...0.2.0
[0.1.0]: https://github.com/EmbarkStudios/rymder/releases/tag/0.1.0
