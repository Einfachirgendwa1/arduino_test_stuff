use std::{fmt::Display, sync::Mutex, thread::sleep, time::Duration};

use colored::Colorize;

#[derive(Clone, Copy)]
struct Pin {
    mode: Option<PinMode>,
    value: PinValue,
}

impl Pin {
    const fn default() -> Self {
        Self {
            mode: None,
            value: PinValue::Low,
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
enum PinMode {
    Input,
    Output,
}

impl Display for PinMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PinMode::Input => write!(f, "{}", "Input ".blue()),
            PinMode::Output => write!(f, "{}", "Output".red()),
        }
    }
}

#[derive(Clone, Copy)]
enum PinValue {
    Low,
    High,
}

impl Display for PinValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PinValue::High => write!(f, "{}", "High".green()),
            PinValue::Low => write!(f, "{}", "Low".yellow()),
        }
    }
}

static PINS: Mutex<[Pin; 14]> = Mutex::new([Pin::default(); 14]);

fn pin_mode(pin_id: usize, mode: PinMode) {
    PINS.lock().unwrap()[pin_id].mode = Some(mode);
}

fn digital_write(pin_id: usize, value: PinValue) {
    let pin = &mut PINS.lock().unwrap()[pin_id];
    println!(
        "{} pin {} changed {} -> {value}",
        pin.mode.unwrap(),
        pin_id.to_string().purple(),
        pin.value
    );
    pin.value = value;
}

trait Iterate<A> {
    fn iterate_over_self(self) -> impl Iterator<Item = A>;
}

impl<A> Iterate<A> for (A, A) {
    fn iterate_over_self(self) -> impl Iterator<Item = A> {
        [self.0, self.1].into_iter()
    }
}

fn right_padd(string: String, length: i64, seperator: char) -> String {
    let mut len = length - string.len() as i64;
    if len < 0 {
        len = 0;
    }

    format!(
        "{string}{}",
        String::from_utf8(vec![seperator as u8; len as usize]).unwrap()
    )
}

fn report() {
    PINS.lock()
        .unwrap()
        .iter()
        .enumerate()
        .filter_map(|(index, pin)| Some((index.to_string().purple(), pin.value, pin.mode?)))
        .partition::<Vec<_>, _>(|(_, _, mode)| *mode == PinMode::Output)
        .iterate_over_self()
        .flatten()
        .for_each(|(index, value, mode)| {
            let l_value = right_padd(format!("{mode} pin {index} "), 30, ' ');
            println!("{l_value} = {value}",)
        });

    println!();
}

fn main() {
    pin_mode(1, PinMode::Output);
    pin_mode(2, PinMode::Output);
    pin_mode(3, PinMode::Output);

    loop {
        digital_write(1, PinValue::High);
        report();
        sleep(Duration::from_secs(1));
        digital_write(1, PinValue::Low);
        digital_write(2, PinValue::High);
        report();
        sleep(Duration::from_secs(1));
        digital_write(2, PinValue::Low);
        digital_write(3, PinValue::High);
        report();
        sleep(Duration::from_secs(1));
        digital_write(3, PinValue::Low);
    }
}
