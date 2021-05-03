trait Sensor {
    fn read(&self) -> u32;
}

struct LightSensor {
    value: u32,
}

impl Sensor for LightSensor {
    fn read(&self) -> u32 {
        self.value
     }
}

struct TemperatureSensor {
    value: f32
}

impl Sensor for TemperatureSensor {

    fn read(&self) -> u32 {
        self.value as u32
    }
}

fn print_sensor_value(sensor: impl Sensor) {
    println!("sensor value {}", sensor.read());
}

fn main() {
    let ls = LightSensor {
        value: 1
    };
    let ts = TemperatureSensor {
        value: 3.5
    };
    print_sensor_value(ls);
    print_sensor_value(ts);

}