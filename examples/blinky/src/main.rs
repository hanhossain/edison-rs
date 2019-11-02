use std::{thread, time};
use edison::arduino::{DigitalPin, Direction, TristateBuffer};

fn main() {
    let mut tristate = TristateBuffer::new();
    
    let d7 = DigitalPin::new(7, &mut tristate, Direction::Out);

    let delay = time::Duration::from_secs(2);

    loop {
        d7.pin.set_value(1).unwrap();
        thread::sleep(delay);
        
        d7.pin.set_value(0).unwrap();
        thread::sleep(delay);
    }
}
