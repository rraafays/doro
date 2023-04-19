use std::thread::sleep;
use std::time::Duration;
use std::io::Write;
use clap::Parser;
use colored::Colorize;

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
    timer(Time { minutes: arguments.rest, seconds: 0 }, Mode::Rest);
  }
}

enum Operation {
  Reset,
  Break,
  None
}

enum Mode {
  Work,
  Rest
}

struct Time {
  minutes: u8,
  seconds: u8
}

fn timer (time: Time, mode: Mode) {
  let mut remaining_time: Time = time;
  loop {
    print_time(&remaining_time, &mode);
    sleep(Duration::from_secs(1));
    match get_operation(&remaining_time) {
      Operation::Break => break,
      Operation::Reset => reset_seconds(&mut remaining_time),
      Operation::None => remaining_time.seconds -= 1
    }
  }
}

fn get_operation(time: &Time) -> Operation {
  if time.seconds == 0 && time.minutes == 0 { return Operation::Break }
  if time.seconds == 0 {  return Operation::Reset }
  else { return Operation::None }
}

fn reset_seconds(mut time: &mut Time) {
  time.seconds = 59;
  if time.minutes > 0 { time.minutes -= 1; }
}

fn print_time(time: &Time, mode: &Mode) {
  let symbol;
  match mode {
    Mode::Work => symbol = "WORK ".bright_red().bold(),
    Mode::Rest => symbol = "REST ".bright_blue().bold()
  }
  print!("\r{}{:0>2}:{:0>2}", symbol, time.minutes, time.seconds);
  std::io::stdout().flush().unwrap();
}
