use edison::arduino::{AnalogPin, TristateBuffer};

fn main() {
    let mut tristate = TristateBuffer::new();

    let temp_pin = AnalogPin::new(1, &mut tristate);
    
    // print raw value
    let raw = temp_pin.get_value();
    println!("raw: {}", raw);

    // print mV
    let volts = 5.0 / 4095.0 * raw as f64;
    println!("volts: {}", volts);

    let celsius = volts / 0.01;
    println!("celsius: {}", celsius);

    let fahrenheit = celsius * 1.8 + 32.0;
    println!("fahrenheit: {}", fahrenheit);
}
