use crate::Command;

pub struct CreateTimer;

impl Command for CreateTimer {
    fn execute_command(&self) {
        println!("CreateTimer executed!");
    }
}