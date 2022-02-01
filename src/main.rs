// On Windows platform, don't show a console when opening the app.
#![windows_subsystem = "windows"]

use rand::Rng;
use std::thread;
use std::time::Duration;

use druid::{
    widget::{Flex, Painter},
    Data, Lens,
};
use druid::{AppLauncher, Color, RenderContext, Widget, WindowDesc};

#[derive(Clone, Data, Lens)]
struct AppData {
    top_left: Color,
    top_right: Color,
    bottom: Color,
}

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

fn generate_colors(event_sink: druid::ExtEventSink) {
    // This function is called in a separate thread, and runs until the program ends.
    // We take an `ExtEventSink` as an argument, we can use this event sink to send
    // commands to the main thread. Every time we generate a new colour we send it
    // to the main thread.

    let mut rng = rand::thread_rng();
    let mut left_color = Color::BLACK;
    let mut right_color = Color::BLACK;
    let mut bottom_color = Color::BLACK;

    loop {
        let r = rng.gen_range(0, 2);
        let target;
        match r {
            0 => target = Color::RED,
            1 => target = Color::BLUE,
            _ => target = Color::GREEN,
        }

        reset(&mut left_color, &mut right_color, &mut bottom_color);

        let r = rng.gen_range(0, 3);
        if r == 0 {
            left_color = target;
        } else if r == 1 {
            right_color = target;
        } else {
            bottom_color = target;
        }

        let data_clone = AppData {
            top_left: left_color.clone(),
            top_right: right_color.clone(),
            bottom: bottom_color.clone(),
        };
        // schedule idle callback to change the data
        event_sink.add_idle_callback(move |data: &mut AppData| {
            *data = data_clone;
        });
        thread::sleep(Duration::from_millis(500));

        reset(&mut left_color, &mut right_color, &mut bottom_color);
        let data_clone = AppData {
            top_left: left_color.clone(),
            top_right: right_color.clone(),
            bottom: bottom_color.clone(),
        };
        event_sink.add_idle_callback(move |data: &mut AppData| {
            *data = data_clone;
        });

        thread::sleep(Duration::from_millis(rng.gen_range(500, 2000)));
    }
}

fn reset(left: &mut Color, right: &mut Color, bottom: &mut Color) {
    *left = Color::BLACK;
    *right = Color::BLACK;
    *bottom = Color::BLACK;
}

fn make_ui() -> impl Widget<AppData> {
    let top_left = Painter::new(|ctx, data: &AppData, _env| {
        let rect = ctx.size().to_rounded_rect(5.0);
        ctx.fill(rect, &data.top_left);
    });

    let top_right = Painter::new(|ctx, data: &AppData, _env| {
        let rect = ctx.size().to_rounded_rect(5.0);
        ctx.fill(rect, &data.top_right);
    });

    let bottom = Painter::new(|ctx, data: &AppData, _env| {
        let rect = ctx.size().to_rounded_rect(5.0);
        ctx.fill(rect, &data.bottom);
    });

    Flex::column()
        .with_flex_child(
            Flex::row()
                .with_flex_child(top_left, 7.0)
                .with_flex_spacer(0.1)
                .with_flex_child(top_right, 7.0),
            5.0,
        )
        .with_flex_spacer(0.1)
        .with_flex_child(bottom, 5.0)
}
