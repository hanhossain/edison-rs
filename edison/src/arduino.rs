pub use sysfs_gpio::Direction;
use sysfs_gpio::Pin;

pub fn say_hello() {
    println!("Hello!");
}

pub struct DigitalPin {
    pub pin: Pin,
    output_enable: OutputEnable,
    pullup_resistor: PullupResistor,
}

impl DigitalPin {
    pub fn new(pin_num: u8, tristate: &mut TristateBuffer, direction: Direction) -> Self {
        let gpio = Pin::new(match pin_num {
            7 => 48,
            8 => 49,
            _ => panic!("Invalid pin_num"),
        });
        export(&gpio);

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
            pin: gpio,
            output_enable,
            pullup_resistor,
        }
    }
}

impl Drop for DigitalPin {
    fn drop(&mut self) {
        unexport(&self.output_enable.pin);
        unexport(&self.pullup_resistor.pin);
        unexport(&self.pin);
    }
}

pub struct TristateBuffer {
    pin: Pin
}

impl TristateBuffer {
    pub fn new() -> Self {
        let pin = Pin::new(214);
        export(&pin);
        TristateBuffer { pin }
    }

    fn disconnect_shield_pins(&mut self) {
        self.pin.set_direction(Direction::Low).unwrap();
    }

    fn connect_shield_pins(&mut self) {
        self.pin.set_direction(Direction::High).unwrap();
    }
}

impl Drop for TristateBuffer {
    fn drop(&mut self) {
        unexport(&self.pin);
    }
}

struct PullupResistor {
    pin: Pin
}

impl PullupResistor {
    fn new(pin_num: u64) -> Self {
        let pin = Pin::new(pin_num);
        export(&pin);
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
        export(&pin);
        OutputEnable { pin }
    }

    fn set_input(&self) {
        self.pin.set_direction(Direction::Low).unwrap();
    }

    fn set_output(&self) {
        self.pin.set_direction(Direction::High).unwrap();
    }
}

fn export(pin: &Pin) {
    // TODO: add a verbosity flag
    // println!("Exporting gpio{}...", pin.get_pin_num());
    pin.export().unwrap();
}

fn unexport(pin: &Pin) {
    // TODO: add a verbosity flag
    // println!("Unexporting gpio{}...", pin.get_pin_num());
    let _ = pin.unexport();
}