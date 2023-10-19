// Copyright © ArkBig
//! This file provides cli options and args.

use clap::Parser as _;
/// Command Line Arguments
#[derive(Debug, clap::Parser)]
#[clap(author, version, about, long_about = None)]
pub struct CliArgs {
    /// Args for Redmine.
    #[clap(flatten)]
    pub redmine: RedmineArgs,

    /// Args for Slack.
    #[clap(flatten)]
    pub slack: SlackArgs,
}

#[derive(Clone, Debug, clap::Parser)]
pub struct RedmineArgs {
    /// Redmine Atom URL to subscribe.
    ///
    /// e.g.) https://redmine-r2s.dev.test
    #[clap(value_parser, required = true)]
    pub subscribe_url: String,

    /// Redmine access interval.
    ///
    /// Set the desired interval to access Redmine and retrieve updates within that time frame, specified in minutes.
    /// On the initial access, it retrieves the updates for the specified time duration.
    /// If you specify 0, it will retrieve at once and exit.
    #[clap(short, long, value_name = "DURATION", default_value="0s", value_parser=humantime::parse_duration)]
    pub interval: std::time::Duration,

    /// Previous data path.
    ///
    /// Specify the file path to save the previous acquisition information.
    /// It will be overwritten and updated during processing.
    #[clap(long, value_name = "PATH", default_value = "redmine-data.json")]
    pub prev_redmine_data: String,

    /// Redmine API access key.
    ///
    /// If you specify a file starting starts with @, it will read from that file.
    /// If you specify a environment variable starting with $, it will read from that environment variable.
    #[clap(long, value_name = "KEY")]
    pub redmine_api_key: Option<String>,

    /// Maximum number of characters in the notification sentence
    #[clap(long, value_name = "NUM", default_value = "1000")]
    pub max_content_length: usize,

    /// Filter for getting issues.
    #[clap(flatten)]
    pub filter: FilterArgs,
}

#[derive(Clone, Debug, clap::Parser)]
pub struct FilterArgs {
    /// Target projects name identifier or number
    ///
    /// If not specified, all projects will be targeted.
    #[clap(short, long, value_name = "PROJ_ID(s)")]
    pub target_project: Option<Vec<String>>,

    /// Ignore projects name identifier or number.
    ///
    /// If not specified, no projects will be ignored.
    #[clap(long, value_name = "PROJ_ID(s)")]
    pub ignore_project: Option<Vec<String>>,

    ///Filter by users
    ///
    /// Specify by login id name or number. If me, the API Key itself is the target.
    #[clap(long, value_name = "USER(s)")]
    pub user: Option<Vec<String>>,
}

#[derive(Clone, Debug, clap::Parser)]
pub struct SlackArgs {
    /// Slack incoming webhook URL to notify.
    ///
    /// e.g.) https://hooks.slack.com/services/<TOKEN>
    #[clap(value_parser)]
    pub notify_url: Option<String>,

    /// Slack notification message template file path
    ///
    /// If not specified, the default template will be used.
    #[clap(long, value_name = "PATH")]
    pub template_path: Option<String>,
}

pub fn parse() -> CliArgs {
    let mut cli_args = CliArgs::parse();

    normalize_redmine(&mut cli_args.redmine);
    normalize_slack(&mut cli_args.slack);

    cli_args
}

fn normalize_redmine(args: &mut RedmineArgs) {
    if args.subscribe_url.ends_with("/") {
        args.subscribe_url.pop();
    }
    normalize_redmine_api_key(args).unwrap();
    normalize_filter(args).unwrap();
}

fn normalize_secret(value: &str) -> anyhow::Result<String> {
    // If specified file
    if value.starts_with("@") {
        let path = value.trim_start_matches("@");
        let value = std::fs::read_to_string(path).unwrap_or_else(|e| {
            let msg = format!(
                "Could not read secret from file [{path}] with {e}",
                path = path,
                e = e
            );
            crate::log::error(&msg).category("cli");
            std::process::exit(1);
        });
        Ok(value)
    }
    // If specified environment variable
    else if value.starts_with("$") {
        let var = value.trim_start_matches("$");
        let value = std::env::var(var).unwrap_or_else(|e| {
            let msg = format!(
                "Could not read secret from environment variable [{var}] with {e}",
                var = var,
                e = e
            );
            crate::log::error(&msg).category("cli");
            std::process::exit(1);
        });
        Ok(value)
    }
    // Otherwise
    else {
        Ok(value.to_string())
    }
}

