use std::time::Duration;
use tokio::time;

use thermo::thermometer::Thermometer;

#[tokio::main]
async fn main() {
    let receiver_address = "127.0.0.1:4321";
    let thermo = Thermometer::new(receiver_address).await.unwrap();
    for _ in 0..120 {
        time::sleep(Duration::from_secs(1)).await;
        let temperature = thermo.get_temperature().await;
        println!("The temperature is {temperature}");
    }
}
