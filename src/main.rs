use common::ClientData;
use druid::{WindowDesc, AppLauncher};

mod ui;
mod common;

fn main() {
    let main_window = WindowDesc::new(ui::build_ui)
        .window_size((600.0, 400.0))
        .title("Motion Capture Client Program");
    
    let initial_data = ClientData::new(
        String::new(),
        1883,
        8888
    );
    
    AppLauncher::with_window(main_window)
        .launch(initial_data)
        .expect("Failed to launch program");
}    
