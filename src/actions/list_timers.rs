use crate::Command;

pub struct ListTimers;

impl Command for ListTimers {
    fn execute_command(&self) {
        println!("ListTimer executed!");
    }
}