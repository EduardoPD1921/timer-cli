use std::env::Args;

use crate::Command;

pub struct ListTimers;

impl Command for ListTimers {
    fn execute_command(&self, cli_args: Args) {
        println!("ListTimer executed!");
    }
}