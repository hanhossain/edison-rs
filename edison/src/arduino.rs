pub use sysfs_gpio::Direction;
use sysfs_gpio::Pin;
use std::fs;

pub fn say_hello() {
    println!("Hello!");
}

pub struct DigitalPin {
    pub pin: Pin,
    output_enable: OutputEnable,
    pullup_resistor: PullupResistor,
    pin_mux: Option<PinMux>,
}

impl DigitalPin {
    pub fn new(pin_num: u8, tristate: &mut TristateBuffer, direction: Direction) -> Self {
        let gpio_pin_num = match pin_num {
            0 => 130,
            1 => 121,
            2 => 128,
            3 => 12,
            4 => 129,
            5 => 13,
            6 => 182,
            7 => 48,
            8 => 49,
            9 => 183,
            13 => 40,
            _ => panic!("Invalid pin_num"),
        };
        let gpio = Pin::new(gpio_pin_num);
        export(&gpio);

        let pullup_resistor = PullupResistor::new(match pin_num {
            0 => 216,
            1 => 217,
            2 => 218,
            3 => 219,
            4 => 220,
            5 => 221,
            6 => 222,
            7 => 223,
            8 => 224,
            9 => 225,
            13 => 229,
            _ => panic!("Invalid pin_num"),
        });

        let output_enable = OutputEnable::new(match pin_num {
            0 => 248,
            1 => 249,
            2 => 250,
            3 => 251,
            4 => 252,
            5 => 253,
            6 => 254,
            7 => 255,
            8 => 256,
            9 => 257,
            13 => 261,
            _ => panic!("Invalid pin_num"),
        });

        let pin_mux_num: Option<u64> = match pin_num {
            13 => Some(243),
            _ => None,
        };

        tristate.disconnect_shield_pins();
        
        let pin_mux = match pin_mux_num {
            Some(x) => {
                let a = PinMux::new(x);
                a.pin.set_direction(Direction::Low).unwrap();
                Some(a)
            },
            _ => None,
        };

        // set input or output
        match direction {
            Direction::In => output_enable.set_input(),
            _ => output_enable.set_output(),
        };

        pullup_resistor.disable();

        set_mode(gpio_pin_num);
        gpio.set_direction(direction).unwrap();

        tristate.connect_shield_pins();

        DigitalPin {
            pin: gpio,
            output_enable,
            pullup_resistor,
            pin_mux,
        }
    }
}

impl Drop for DigitalPin {
    fn drop(&mut self) {
        match &self.pin_mux {
            Some(x) => unexport(&x.pin),
            _ => (),
        };
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

struct PinMux {
    pin: Pin
}

impl PinMux {
    fn new(pin_num: u64) -> Self {
        let pin = Pin::new(pin_num);
        export(&pin);
        PinMux { pin }
    }
}

fn export(pin: &Pin) {
    pin.export().unwrap();
}

fn unexport(pin: &Pin) {
    let _ = pin.unexport();
}

fn set_mode(pin_num: u64) {
    let path = format!("/sys/kernel/debug/gpio_debug/gpio{}/current_pinmux", pin_num);
    fs::write(path, "mode0").unwrap();
}