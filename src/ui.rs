use crate::common::ClientData;
use druid::{Widget, widget::{Container, Label, Flex, LensWrap, TextBox, Align, Button}, text::format::ParseFormatter, WidgetExt, EventCtx, Env};

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

fn button_callback(_ctx: &mut EventCtx, data: &mut ClientData, _env: &Env) {
    println!("Hello, World!");
}