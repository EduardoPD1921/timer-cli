use std::io::stdin;
use std::env::Args;
use std::{thread, time::Duration};
use std::sync::mpsc;

use crate::Command;

const NO_TIMER_NAME_ERROR: &str = "No timer name was given.";

pub struct Timer;

impl Command for Timer {
    fn execute_command(&self, mut cli_args: Args) {
        let (status_sender, status_receiver) = mpsc::channel();
        let (seconds_sender, seconds_receiver) = mpsc::channel();

        thread::spawn(move || {
            let mut seconds = 0;
            let mut is_timer_active: bool = true;

            while is_timer_active {
                match status_receiver.try_recv() {
                    Ok(_) => {
                        println!("noice");
                        is_timer_active = false;

                        seconds_sender.send(seconds);
                    },
                    Err(_) => {}
                }

                seconds += 1;

                thread::sleep(Duration::from_millis(1000));
            }
        });

        let mut input_string = String::new();
        stdin().read_line(&mut input_string)
            .ok()
            .expect("msg");

        status_sender.send(true);

        let timer_seconds = seconds_receiver.recv().unwrap();

        println!("{} seconds", timer_seconds);
    }
}