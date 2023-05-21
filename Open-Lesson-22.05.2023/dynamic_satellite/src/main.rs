//! # Задание:
//!
//! На открытом уроке мы написали спутник, в который можно динамически добавлять новые устройства,
//! однако мы не сильно пользовались *динамичностью*, т.к. при первой же поломке наш спутник
//! прекращает работу.
//!
//! В этом задании вам предлагается исправить это и сделать так, чтобы при неисправности устройство
//! удалялось из спутника, а он сам продолжал лететь до бесконечности.
//!
//! # Как запустить?
//!
//! Для запуска используйте команду `cargo run` из директории `dynamic_satellite`.
//!
//! Как установить `cargo` можно прочитать в
//! [официальной документации](https://doc.rust-lang.org/cargo/getting-started/installation.html).

use std::{collections::HashMap, time::Duration};

use rand::Rng as _;

struct Satellite {
    devices: HashMap<String, Box<dyn Device>>,
}

trait Device {
    fn is_working(&self) -> bool;
}

impl Satellite {
    pub fn new() -> Satellite {
        Satellite {
            devices: HashMap::new(),
        }
    }

    pub fn add_device(&mut self, name: impl Into<String>, device: impl Device + 'static) {
        self.devices.insert(name.into(), Box::new(device));
    }

    pub fn run(&self) {
        loop {
            println!("Проверка устройств...");

            for (name, device) in &self.devices {
                if device.is_working() {
                    println!("  {name} работает");
                } else {
                    println!("  {name} не работает, завершение миссии!");
                    return;
                }
            }

            println!("Спим 1 секунду...\n");
            std::thread::sleep(Duration::from_secs(1));
        }
    }
}

struct Camera;

impl Device for Camera {
    fn is_working(&self) -> bool {
        // Работает с вероятностью 70%
        rand::thread_rng().gen_bool(0.7)
    }
}

struct Engine;

impl Device for Engine {
    fn is_working(&self) -> bool {
        // Работает с вероятностью 80%
        rand::thread_rng().gen_bool(0.8)
    }
}

fn main() {
    let mut satellite = Satellite::new();
    satellite.add_device("Камера", Camera {});
    satellite.add_device("Двигатель", Engine {});

    satellite.run()
}
