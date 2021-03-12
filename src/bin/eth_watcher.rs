use std::time::Duration;

use futures::{channel::mpsc, SinkExt};
use tokio::{runtime::Runtime, time};

fn main() {
    // Always print backtrace on panic.
    ::std::env::set_var("RUST_BACKTRACE", "1");
    ::std::env::set_var("RUST_LOG", "Info");
    env_logger::init();
}
