use edison::arduino::{DigitalPin, Direction, TristateBuffer};

fn main() {
    let mut tristate = TristateBuffer::new();
    
    let d7 = DigitalPin::new(7, &mut tristate, Direction::In);
    println!("D7 value: {}", d7.pin.get_value().unwrap());

    let d8 = DigitalPin::new(8, &mut tristate, Direction::In);
    println!("D8 value: {}", d8.pin.get_value().unwrap());
}
