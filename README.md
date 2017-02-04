<p align="center">
  <a href="https://github.com/slog-rs/slog">
  <img src="https://cdn.rawgit.com/slog-rs/misc/master/media/slog.svg" alt="slog-rs logo">
  </a>
  <br>

  <a href="https://travis-ci.org/slog-rs/html">
      <img src="https://img.shields.io/travis/slog-rs/html/master.svg" alt="Travis CI Build Status">
  </a>

  <a href="https://ci.appveyor.com/project/slog-rs/html">
      <img src="https://ci.appveyor.com/api/projects/status/github/slog-rs/html?svg=true" alt="AppVeyor Build Status">
  </a>

  <a href="https://crates.io/crates/slog-html">
      <img src="https://img.shields.io/crates/v/slog-html.svg" alt="slog-html on crates.io">
  </a>

  <a href="https://gitter.im/slog-rs/slog">
      <img src="https://img.shields.io/gitter/room/slog-rs/slog.svg" alt="slog-rs Gitter Chat">
  </a>

  <a href="https://docs.rs/slog-html">
      <img src="https://img.shields.io/badge/documentation-docs.rs-df3600.svg" alt="slog-html documentation">
  </a>
</p>

# slog-html - HTML format for [slog-stream] of [slog-rs]

[slog-rs]: //github.com/slog-rs/slog
[slog-stream]: //github.com/slog-rs/stream

## Example

```rust
#[macro_use]
extern crate slog;
extern crate slog_html;
extern crate slog_stream;

use slog::DrainExt;

use std::fs::OpenOptions;

fn main() {
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("target/log.html").unwrap();

    let log = slog::Logger::root(
        slog_stream::stream(
            file,
            slog_html::default()
        ).fuse(),
        o!("version" => env!("CARGO_PKG_VERSION"))
    );

    debug!(log, "debug values"; "x" => 1, "y" => -1);
}
```

## Rendered example output

<img src="https://i.imgur.com/7xyv5Sg.png" width="601" height="130" alt="slog-rs html full-format output">

Compact mode:

<img src="https://i.imgur.com/Ur6g8Q4.png" width="434" height="349" alt="slog-rs html compact output">

Full mode:

<img src="https://i.imgur.com/mVvzYCN.png" width="960" height="230" alt="slog-rs html full output">
