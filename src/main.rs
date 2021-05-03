struct Sensor {
    active: bool,
    latest: u32,
}

impl Sensor {
    fn new() -> Sensor {
        Sensor {
            active: false,
            latest: 0
        }
    }

    fn read(&self) -> u32 {
        self.latest
    }
    fn init(&mut self) {
        self.active = true;
        self.latest = 42;
    }
}


fn main() {
    let mut sensor = Sensor::new();
    sensor.init();
    println!("active = {}, latest ={}", sensor.active, sensor.read());
    sensor.active = false;
    sensor.latest = 32;
    println!("active = {}, latest ={}", sensor.active, sensor.read());
}
