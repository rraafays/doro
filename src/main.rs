#![allow(dead_code)]

fn main() {

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
  
}

fn get_operation(minutes: u8, seconds: u8) -> Operation {
  if seconds == 0 {  return Operation::Reset }
  if seconds == 0 && minutes == 0 { return Operation::Break }
  else { return Operation::None }
}

fn reset_seconds(mut time: Time) -> Time {
  time.seconds = 60;
  time.minutes -= 1;
  return time
}
