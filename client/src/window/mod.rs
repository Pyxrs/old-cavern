use std::time::Instant;

use pollster::block_on;
use shared::{Module, InnerModule, extra::warn, util::ThisOrThat};
use winit::{event_loop::{EventLoop, ControlFlow}, window::WindowBuilder, event::{Event, WindowEvent, ElementState}};

use crate::{world::World, input::{Input, InputInfo}};

use self::surface::WindowSurface;

pub mod surface;
mod camera;
mod texture;

pub struct Window {
    pub info: WindowInfo,
}

#[derive(Default)]
pub struct WindowInfo {
    pub delta: f64,
    pub fps: f64,
}

impl Window {
    pub fn new() -> Self {
        Self {
            info: WindowInfo::default(),
        }
    }
}

impl InnerModule<(Module<Input>, Module<World>)> for Window {
    fn run(module: Module<Self>, (input, _world): (Module<Input>, Module<World>)) {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().build(&event_loop).unwrap();
        
        let mut state = block_on(WindowSurface::new(window));
        let mut last_render_time = Instant::now();
        let exit = input.write().unwrap().subscribe_action("exit");

        event_loop.run(move |event, _, control_flow| {
            let input = input.clone();

            if let Ok(InputInfo::Key(ElementState::Pressed)) = exit.try_recv() {
                warn!("Exit requested!");
                *control_flow = ControlFlow::Exit;
            }

            match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == state.window().id() => {
                    Input::process_input(input, ThisOrThat::This(event));

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
                    Input::process_input(input, ThisOrThat::That(event));
                }
                Event::RedrawRequested(window_id) if window_id == state.window().id() => {
                    input.write().unwrap().early_process_frame();

                    let now = Instant::now();
                    let delta = now - last_render_time;
                    last_render_time = now;

                    {
                        let mut write = module.write().unwrap();
                        write.info.delta = delta.as_secs_f64();
                        write.info.fps = 1.0 / write.info.delta;
                    }

                    state.update(delta.as_secs_f32());
                    match state.render() {
                        Ok(_) => {},
                        Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => state.resize(state.size),
                        Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                        Err(wgpu::SurfaceError::Timeout) => warn!("Surface timeout"),
                    }
                    input.write().unwrap().late_process_frame();
                }
                Event::MainEventsCleared => state.window().request_redraw(),
                _ => {}
            }
        });
    }
}