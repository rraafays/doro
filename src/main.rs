mod utils;

use std::thread::sleep;
use std::time::Duration;
use clap::Parser;
use utils::{ 
  Time, 
  Operation, 
  Mode, 
  print_time, 
  get_operation, 
  reset_seconds, 
  advance 
};

#[derive(Parser)]
struct Arguments {
  #[arg(short, long, default_value_t = 25)]
  work: u8,
  #[arg(short, long, default_value_t = 5)]
  rest: u8
}

fn main() {
  let arguments = Arguments::parse();
  loop {
    timer(Time { minutes: arguments.work, seconds: 0 }, Mode::Work);
    print!("\x07");
    timer(Time { minutes: arguments.rest, seconds: 0 }, Mode::Rest);
    print!("\x07");
  }
}

fn timer (time: Time, mode: Mode) {
  let mut remaining_time: Time = time;
  loop {
    print_time(&remaining_time, &mode);
    sleep(Duration::from_secs(1));
    match get_operation(&remaining_time) {
      Operation::Break => break,
      Operation::Reset => reset_seconds(&mut remaining_time),
      Operation::None => advance(&mut remaining_time)
    }
  }
}
