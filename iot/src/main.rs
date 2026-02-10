use std::time::Duration;

use rumqttc::{AsyncClient, Event, MqttOptions, Packet, QoS};
use tokio::{task, time};

#[tokio::main]
async fn main() {
    let mqtt_options = {
        let mut mqtt_options = MqttOptions::new("rumqtt-sync", "localhost", 1883);
        mqtt_options.set_keep_alive(Duration::from_secs(15));
        mqtt_options
    };

    let (client, mut event_pool) = AsyncClient::new(mqtt_options, 10);

    client
        .subscribe("traffic/test", QoS::AtMostOnce)
        .await
        .unwrap();

    for i in 0..=31 {
        let client = client.clone();
        let wait_time = 55 - ((i * 13) % 7);

        task::spawn(async move {
            loop {
                let avg_speed: u32 = ((i * 17) % 43) + 23;
                let data = vec![i as u8, avg_speed as u8 /* traffics... */];

                client
                    .publish("traffic/test", QoS::AtLeastOnce, false, data)
                    .await
                    .unwrap();

                time::sleep(Duration::from_secs(wait_time as u64)).await;
            }
        });
    }

    while let Ok(notification) = event_pool.poll().await {
        let Event::Incoming(Packet::Publish(data)) = notification else {
            continue;
        };

        println!("Data: {data:?}");
        println!("Payload: {:?}", data.payload.to_vec());
    }
}