fn normalize_redmine_api_key(args: &mut RedmineArgs) -> anyhow::Result<()> {
    if let Some(redmine_api_key) = &args.redmine_api_key {
        args.redmine_api_key = Some(normalize_secret(redmine_api_key)?);
    }
    Ok(())
}

fn normalize_filter(args: &mut RedmineArgs) -> anyhow::Result<()> {
    if args.filter.target_project.is_some() && args.filter.ignore_project.is_some() {
        let msg =
            format!("Cannot specify both target_project and ignore_project at the same time.");
        crate::log::error(&msg).category("cli");
        std::process::exit(1);
    }
    normalize_filter_target_project(args)?;
    normalize_filter_ignore_project(args)?;
    normalize_filter_user(args)?;
    Ok(())
}

fn normalize_filter_target_project(args: &mut RedmineArgs) -> anyhow::Result<()> {
    if args.filter.target_project.is_none() {
        return Ok(());
    }
    // If specified id, convert to name identifier.
    let mut list = None;
    for item in args.filter.target_project.as_mut().unwrap() {
        let id = item.parse::<u32>();
        if id.is_err() {
            continue;
        }
        let id = id.unwrap();
        if list.is_none() {
            list = Some(crate::redmine::get_projects(
                &args.subscribe_url,
                &args.redmine_api_key,
            )?);
        }
        let list = list.as_ref().unwrap();
        let project = list.iter().find(|p| p.id == id).unwrap_or_else(|| {
            let msg = format!("Could not find project id [{item}]", item = item);
            crate::log::error(&msg).category("cli");
            std::process::exit(1);
        });
        *item = project.name_id.clone();
    }
    Ok(())
}

fn normalize_filter_ignore_project(args: &mut RedmineArgs) -> anyhow::Result<()> {
    if args.filter.ignore_project.is_none() {
        return Ok(());
    }
    // If specified id or name identifier, convert to name.
    let list = crate::redmine::get_projects(&args.subscribe_url, &args.redmine_api_key)?;
    for item in args.filter.ignore_project.as_mut().unwrap() {
        let id = item.parse::<u32>();
        let project = list
            .iter()
            .find(|p| id.is_ok() && &p.id == id.as_ref().unwrap() || p.name_id.eq(item))
            .unwrap_or_else(|| {
                let msg = if id.is_ok() {
                    format!("Could not find project id [{item}]", item = item)
                } else {
                    format!(
                        "Could not find project name identifier [{item}]",
                        item = item
                    )
                };
                crate::log::error(&msg).category("cli");
                std::process::exit(1);
            });
        *item = project.name.clone();
    }

    Ok(())
}

fn normalize_filter_user(args: &mut RedmineArgs) -> anyhow::Result<()> {
    if args.filter.user.is_none() {
        return Ok(());
    }
    // If specified name, convert to id.
    let mut map = None;
    for item in args.filter.user.as_mut().unwrap() {
        if item.parse::<u32>().is_ok() {
            continue;
        }
        if item == "me" {
            continue;
        }
        if map.is_none() {
            map = Some(crate::redmine::get_users_map(
                &args.subscribe_url,
                &args.redmine_api_key,
            )?);
        }
        let map = map.as_ref().unwrap();
        let id = map.get(item).unwrap_or_else(|| {
            let msg = format!("Could not find user [{item}]", item = item);
            crate::log::error(&msg).category("cli");
            std::process::exit(1);
        });
        *item = id.to_string();
    }
    Ok(())
}

fn normalize_slack(args: &mut SlackArgs) {
    if let Some(notify_url) = args.notify_url.as_mut() {
        if notify_url.ends_with("/") {
            notify_url.pop();
        }
        // normalize_slack_oauth_token(args).unwrap();
    }
}

// fn normalize_slack_oauth_token(args: &mut SlackArgs) -> anyhow::Result<()> {
//     if let Some(slack_oauth_token) = &args.slack_oauth_token {
//         args.slack_oauth_token = Some(normalize_secret(slack_oauth_token)?);
//     }
//     Ok(())
// }
