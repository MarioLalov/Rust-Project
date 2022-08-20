mod memory_tape;

use crate::memory_tape::*;
use std::{thread, time::Duration};
use std::io::stdout;
use std::io::Write;

fn main() {
    let mut tape = MemoryTape::new();
    //tape.move_to(1);
    //tape.print_tape_sniplet();

    for i in 0..100{
    std::process::Command::new("cmd")
    .args(&["/c", "cls"])
    .spawn()
    .expect("cls command failed to start")
    .wait()
    .expect("failed to wait");

    tape.print_tape_sniplet();
    tape.move_right();

    thread::sleep(Duration::from_millis(1500));
    }
}
