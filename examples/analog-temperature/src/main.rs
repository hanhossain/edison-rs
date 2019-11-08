use edison::arduino::{AnalogPin, TristateBuffer};
use std::thread;
use std::time::Duration;

fn main() {
    let mut tristate = TristateBuffer::new();

    let temp_pin = AnalogPin::new(1, false, &mut tristate);

    let delay = Duration::from_secs(1);
    
    let mut temperatures: Vec<f64> = vec![];

    // initialize vector
    for _ in 0..10 {
        temperatures.push(get_celsius(&temp_pin));
    }

    loop {
        for x in 0..10 {
            temperatures[x] = get_celsius(&temp_pin);
            let avg_c = get_average(&temperatures);
            let avg_f = to_fahrenheit(avg_c);
            println!("temperature: {:.1}", avg_f);
            thread::sleep(delay);
        }
    }
}

fn to_fahrenheit(celsius: f64) -> f64 {
    celsius * 1.8 + 32.0
}

fn get_celsius(pin: &AnalogPin) -> f64 {
    let raw = pin.get_value();
    let volts = 5.0 / 4095.0 * raw as f64;
    volts / 0.01
}

fn get_average(array: &[f64]) -> f64 {
    array.iter().sum::<f64>() / array.len() as f64
}