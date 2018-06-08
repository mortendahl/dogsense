extern crate sensehat;
extern crate dogstatsd;

use dogstatsd::Client;

fn main() {

  let mut hat = sensehat::SenseHat::new().unwrap();
  let client = Client::new().unwrap();
  let tags = ["hardware:pi3"];

  let reader = std::thread::spawn(move || {
    loop {
      let temperature = hat.get_temperature_from_pressure();
      //let temperature = hat.get_orientation();
      if let Ok(temperature) = temperature {
         println!("{}", temperature.as_celsius());
         client.gauge("sensehat.temperature", temperature.as_celsius(), &tags).unwrap();
      //println!("{:?}", temperature);      
}
    std::thread::sleep(std::time::Duration::from_millis(1000)); 
  }});

  reader.join();
}
