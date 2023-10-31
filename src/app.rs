// Copyright © ArkBig
//! This file provides application flow.

/// The application is started and terminated.
///
/// 1. Subscribe to Redmine Atom.
/// 2. Get action that have been updated.
/// 3. Notify Slack.
pub fn run() -> proc_exit::ExitResult {
    let cli_args = crate::cli_args::parse();
    if cli_args.verbose {
        crate::log::set_level(crate::log::Severity::Debug);
    }

    let mut ret = (proc_exit::Code::SUCCESS, None);

    // Subscribe to Redmine
    let redmine_args = &cli_args.redmine;
    let prev_redmine_data = crate::redmine::load_prev_data(&redmine_args);
    if let Err(err) = prev_redmine_data {
        ret = (proc_exit::Code::FAILURE, Some(err.to_string()));
        return Err(proc_exit::Exit::new(ret.0).with_message(ret.1.unwrap()));
    }
    let mut prev_redmine_data = prev_redmine_data.unwrap();

    let updated_issues =
        crate::redmine::refresh_updated_issues(&redmine_args, &mut prev_redmine_data);
    if let Err(err) = updated_issues {
        ret = (proc_exit::Code::FAILURE, Some(err.to_string()));
        return Err(proc_exit::Exit::new(ret.0).with_message(ret.1.unwrap()));
    }

    // Check exists
    let mut updated_issues = updated_issues.unwrap();
    if updated_issues.is_empty() {
        println!("No updated issues.");
        return Ok(());
    }

    if let Some(notify_url) = &cli_args.slack.notify_url {
        // Sort by updated_on
        updated_issues.sort_by(|a, b| a.local_updated_time.cmp(&b.local_updated_time));
        // Notify to Slack
        let slack_args = &cli_args.slack;
        for update in updated_issues {
            let result = crate::slack::notify(slack_args, notify_url, &update);
            if let Err(err) = result {
                ret = (proc_exit::Code::FAILURE, Some(err.to_string()));
                return Err(proc_exit::Exit::new(ret.0).with_message(ret.1.unwrap()));
            }
        }
    }

    // Save updated data
    let result = crate::redmine::save_purged_data(redmine_args, &mut prev_redmine_data);
    if let Err(err) = result {
        ret = (proc_exit::Code::FAILURE, Some(err.to_string()));
        return Err(proc_exit::Exit::new(ret.0).with_message(ret.1.unwrap()));
    }

    // Exit Code
    let exit_code = ret.0;
    let exit_msg: Option<String> = ret.1;
    if exit_code == proc_exit::Code::SUCCESS && exit_msg.is_none() {
        Ok(())
    } else {
        let res = proc_exit::Exit::new(exit_code);
        if let Some(msg) = exit_msg {
            Err(res.with_message(msg))
        } else {
            Err(res)
        }
    }
}
