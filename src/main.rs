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
                let temperature = temperature.as_celsius();
                println!("Temperature: {:?}", temperature);
                client.gauge("sensehat.temperature", temperature, &tags).unwrap();
            }

            if let Ok(pressure) = hat.get_pressure() {
                let pressure = pressure.as_hectopascals();
                println!("Pressure: {:?}", pressure);
                client.gauge("sensehat.pressure", pressure, &tags).unwrap();
            }

            if let Ok(humidity) = hat.get_humidity() {
                let humidity = humidity.as_percent();
                println!("Humidity: {:?}", humidity);
                client.gauge("sensehat.humidity", humidity, &tags).unwrap();
            }

            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    });

    reader.join();
}
