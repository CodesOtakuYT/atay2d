#![allow(unused)]
#![warn(unused_must_use)]

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn run() {
    let window_size = PhysicalSize::new(720, 480);
    let title = "Atay2d Game";
    let canvas_parent_id = "atay2d-canvas-parent";
    let resizable = true;

    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Warn).expect("Couldn't initialize logger");
        } else {
            env_logger::init();
        }
    }

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title(title)
        .with_inner_size(window_size)
        .with_resizable(resizable)
        .build(&event_loop)
        .unwrap();
    let window_id = window.id();

    #[cfg(target_arch = "wasm32")]
    {
        use winit::platform::web::WindowExtWebSys;
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| {
                let dst = doc.get_element_by_id(canvas_parent_id)?;
                let canvas = web_sys::Element::from(window.canvas());
                dst.append_child(&canvas).ok()?;
                Some(())
            })
            .expect("Couldn't append canvas to document body.");
    }

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent { window_id, event } => match event {
            WindowEvent::CloseRequested => control_flow.set_exit(),
            _ => {}
        },
        _ => {}
    })
}
