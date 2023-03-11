use std::{thread, str, net::UdpSocket, sync::mpsc};

use crate::common::{ClientData, JsonData};
use druid::{Widget, widget::{Container, Label, Flex, LensWrap, TextBox, Align, Button}, text::format::ParseFormatter, WidgetExt, EventCtx, Env};
use log::{info, warn};
use paho_mqtt::{CreateOptionsBuilder, Client, ConnectOptionsBuilder, Message};
use uuid::Uuid;

/// Creates the ui of the program
pub fn build_ui() -> impl Widget<ClientData> {
    let mqtt_data_input_row = Flex::row()
        .with_child(Label::new("MQTT Hostname: "))
        .with_child(LensWrap::new(
            TextBox::new(),
            ClientData::mqtt_hostname
        ))
        .with_child(Label::new("Port: "))
        .with_child(LensWrap::new(
            TextBox::new()
                .with_formatter(ParseFormatter::new()),
            ClientData::mqtt_port
        ));
    
    let mqtt_user_pwd = Flex::row()
        .with_child(Label::new("MQTT Username: "))
        .with_child(LensWrap::new(
            TextBox::new(),
            ClientData::mqtt_user
        ))
        .with_child(Label::new("Password: "))
        .with_child(LensWrap::new(
            TextBox::new(),
            ClientData::mqtt_pwd
        ));
    
    let udp_data_input_row = Flex::row()
        .with_child(Label::new("UDP Port: "))
        .with_child(LensWrap::new(
            TextBox::new()
                .with_formatter(ParseFormatter::new()),
            ClientData::udp_port
        ))
        .with_spacer(10.0)
        .with_child(
            Button::new("Connect")
                .on_click(button_callback)
        );
    
    Container::new(
        Flex::column()
            .with_child(mqtt_data_input_row)
            .with_child(mqtt_user_pwd)
            .with_child(udp_data_input_row)
            .center()
    )
}

/// Callback for the connect button. Creates a new thread to listen to UDP messages
fn button_callback(_ctx: &mut EventCtx, data: &mut ClientData, _env: &Env) {
    // Initialize UDP Socket
    let socket = match UdpSocket::bind(format!("0.0.0.0:{}", data.udp_port)) {
        Ok(s) => s,
        Err(err) => {
            warn!("Failed to bind UDP Port: {}", err.to_string());
            return;
        }
    };
    info!("Bound to UDP Port: {}", data.udp_port);
    
    // Initialize MQTT Connection
    let mqtt_client = match init_mqtt_client(&data) {
        Some(client) => client,
        None => return,
    };
    info!("Initialized MQTT Client");
    
    // Initialize Message Passing Channel
    let (tx, rx) = mpsc::channel();
    
    let _udp_thread_handle = thread::spawn(move || {
        loop {
            let mut buf = [0; 512];
            if let Err(err) = socket.recv(&mut buf) {
                warn!("Failed to receive UDP message: {}", err.to_string());
            }
            
            let json_str = match str::from_utf8(&buf) {
                Ok(res) => {
                    info!("Received: {}", res.trim());
                    let result = res.trim_matches(char::from(0));
                    result.trim()
                },
                Err(e) => {
                    warn!("{}", e.to_string());
                    continue;
                },
            };
            
            match serde_json::from_str::<JsonData>(json_str) {
                Ok(obj) => {                    
                    if let Err(err) = tx.send(obj) {
                        warn!("Failed to pass message: {}", err.to_string());
                    }
                },
                Err(e) => {
                    warn!("Failed to parse String into JsonData: {}", e);
                    continue;
                },
            };
        }
    });
    
    let _mqtt_thread_handle = thread::spawn(move || {
        loop {
            let payload = match rx.recv() {
                Ok(msg) => msg,
                Err(_) => continue,
            };
            
            let topic = match payload {
                JsonData::MPU6050Data { .. } => {
                    "MOTIONCAPTURE/MPU6050"
                },
                JsonData::UWBData { .. } => {
                    "MOTIONCAPTURE/UWB"
                }
            };
            
            let payload_str = match serde_json::to_string(&payload) {
                Ok(s) => s,
                Err(_) => {
                    warn!("Failed to parse Payload into String");
                    return;
                },
            };
            
            let message = Message::new(topic, payload_str, 0);
            if let Err(err) = mqtt_client.publish(message) {
                warn!("Failed to publish message: {}", err.to_string());
            }
        }
    });
}

fn init_mqtt_client(data: &ClientData) -> Option<Client> {
    // Initializing the Connection Arguments
    let host_uri = format!("tcp://{}:{}", &data.mqtt_hostname, &data.mqtt_port);
    let mqtt_client_args = CreateOptionsBuilder::new()
        .server_uri(&host_uri)
        .client_id(Uuid::new_v4().to_string())
        .finalize();
    let mqtt_connect_args = ConnectOptionsBuilder::new()
        .user_name(&data.mqtt_user)
        .password(&data.mqtt_pwd)
        .finalize();
    
    // Initializes the MQTT Client
    let mqtt_client = match Client::new(mqtt_client_args) {
        Ok(client) => client,
        Err(err) => {
            warn!("Failed to create MQTT client: {}", err.to_string());
            return None;
        }
    };
    
    // Tries to Establish Connection to the MQTT Server
    if let Err(err) = mqtt_client.connect(mqtt_connect_args) {
        warn!("Failed to connect to MQTT server: {}", err.to_string());
        return None;
    }
    
    Some(mqtt_client)
}