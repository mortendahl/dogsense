extern crate sensehat;
extern crate dogstatsd;

use dogstatsd::Client;

fn main() {
    let mut hat = sensehat::SenseHat::new().unwrap();
    let client = Client::new().unwrap();
    let tags = ["hardware:pi3"];

    let reader = std::thread::spawn(move || {
        loop {

            if let Ok(temperature) = hat.get_temperature_from_pressure() {
                client.gauge("sensehat.temperature", temperature.as_celsius(), &tags).unwrap();
            }

            if let Ok(pressure) = hat.get_pressure() {
                client.gauge("sensehat.pressure", pressure.as_hectopascals(), &tags).unwrap();
            }

            if let Ok(humidity) = hat.get_humidity() {
                client.gauge("sensehat.humidity", humidity.as_percent(), &tags).unwrap();
            }

            std::thread::sleep(std::time::Duration::from_secs(5));
        }
    });

    reader.join();
}
