use std::{thread, time};
use edison::arduino::{DigitalPin, Direction, TristateBuffer};

fn main() {
    let mut tristate = TristateBuffer::new();
    
    let pin = DigitalPin::new(13, &mut tristate, Direction::Out);

    let delay = time::Duration::from_secs(1);

    loop {
        pin.pin.set_value(1).unwrap();
        thread::sleep(delay);
        
        pin.pin.set_value(0).unwrap();
        thread::sleep(delay);
    }
}
