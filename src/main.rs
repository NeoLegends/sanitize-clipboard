#[macro_use]
extern crate clap;
extern crate clipboard;
extern crate url;

use clap::{AppSettings, Arg};
use std::time::Duration;

mod sanitize;

const POLL_INTERVAL_PARAM: &'static str = "POLL_INTERVAL";

fn main() {
    let matches = app_from_crate!()
        .setting(AppSettings::DisableHelpSubcommand)
        .setting(AppSettings::GlobalVersion)
        .arg(
            Arg::with_name(POLL_INTERVAL_PARAM)
                .help("The interval to poll the clipboard (in milliseconds) at. Set to larger numbers to reduce CPU load at the expense of usability.")
                .takes_value(true)
                .short("i")
                .long("interval")
                .global(true)
                .default_value("500")
        )
        .get_matches();

    let poll_interval = {
        let millis =
            value_t!(matches.value_of(POLL_INTERVAL_PARAM), u64).unwrap_or_else(|e| e.exit());
        Duration::from_millis(millis)
    };

    sanitize::sanitize_loop(poll_interval);
}
