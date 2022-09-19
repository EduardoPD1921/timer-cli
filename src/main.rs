mod actions;
mod traits;

use std::env;

use actions::create_timer::CreateTimer;
use actions::list_timers::ListTimers;
use actions::none::None;

use traits::command::Command;

fn main() {
    let args: Vec<String> = env::args().collect();

    let _file_path = &args[0];
    let cli_command = &args[1];
    let command_type = define_command_type_by_arg(cli_command);

    command_type.execute_command();
}

fn define_command_type_by_arg(command: &String) -> Box<dyn Command> {
    match command.as_str() {
        "create" => Box::new(CreateTimer),
        "list" => Box::new(ListTimers),
        _ => Box::new(None)
    }
}
