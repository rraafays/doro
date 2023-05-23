use colored::Colorize;
use std::io::Write;

pub enum Operation {
    Reset,
    Break,
    None,
}

pub enum Mode {
    Work,
    Rest,
}

pub struct Time {
    pub minutes: u8,
    pub seconds: u8,
}

pub fn get_operation(time: &Time) -> Operation {
    if time.seconds == 0 && time.minutes == 0 {
        return Operation::Break;
    }
    if time.seconds == 0 {
        return Operation::Reset;
    } else {
        return Operation::None;
    }
}

pub fn reset_seconds(time: &mut Time) {
    time.seconds = 59;
    if time.minutes > 0 {
        time.minutes -= 1;
    }
}

pub fn advance(time: &mut Time) {
    time.seconds -= 1;
}

pub fn print_time(time: &Time, mode: &Mode) {
    let symbol;
    match mode {
        Mode::Work => symbol = "WORK ".bright_red().bold(),
        Mode::Rest => symbol = "REST ".bright_blue().bold(),
    }
    print!("\r{}{:0>2}:{:0>2}", symbol, time.minutes, time.seconds);
    std::io::stdout().flush().unwrap();
}
