// Copyright ©️ ArkBig
/*!
# redmine-to-slack command
*/
fn main() {
    {
        use signal_hook::consts::{SIGHUP, SIGINT, SIGQUIT, SIGTERM};
        let mut signals = signal_hook::iterator::Signals::new(&[SIGHUP, SIGINT, SIGQUIT, SIGTERM])
            .expect("Error setting signal handler");
        std::thread::spawn(move || {
            for sig in signals.forever() {
                println!("Received signal {:?}", sig);
                proc_exit::exit(Err(proc_exit::Exit::new(proc_exit::Code::FAILURE)));
            }
        });
    }

    let res = redmine_to_slack_lib::app::run();
    proc_exit::exit(res);
}
