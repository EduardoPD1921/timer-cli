use std::io::Write;
use std::path::Path;
use std::fs::{File, OpenOptions};
use std::env::Args;

use crate::Command;

const FILE_PATH: &str = "timers.txt";
const NO_TIMER_NAME_ERROR: &str = "No name was given for the timer.";
const CANT_WRITE_INTO_FILE_ERROR: &str = "Couldn't write into the store file.";
const CANT_CREATE_FILE_ERROR: &str = "Couldn't create the store file.";
const CANT_OPEN_THE_FILE_ERROR: &str = "Couldn't open the file.";
pub struct CreateTimer;

impl Command for CreateTimer {
    fn execute_command(&self, mut args: Args) {
        let timer_name = args.next().expect(NO_TIMER_NAME_ERROR);
        write_into_file(timer_name);
        println!("Timer created!");
    }
}

fn write_into_file(timer_name: String) {
    let mut file: File;
    let timer_name_formatted: String;

    let path_exists = Path::new(FILE_PATH).exists();
    if path_exists {
        timer_name_formatted = format!("\n{}", timer_name);
    } else {
        File::create(FILE_PATH).expect(CANT_CREATE_FILE_ERROR);
        timer_name_formatted = timer_name; 
    }

    file = open_file_with_append_option();
    file.write_all(timer_name_formatted.as_bytes()).expect(CANT_WRITE_INTO_FILE_ERROR);
}

fn open_file_with_append_option() -> File {
    OpenOptions::new()
        .write(true)
        .append(true)
        .open(FILE_PATH)
        .expect(CANT_OPEN_THE_FILE_ERROR)
}