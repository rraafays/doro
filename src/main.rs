use std::thread::sleep;
use std::time::Duration;
use std::io::Write;

fn main() {
  loop {
    timer(Time { minutes: 25, seconds: 0 });
    timer(Time { minutes: 5, seconds: 0 });
  }
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

fn timer (time: Time) {
  let mut remaining_time: Time = time;

  loop {
    print_time(&remaining_time);
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

fn print_time(time: &Time) {
  print!("\r{:0>2}:{:0>2}", time.minutes, time.seconds);
  std::io::stdout().flush().unwrap();
}
