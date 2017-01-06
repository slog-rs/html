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

    let d1 = slog_stream::stream(
        file,
        slog_html::default()
    );
    let d2 = slog_stream::stream(
        std::io::stderr(),
        slog_html::default()
    );

    let log = slog::Logger::root(
        slog::duplicate(d1, d2).fuse(),
        o!("version" => env!("CARGO_PKG_VERSION"))
    );

    trace!(log, "logging a trace message");
    debug!(log, "debug values"; "x" => 1, "y" => -1);
    info!(log, "some interesting info"; "where" => "right here");
    warn!(log, "be cautious!"; "why" => "you never know...");
    error!(log, "type" => "unknown"; "wrong {}", "foobar");
    crit!(log, "abandoning test");
}
