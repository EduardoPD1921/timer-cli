use crate::Command;

pub struct None;

impl Command for None {
    fn execute_command(&self) {
        println!("None executed!");
    }
}