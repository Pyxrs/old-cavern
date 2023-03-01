use std::time::Instant;

use pollster::block_on;
use shared::{extra::warn, util::ThisOrThat, StaticModule};
use winit::{
    event::{ElementState, Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use crate::{
    input::{Input, InputInfo, Key},
    Client, ClientIO,
};

use self::surface::WindowSurface;

pub mod camera;
pub mod surface;
mod texture;

pub struct Window {
    pub info: WindowInfo,
}

#[derive(Default)]
pub struct WindowInfo {
    pub delta: f64,
    pub fps: f64,
}

impl StaticModule<(), ()> for Window {
    #[profiling::function]
    fn new(_: ()) -> ((), Self) {
        (
            (),
            Self {
                info: WindowInfo::default(),
            },
        )
    }
}

impl Window {
    #[profiling::function]
    pub fn run<F, E, S: 'static>(
        mut self,
        (mut game_state, mut client, client_io, frame, exit): (S, Client, ClientIO, F, E),
    ) where
        F: Fn(&mut S, &mut Client, &ClientIO) + 'static,
        E: Fn(&mut S, &mut Client, &ClientIO) + 'static,
    {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().build(&event_loop).unwrap();

        let mut state = block_on(WindowSurface::new(
            window,
            &client.config,
            &client.addon_manager,
        ));
        let mut last_render_time = Instant::now();

        event_loop.run(move |event, _, control_flow| match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == state.window().id() => {
                Input::process_input(&mut client.input, ThisOrThat::This(event));
                state.camera_controller.process_input(&client.input);
                if let Ok(InputInfo::Key(Key::Escape, ElementState::Pressed)) =
                    client_io.input_io.try_recv()
                {
                    warn!("Exit requested!");
                    exit(&mut game_state, &mut client, &client_io);
                    *control_flow = ControlFlow::Exit;
                }

                match event {
                    WindowEvent::CloseRequested => {
                        exit(&mut game_state, &mut client, &client_io);
                        *control_flow = ControlFlow::Exit
                    },
                    WindowEvent::Resized(physical_size) => state.resize(*physical_size),
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        state.resize(**new_inner_size)
                    }
                    _ => {}
                }
            }

            Event::DeviceEvent { ref event, .. } => {
                Input::process_input(&mut client.input, ThisOrThat::That(event));
                state.camera_controller.process_input(&client.input);
            }

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
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                        state.resize(state.size)
                    }
                    Err(wgpu::SurfaceError::OutOfMemory) => {
                        exit(&mut game_state, &mut client, &client_io);
                        *control_flow = ControlFlow::Exit
                    },
                    Err(wgpu::SurfaceError::Timeout) => warn!("Surface timeout"),
                    _ => {}
                };
                profiling::finish_frame!();
            }

            Event::MainEventsCleared => state.window().request_redraw(),

            _ => {}
        });
    }
}
