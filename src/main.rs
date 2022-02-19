extern crate stats_alloc;

use std::alloc::System;

#[global_allocator]
static GLOBAL: &StatsAlloc<System> = &INSTRUMENTED_SYSTEM;

use fltk::{app, button::Button, frame::Frame, prelude::*, window::Window};
use stats_alloc::{INSTRUMENTED_SYSTEM, Region, StatsAlloc};

fn main() {
    // let app = app::App::default();
    // let mut wind = Window::default().with_size(400, 300);
    // let mut frame = Frame::default().with_size(200, 100).center_of(&wind);
    // let mut but = Button::new(160, 210, 80, 40, "Click me!");
    // wind.end();
    // wind.show();
    //
    // but.set_callback(move |_| frame.set_label("Hello world"));
    //
    // app.run().unwrap();

    let app = app::App::default();
    let mut wind = Window::default().with_size(400, 300);
    wind.end();
    wind.show();
    // let reg = Region::new(&GLOBAL);
    for i in 0..100 {
        app::awake_callback(move || {
            println!("Called from event loop {i}");
        });
    }
    // println!("Stats at 1: {:#?}", reg.change());

    app.run().unwrap();
}
