use edison::arduino::{DigitalPin, Direction, TristateBuffer};

fn main() {
    let mut tristate = TristateBuffer::new();
    
    let d7 = DigitalPin::new(7, &mut tristate, Direction::In);
    println!("D7 value: {}", d7.pin.get_value().unwrap());
}
