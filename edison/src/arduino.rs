pub use sysfs_gpio::Direction;
use sysfs_gpio::Pin;

pub fn say_hello() {
    println!("Hello!");
}

pub struct DigitalPin {
    pub pin: Pin
}

impl DigitalPin {
    pub fn new(pin_num: u8, tristate: &mut TristateBuffer, direction: Direction) -> Self {
        let gpio = Pin::new(match pin_num {
            7 => 48,
            8 => 49,
            _ => panic!("Invalid pin_num"),
        });
        gpio.export().unwrap();

        let pullup_resistor = PullupResistor::new(match pin_num {
            7 => 223,
            8 => 224,
            _ => panic!("Invalid pin_num"),
        });

        let output_enable = OutputEnable::new(match pin_num {
            7 => 255,
            8 => 256,
            _ => panic!("Invalid pin_num"),
        });

        tristate.disconnect_shield_pins();

        output_enable.set_input();
        pullup_resistor.disable();
        gpio.set_direction(direction).unwrap();

        tristate.connect_shield_pins();

        DigitalPin {
            pin: gpio
        }
    }
}

pub struct TristateBuffer {
    pin: Pin
}

impl TristateBuffer {
    pub fn new() -> Self {
        let pin = Pin::new(214);
        pin.export().unwrap();
        TristateBuffer { pin }
    }

    fn disconnect_shield_pins(&mut self) {
        self.pin.set_direction(Direction::Low).unwrap();
    }

    fn connect_shield_pins(&mut self) {
        self.pin.set_direction(Direction::High).unwrap();
    }
}

struct PullupResistor {
    pin: Pin
}

impl PullupResistor {
    fn new(pin_num: u64) -> Self {
        let pin = Pin::new(pin_num);
        pin.export().unwrap();
        PullupResistor { pin }
    }

    fn disable(&self) {
        self.pin.set_direction(Direction::Low).unwrap();
    }
}

// TODO: possibly rename to DirectionController
struct OutputEnable {
    pin: Pin
}

impl OutputEnable {
    fn new(pin_num: u64) -> Self {
        let pin = Pin::new(pin_num);
        pin.export().unwrap();
        OutputEnable { pin }
    }

    fn set_input(&self) {
        self.pin.set_direction(Direction::Low).unwrap();
    }

    fn set_output(&self) {
        self.pin.set_direction(Direction::High).unwrap();
    }
}