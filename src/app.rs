// Copyright © ArkBig
//! This file provides application flow.

/// The application is started and terminated.
///
/// 1. Subscribe to Redmine Atom.
/// 2. Get action that have been updated.
/// 3. Notify Slack.
pub fn run() -> proc_exit::ExitResult {
    let cli_args = crate::cli_args::parse();

    let mut ret = (proc_exit::Code::SUCCESS, None);
    print!("{}", cli_args.subscribe_url);
    print!("{}", cli_args.notify_url);
    // let redmine = crate::redmine::new(cli_args.subscribe_url);
    // let slack = crate::slack::new(cli_args.notify_url);

    // let updated_entries = redmine.subscribe_updated()?;
    // for entry in updated_entries {
    //     slack.notify(entry)?;
    // }

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
