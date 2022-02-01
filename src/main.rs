// On Windows platform, don't show a console when opening the app.
#![windows_subsystem = "windows"]

use shadow_box::logic::app_data::AppData;
use shadow_box::logic::colors::generate_colors;
use shadow_box::ui::parent::make_ui;

use std::thread;

use druid::{AppLauncher, Color, WindowDesc};

pub fn main() {
    let window = WindowDesc::new(make_ui()).title("Shadow Boxing");

    let launcher = AppLauncher::with_window(window);

    // If we want to create commands from another thread `launcher.get_external_handle()`
    // should be used. For sending commands from within widgets you can always call
    // `ctx.submit_command`
    let event_sink = launcher.get_external_handle();
    // We create a new thread and generate colours in it.
    // This happens on a second thread so that we can run the UI in the
    // main thread. Generating some colours nicely follows the pattern for what
    // should be done like this: generating something over time
    // (like this or reacting to external events), or something that takes a
    // long time and shouldn't block main UI updates.
    thread::spawn(move || generate_colors(event_sink));
    let data = AppData {
        top_left: Color::rgb8(0, 0, 0),
        top_right: Color::rgb8(0, 0, 0),
        bottom: Color::rgb8(0, 0, 0),
    };

    launcher
        .log_to_console()
        .launch(data)
        .expect("launch failed");
}
