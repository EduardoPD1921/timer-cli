use std::env::Args;

use crate::Command;

pub struct None;

impl Command for None {
    fn execute_command(&self, cli_args: Args) {
        println!("None executed!");
    }
}