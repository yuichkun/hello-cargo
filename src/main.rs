
struct Sensor {
    active: bool,
    latest: u32
}

struct SensorFusion {
    temperature: Sensor,
    light: Sensor,
}

fn main() {
    let sensors = SensorFusion {
        temperature: Sensor {
            active: false,
            latest: 0
        },
        light:  Sensor {
            active: true,
            latest: 3
        }
    };
    let _l = sensors.light;
    // println!("sensors.light.latest {}", sensors.light.latest);
    println!("_l.latest {}", _l.latest);
    println!("sensors.temperature.latest {}", sensors.temperature.latest);

}