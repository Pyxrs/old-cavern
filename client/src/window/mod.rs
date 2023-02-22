use std::time::Instant;

use pollster::block_on;
use shared::{extra::warn, util::ThisOrThat, StaticModule};
use winit::{event_loop::{EventLoop, ControlFlow}, window::WindowBuilder, event::{Event, WindowEvent, ElementState}};

use crate::{input::{Input, InputInfo, Key}, Client, ClientIO};

use self::surface::WindowSurface;

pub mod surface;
pub mod camera;
mod texture;

pub struct Window {
    pub info: WindowInfo,
}

#[derive(Default)]
pub struct WindowInfo {
    pub delta: f64,
    pub fps: f64,
}

impl StaticModule<()> for Window {
    fn new() -> ((), Self) {
        (
            (),
            Self {
                info: WindowInfo::default(),
            }
        )
    }
}

impl Window {
    pub fn run<F, S: 'static>(mut self, (mut game_state, mut client, client_io, frame): (S, Client, ClientIO, F)) where F: Fn(&mut S, &mut Client, &ClientIO) + 'static {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().build(&event_loop).unwrap();
        
        let mut state = block_on(WindowSurface::new(window, &client.config, &client.addon_manager));
        let mut last_render_time = Instant::now();

        event_loop.run(move |event, _, control_flow| {
            match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == state.window().id() => {
                    Input::process_input(&mut client.input, ThisOrThat::This(event));
                    state.camera_controller.process_input(&client.input);
                    if let Ok(InputInfo::Key(Key::Escape, ElementState::Pressed)) = client_io.input_io.try_recv() {
                        warn!("Exit requested!");
                        *control_flow = ControlFlow::Exit;
                    }

                    match event {
                        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                        WindowEvent::Resized(physical_size) => state.resize(*physical_size),
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => state.resize(**new_inner_size),
                        _ => {}
                    }
                }

                Event::DeviceEvent {
                    ref event,
                    ..
                } => {
                    Input::process_input(&mut client.input, ThisOrThat::That(event));
                    state.camera_controller.process_input(&client.input);
                },
            
                Event::RedrawRequested(window_id) if window_id == state.window().id() => {
                    client.input.process_frame();

                    let now = Instant::now();
                    let delta = now - last_render_time;
                    last_render_time = now;

                    {
                        self.info.delta = delta.as_secs_f64();
                        self.info.fps = 1.0 / self.info.delta;
                    }

                    state.update(delta.as_secs_f32());

                    frame(&mut game_state, &mut client, &client_io);

                    match state.render() {
                        Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => state.resize(state.size),
                        Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                        Err(wgpu::SurfaceError::Timeout) => warn!("Surface timeout"),
                        _ => {},
                    }
                }

                Event::MainEventsCleared => state.window().request_redraw(),

                _ => {}
            }
        });
    }
}