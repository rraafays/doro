#![allow(dead_code)]

use std::thread::sleep;
use std::time::Duration;

fn main() {
  timer(Time { minutes: 0, seconds: 30 }, String::from("test"));
}

enum Operation {
  Reset,
  Break,
  None
}

struct Time {
  minutes: u8,
  seconds: u8
}

fn timer (time: Time, message: String) {
  let mut remaining_time: Time = time;
  
  loop {
    println!("{}:{}", remaining_time.minutes, remaining_time.seconds);
    match get_operation(&remaining_time) {
      Operation::Break => println!("break!"),
      Operation::Reset => reset_seconds(&mut remaining_time),
      Operation::None => remaining_time.seconds -= 1
    }
    sleep(Duration::from_secs(1));
  }
}

fn get_operation(time: &Time) -> Operation {
  if time.seconds == 0 {  return Operation::Reset }
  if time.seconds == 0 && time.minutes == 0 { return Operation::Break }
  else { return Operation::None }
}

fn reset_seconds(mut time: &mut Time) {
  time.seconds = 60;
  // time.minutes -= 1;
}
