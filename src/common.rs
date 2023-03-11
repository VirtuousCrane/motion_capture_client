use druid::{Data, Lens};
use serde::{Serialize, Deserialize};

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

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum JsonData {
    MPU6050Data {
        source: String,
        acc_x: f64,
        acc_y: f64,
        acc_z: f64,
        rot_x: f64,
        rot_y: f64,
        rot_z: f64,
    },
    UWBData {
        source: String,
        range: f64,
    }
}