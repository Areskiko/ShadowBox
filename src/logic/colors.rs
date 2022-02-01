use std::thread;
use std::time::Duration;
use rand::Rng;

use druid::Color;

use super::appdata::AppData;

pub fn generate_colors(event_sink: druid::ExtEventSink) {
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