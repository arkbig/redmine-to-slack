# Redmine to Slack

This `redmine-to-slack` command subscribes to Redmine and notifies Slack.

## TODO

**[WIP] NOT working yet.**

<!--
## Demo

![demo](docs/demo.gif)

## Key Features

- Subscribe to Redmine Atom.
  - And even get what action happened.
- Notify Slack.

## Motivation

- Want to use Redmine even if it has a connection source limitation.
- Want to get the reason for the update.

## Installation

### Rust Version

MSRV: `redmine-to-slack` requires rustc 1.70.0 or newer.

### Dependent tools

nothing.

### redmine-to-slack

```sh
cargo install redmine-to-slack
```

Other convenient package is available yet.

## Usage

Please use the `-h`(shortly) or `--help`(fully) option for more information.

### Basic usage

```sh
redmine-to-slack https://my.redmine.jp/demo/projects/demo/issues.atom <:TODO: slack webhook endpoint>
```

result:

```txt
TODO
slack notify
```

## Roadmap

- [ ] Subscribe to Redmine Atom.
- [ ] Notification Slack.
- [ ] Get what action happened.

See the open issues for a full list of proposed features (and known issues).

## Alternative tools

`redmine-to-slack` is similar to the [RSS|Slack App](https://slack.com/apps/A0F81R7U7-rss?tab=more_info).
The extension can run on a machine that you manage and can also take the reason for the update.

## Dependencies

Direct dependencies crates:

```sh
cargo license --direct-deps-only --avoid-build-deps --avoid-dev-deps | awk -F ":" 'BEGIN {printf "|License|crate|\n|-|-|\n"} {printf "|%s|%s|\n", $1, $2}'
```

|License|crate|
|-|-|
|Apache-2.0 OR MIT (9)| anyhow, chrono, clap, proc-exit, serde, serde_json, signal-hook, thiserror, ureq|
|MIT (1)| feed-rs|
|Zlib (1)| redmine-to-slack|

Chain dependencies crates:

```sh
cargo license --avoid-build-deps --avoid-dev-deps | awk -F ":" 'BEGIN {printf "|License|crate|\n|-|-|\n"} {printf "|%s|%s|\n", $1, $2}'
```

|License|crate|
|-|-|
|(Apache-2.0 OR MIT) AND BSD-3-Clause (1)| encoding_rs|
|(MIT OR Apache-2.0) AND Unicode-DFS-2016 (1)| unicode-ident|
|0BSD OR Apache-2.0 OR MIT (1)| adler|
|Apache-2.0 OR Apache-2.0 WITH LLVM-exception OR MIT (4)| linux-raw-sys, rustix, wasi, wasi|
|Apache-2.0 OR BSL-1.0 (1)| ryu|
|Apache-2.0 OR ISC OR MIT (2)| rustls, sct|
|Apache-2.0 OR MIT (79)| android-tzdata, android_system_properties, anstream, anstyle, anstyle-parse, anstyle-query, anstyle-wincon, anyhow, base64, bitflags, bumpalo, cfg-if, chrono, clap, clap_builder, clap_derive, clap_lex, colorchoice, core-foundation-sys, crc32fast, errno, flate2, form_urlencoded, getrandom, heck, hermit-abi, iana-time-zone, iana-time-zone-haiku, idna, itoa, js-sys, lazy_static, libc, log, mime, num-traits, once_cell, percent-encoding, proc-exit, proc-macro2, quote, regex, regex-automata, regex-syntax, serde, serde_derive, serde_json, signal-hook, signal-hook-registry, siphasher, syn, thiserror, thiserror-impl, time, unicode-bidi, unicode-normalization, ureq, url, utf8parse, uuid, wasm-bindgen, wasm-bindgen-backend, wasm-bindgen-macro, wasm-bindgen-macro-support, wasm-bindgen-shared, web-sys, winapi, winapi-i686-pc-windows-gnu, winapi-x86_64-pc-windows-gnu, windows, windows-sys, windows-targets, windows_aarch64_gnullvm, windows_aarch64_msvc, windows_i686_gnu, windows_i686_msvc, windows_x86_64_gnu, windows_x86_64_gnullvm, windows_x86_64_msvc|
|Apache-2.0 OR MIT OR Zlib (3)| miniz_oxide, tinyvec, tinyvec_macros|
|Custom License File (2)| ring, rustls-webpki|
|ISC (2)| rustls-webpki, untrusted|
|MIT (6)| errno-dragonfly, feed-rs, is-terminal, quick-xml, spin, strsim|
|MIT OR Unlicense (2)| aho-corasick, memchr|
|MPL-2.0 (1)| webpki-roots|
|Zlib (1)| redmine-to-slack|
-->

## License

"redmine-to-slack" is under [zlib License](./LICENSE). Please feel free to use this, but no warranty.
