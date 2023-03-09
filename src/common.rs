use druid::{Data, Lens};

#[derive(Clone, Data, Lens)]
pub struct ClientData {
    pub mqtt_hostname: String,
    pub mqtt_port: i32,
    pub mqtt_user: String,
    pub mqtt_pwd: String,
    pub udp_port: i32,
}

impl ClientData {
    pub fn new(mqtt_hostname: String, mqtt_port: i32, udp_port: i32) -> Self {
        ClientData {
            mqtt_hostname,
            mqtt_port,
            mqtt_user: String::new(),
            mqtt_pwd: String::new(),
            udp_port,
        }
    }
}