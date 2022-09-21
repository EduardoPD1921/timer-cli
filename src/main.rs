#[allow(unused)]

mod actions;
mod traits;

use std::{
    env,
    path::Path,
    fs::{self, File}
};

use actions::{create_timer::CreateTimer, list_timers::ListTimers, timer::Timer, none::None};
use traits::command::Command;

// TODO:
// - colocar todas as constantes em um único arquivo de constantes
// - criar uma estrutura melhor dos dados armazenados em timers.txt usando o struct de Timer
// - tratar os unwraps sobrando
// - analisar se faz sentido isolar as operações de arquivo em algum struct
// - testar precisão do timer

const FILE_PATH: &str = "timers.txt";
const CANT_CREATE_FILE_ERROR: &str = "Couldn't create the store file.";
const CANT_OPEN_THE_FILE_ERROR: &str = "Couldn't open the file.";

fn main() {
    let mut args = env::args();
    args.next();

    let main_command = args.next().expect("You need to pass a main command");
    let command_type = get_command_type_by_arg(main_command);

    initialize_timers_file();
    let mut timers_as_vec: Vec<(String, u32)> = Vec::new();

    let file_buf = fs::read_to_string(FILE_PATH).expect(CANT_OPEN_THE_FILE_ERROR);
    for line in file_buf.lines() {
        let mut chunk = line.split(';');

        let timer_name = chunk.next().unwrap().to_owned();
        let timer_seconds = chunk.next().unwrap().parse::<u32>().unwrap();

        timers_as_vec.push((timer_name, timer_seconds));
    }

    println!("{:?}", timers_as_vec);
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

fn initialize_timers_file() {
    let path_exists = Path::new(FILE_PATH).exists();
    if !path_exists {
        File::create(FILE_PATH).expect(CANT_CREATE_FILE_ERROR);
    }
}