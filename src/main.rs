#[allow(unused)]

mod actions;
mod traits;

use std::env;

use actions::{create_timer::CreateTimer, list_timers::ListTimers, timer::Timer, none::None};
use traits::command::Command;

fn main() {
    let mut args = env::args();
    args.next();

    let main_command = args.next().expect("You need to pass a main command");
    let command_type = get_command_type_by_arg(main_command);

    command_type.execute_command(args);
}

fn get_command_type_by_arg(command: String) -> Box<dyn Command> {
    match command.as_str() {
        "create" => Box::new(CreateTimer),
        "list" => Box::new(ListTimers),
        "play" => Box::new(Timer),
        _ => Box::new(None)
    }
}
