use std::io::{stdin, stdout};
use std::env::Args;
use std::{thread, time::Duration};
use std::sync::mpsc::{self, Sender, Receiver};

use crossterm::{
    style::Stylize,
    terminal::{Clear, ClearType},
    execute,
    cursor::MoveUp
};

use crate::Command;

const NO_TIMER_NAME_ERROR: &str = "No timer name was given.";

pub struct Timer;

impl Command for Timer {
    fn execute_command(&self, mut cli_args: Args) {
        let timer_name = cli_args.next().expect(NO_TIMER_NAME_ERROR);

        let (status_sender, status_receiver) = mpsc::channel();
        let (time_sender, time_receiver) = mpsc::channel();

        thread::spawn(move || {
            let mut seconds: u32 = 0;
            let mut is_timer_active: bool = true;

            while is_timer_active {
                match status_receiver.try_recv() {
                    Ok(_) => {
                        is_timer_active = false;
                        time_sender.send(seconds);
                    },
                    Err(_) => {}
                }

                seconds += 1;

                thread::sleep(Duration::from_millis(1000));
            }
        });

        println!("{}", "Timer running...".cyan());
        println!("{}", "to stop type 'stop'".dark_grey());

        await_command(status_sender, time_receiver);
    }
}

fn await_command(status_sender: Sender<bool>, time_receiver: Receiver<u32>) {
    let mut input_command = String::new();
        stdin().read_line(&mut input_command)
        .ok()
        .expect("msg");

    clean_input_command(&mut input_command);
    match input_command.as_str() {
        "stop" => {
            println!("{}", "Stopping timer...".cyan());
            status_sender.send(true);
            let seconds = time_receiver.recv().unwrap();

            move_cursor_up_and_clear();
            println!("{} {} seconds", "Elapsed time:".cyan(), seconds);
        },
        _ => {
            move_cursor_up_and_clear();

            println!("{} {}", input_command, "it's not a known command".red());
            await_command(status_sender, time_receiver);
        }
    }
}

fn clean_input_command(command: &mut String) {
    if command.ends_with('\n') {
        command.pop();

        if command.ends_with('\r') {
            command.pop();
        }
    }
}

fn move_cursor_up_and_clear() {
    execute!(
        stdout(),
        MoveUp(1),
        Clear(ClearType::CurrentLine)
    );
}