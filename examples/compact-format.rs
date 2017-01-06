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
        slog_html::new().compact().build()
    );
    let d2 = slog_stream::stream(
        std::io::stderr(),
        slog_html::new().compact().build()
    );

    let root_log = slog::Logger::root(
        slog::duplicate(d1, d2).fuse(),
        o!("version" => env!("CARGO_PKG_VERSION"))
    );

    let server_log = root_log.new(o!("host" => "localhost", "port" => "8080"));
    let peer1_log = server_log.new(o!("peer_addr" => "8.8.8.8", "port" => "18230"));
    let peer2_log = server_log.new(o!("peer_addr" => "82.9.9.9", "port" => "42381"));

    info!(server_log, "starting");
    info!(server_log, "listening");
    debug!(peer2_log, "connected");
    debug!(peer2_log, "message received"; "length" => 2);
    debug!(peer1_log, "connected");
    debug!(peer2_log, "response sent"; "length" => 8);
    debug!(peer2_log, "disconnected");
    debug!(peer1_log, "message received"; "length" => 2);
    debug!(peer1_log, "response sent"; "length" => 8);
    debug!(peer1_log, "disconnected");
    info!(server_log, "exit");
}
