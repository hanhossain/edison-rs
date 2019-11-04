use edison::arduino::{AnalogPin, TristateBuffer};

fn main() {
    let mut tristate = TristateBuffer::new();

    let a3 = AnalogPin::new(3, &mut tristate);
    let a4 = AnalogPin::new(4, &mut tristate);
    let a5 = AnalogPin::new(5, &mut tristate);

    println!("A3: {}", a3.get_value());
    println!("A4: {}", a4.get_value());
    println!("A5: {}", a5.get_value());
}
