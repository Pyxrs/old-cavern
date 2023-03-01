use std::{collections::HashMap, sync::mpsc::{Sender, Receiver, channel}};

use shared::{
    extra::{debug, Vector2, Zero, InnerSpace},
    util::{GetOrInsert, ThisOrThat},
    StaticModule, Ignore,
};
use winit::event::{DeviceEvent, ElementState, MouseButton, MouseScrollDelta, WindowEvent};

pub use winit::event::VirtualKeyCode as Key;

pub struct Input {
    mouse: Mouse,
    inputs: HashMap<InputType, Vec<String>>,
    actions: HashMap<String, InputInfo>,
    sender: Sender<InputInfo>,
}

#[derive(Eq, Hash, PartialEq, Debug)]
pub enum InputType {
    Key(Key),
    MouseButton(MouseButton),
    MouseScroll,
    MouseMotion,
}

#[derive(Clone, Copy, Debug)]
pub enum InputInfo {
    None,
    Key(Key, ElementState),
    MouseButton(MouseButton, ElementState),
    MouseScroll(Vector2<f32>),
    MouseMotion(Vector2<f32>),
}

impl StaticModule<(), Receiver<InputInfo>> for Input {
    #[profiling::function]
    fn new(_: ()) -> (Receiver<InputInfo>, Self) {
        let (sender, receiver) = channel();

        (receiver, Self {
            mouse: Mouse::default(),
            inputs: HashMap::new(),
            actions: HashMap::new(),
            sender,
        })
    }
}

impl Input {
    #[profiling::function]
    pub(crate) fn process_frame(&mut self) {
        let actions = self.inputs.get_or_insert(InputType::MouseMotion, vec![]);
        for action in actions.iter() {
            *self.actions.get_or_insert(action.to_string(), InputInfo::None) = InputInfo::MouseMotion(self.mouse.motion);
        }

        let actions = self.inputs.get_or_insert(InputType::MouseScroll, vec![]);
        for action in actions.iter() {
            *self.actions.get_or_insert(action.to_string(), InputInfo::None) = InputInfo::MouseScroll(self.mouse.scroll);
        }

        self.mouse = Mouse {
            motion: Vector2::zero(),
            scroll: Vector2::zero(),
        };
    }

    #[profiling::function]
    pub(crate) fn process_input(
        input: &mut Input,
        event: ThisOrThat<&WindowEvent, &DeviceEvent>,
    ) {
        match event {
            ThisOrThat::This(window_event) => match window_event {
                // Keyboard buttons
                WindowEvent::KeyboardInput {
                    input: keyboard_input,
                    ..
                } => if let Some(virtual_keycode) = keyboard_input.virtual_keycode {
                    button(
                        input,
                        &keyboard_input.state,
                        &InputType::Key(virtual_keycode),
                        |state| InputInfo::Key(virtual_keycode, *state),
                    )
                },

                // Mouse buttons
                WindowEvent::MouseInput {
                    state,
                    button: mouse_button,
                    ..
                } => button(
                    input,
                    state,
                    &InputType::MouseButton(*mouse_button),
                    |state| InputInfo::MouseButton(*mouse_button, *state),
                ),

                // Mouse wheel
                WindowEvent::MouseWheel { delta, .. } => {
                    if let MouseScrollDelta::PixelDelta(delta) = delta {
                        input.mouse.scroll += Vector2::new(delta.x as f32, delta.y as f32)
                    }
                }

                _ => (),
            },

            ThisOrThat::That(device_event) => match device_event {
                // Mouse motion
                DeviceEvent::MouseMotion { delta } => {
                    input.mouse.motion += Vector2::new(delta.0 as f32, delta.1 as f32);
                }

                _ => (),
            },
        };
    }

    #[profiling::function]
    pub fn add_action(&mut self, name: impl Into<String>, inputs: Vec<InputType>) {
        let name: String = name.into();
        debug!("Action added: {}", &name);

        for input in inputs {
            self.inputs.get_or_insert(input, vec![]).push(name.clone());
        }
        self.actions.insert(name, InputInfo::None);
    }

    #[profiling::function]
    pub fn add_actions(&mut self, actions: Vec<(impl Into<String>, Vec<InputType>)>) {
        for (name, inputs) in actions {
            let name: String = name.into();
            self.actions.insert(name.clone(), InputInfo::None);
            self.add_action(name, inputs);
        }
    }

    #[profiling::function]
    pub fn query_action(&self, name: impl Into<String>) -> Result<&InputInfo, &str> {
        self.actions
            .get(&name.into())
            .ok_or("Action not found!")
    }

    /// Mouse scroll or motion returns the magnitude squared
    #[profiling::function]
    pub fn query_action_strength(&self, name: impl Into<String>) -> Result<f32, &str> {
        let input = self.query_action(name)?;
        match input {
            InputInfo::None => Ok(0.0),
            InputInfo::Key(_, state) | InputInfo::MouseButton(_, state) => match state {
                ElementState::Pressed => Ok(1.0),
                ElementState::Released => Ok(0.0),
            },
            InputInfo::MouseScroll(delta) | InputInfo::MouseMotion(delta) => Ok(delta.normalize().magnitude2()),
        }
    }
}

#[derive(Clone, Copy)]
pub struct Mouse {
    pub motion: Vector2<f32>,
    pub scroll: Vector2<f32>,
}

impl Default for Mouse {
    fn default() -> Self {
        Self {
            motion: Vector2::zero(),
            scroll: Vector2::zero(),
        }
    }
}

#[profiling::function]
fn button<T>(input: &mut Input, button: &ElementState, input_type: &InputType, input_value: T)
where
    T: Fn(&ElementState) -> InputInfo,
{
    let mut actions_to_update = vec![];

    if let Some(actions) = input.inputs.get(input_type) {
        for action_name in actions.iter() {
            if input.actions.contains_key(action_name) {
                actions_to_update.push(action_name.clone());
            }
        }
    }

    for action_name in actions_to_update {
        let action = input
            .actions
            .get_or_insert(action_name, InputInfo::None);

        *action = input_value(button);

        input.sender.send(*action).ignore();
    }
}
