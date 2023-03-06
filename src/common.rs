use druid::{Data, Lens};

#[derive(Clone, Data, Lens)]
pub struct ClientData {
    mqtt_hostname: String,
    mqtt_port: i32,
    mqtt_user: String,
    mqtt_pwd: String,
    udp_port: i32,
}

impl ClientData {
    pub fn new(mqtt_hostname: String, mqtt_port: i32, udp_port: i32) -> Self {
        ClientData {
            mqtt_hostname,
            mqtt_port,
            mqtt_user: String::new(),
            mqtt_pwd: String::new(),
            udp_port
        }
    }
}