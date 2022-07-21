use std::thread;
use std::time::Duration;

pub fn run_on_app_start() {
    nslogln!("App started!");
    thread::sleep(Duration::from_millis(1000));
    nslogln!("App done init!");
}

pub fn run_on_button_press() {
    nslogln!("Running payload!");
    thread::sleep(Duration::from_millis(1000));
    nslogln!("Done running payload!");
}