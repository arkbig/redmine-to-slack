// Copyright © ArkBig
//! This file provides cli options and args.

pub fn parse() -> CliArgs {
    CliArgs::parse()
}

use clap::Parser as _;
/// Command Line Arguments
#[derive(Debug, clap::Parser)]
#[clap(author, version, about, long_about = None)]
pub struct CliArgs {
    /// Redmine access interval.
    ///
    /// Set the desired interval to access Redmine and retrieve updates within that time frame, specified in minutes.
    /// On the initial access, it retrieves the updates for the specified time duration.
    #[clap(short, long, value_name = "MINUTES", default_value_t = 10)]
    pub interval: u16,

    /// One-shot Mode
    ///
    /// Accesses the functionality once without looping via sleep, and then terminates.
    #[clap(long)]
    pub once: bool,

    /// Redmine Atom URL to subscribe.
    ///
    /// e.g.) https://my.redmine.jp/demo/projects/demo/issues.atom
    #[clap(value_parser, required = true)]
    pub subscribe_url: String,

    /// Slack webhook URL to notify.
    ///
    /// e.g.) :TODO:
    #[clap(value_parser, required = true)]
    pub notify_url: String,
}
