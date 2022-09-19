use std::env::Args;

pub trait Command {
    fn execute_command(&self, cli_args: Args);
}