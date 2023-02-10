use std::collections::HashMap;

use shared::{
    extra::{anyhow, debug, unbounded, Receiver, Result, Sender, Vector2, Zero, InnerSpace},
    util::{GetOrInsert, ThisOrThat},
    InnerModule, Module,
};
use winit::event::{DeviceEvent, ElementState, MouseButton, MouseScrollDelta, WindowEvent};

pub use winit::event::VirtualKeyCode as Key;

pub struct Input {
    mouse: Mouse,
    inputs: HashMap<InputType, Vec<String>>,
    actions: HashMap<String, (InputInfo, Vec<Sender<InputInfo>>)>,
}

impl InnerModule<()> for Input {}

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
    Key(ElementState),
    MouseButton(ElementState),
    MouseScroll(Vector2<f32>),
    MouseMotion(Vector2<f32>),
}

impl Input {
    pub fn new() -> Self {
        Input {
            mouse: Mouse::default(),
            inputs: HashMap::new(),
            actions: HashMap::new(),
        }
    }

    pub(crate) fn process_frame(&mut self) {
        let actions = self.inputs.get_or_insert(InputType::MouseMotion, vec![]);
        for action in actions.iter() {
            self.actions
                .get_or_insert(action.to_string(), (InputInfo::None, vec![]))
                .0 = InputInfo::MouseMotion(self.mouse.motion);
        }

        let actions = self.inputs.get_or_insert(InputType::MouseScroll, vec![]);
        for action in actions.iter() {
            self.actions
                .get_or_insert(action.to_string(), (InputInfo::None, vec![]))
                .0 = InputInfo::MouseScroll(self.mouse.scroll);
        }

        self.mouse = Mouse {
            motion: Vector2::zero(),
            scroll: Vector2::zero(),
        };
    }

    pub(crate) fn process_input(
        input: &Module<Input>,
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
                        &input,
                        &keyboard_input.state,
                        &InputType::Key(virtual_keycode),
                        |state| InputInfo::Key(*state),
                    )
                },

                // Mouse buttons
                WindowEvent::MouseInput {
                    state,
                    button: mouse_button,
                    ..
                } => button(
                    &input,
                    state,
                    &InputType::MouseButton(*mouse_button),
                    |state| InputInfo::MouseButton(*state),
                ),

                // Mouse wheel
                WindowEvent::MouseWheel { delta, .. } => {
                    if let MouseScrollDelta::PixelDelta(delta) = delta {
                        input.write().unwrap().mouse.scroll +=
                            Vector2::new(delta.x as f32, delta.y as f32)
                    }
                }

                _ => (),
            },

            ThisOrThat::That(device_event) => match device_event {
                // Mouse motion
                DeviceEvent::MouseMotion { delta } => {
                    input.write().unwrap().mouse.motion +=
                        Vector2::new(delta.0 as f32, delta.1 as f32);
                }

                _ => (),
            },
        };
    }

    pub fn add_action(&mut self, name: impl Into<String>, inputs: Vec<InputType>) {
        let name: String = name.into();
        debug!("Action added: {}", &name);

        for input in inputs {
            self.inputs.get_or_insert(input, vec![]).push(name.clone());
        }
        self.actions.insert(name, (InputInfo::None, vec![]));
    }

    pub fn add_actions(&mut self, actions: Vec<(impl Into<String>, Vec<InputType>)>) {
        for (name, inputs) in actions {
            let name: String = name.into();
            self.actions.insert(name.clone(), (InputInfo::None, vec![]));
            self.add_action(name, inputs);
        }
    }

    pub fn subscribe_action(&mut self, name: impl Into<String>) -> Receiver<InputInfo> {
        let name: String = name.into();
        debug!("Action subscribed: {}", &name);

        let (sender, receiver) = unbounded();
        self.actions
            .get_or_insert(&name, (InputInfo::None, vec![]))
            .1
            .push(sender);
        
        receiver
    }

    pub fn query_action(&self, name: impl Into<String>) -> Result<InputInfo> {
        self.actions
            .get(&name.into())
            .ok_or(anyhow!("Action not found!"))
            .map(|t| t.0)
    }

    /// Mouse scroll or motion returns the magnitude squared
    pub fn query_action_strength(&self, name: impl Into<String>) -> Result<f32> {
        let input = self.query_action(name)?;
        match input {
            InputInfo::None => Ok(0.0),
            InputInfo::Key(state) | InputInfo::MouseButton(state) => match state {
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

fn button<T>(input: &Module<Input>, button: &ElementState, input_type: &InputType, input_value: T)
where
    T: Fn(&ElementState) -> InputInfo,
{
    let input_read = input.read().unwrap();
    let mut actions_to_update = vec![];

    if let Some(actions) = input_read.inputs.get(input_type) {
        for action_name in actions.iter() {
            if input_read.actions.contains_key(action_name) {
                actions_to_update.push(action_name.clone());
            }
        }
    }

    drop(input_read);
    let mut input_write = input.write().unwrap();

    for action_name in actions_to_update {
        let action = input_write
            .actions
            .get_or_insert(action_name, (InputInfo::None, vec![]));

        action.0 = input_value(button);
        for sender in &action.1 {
            sender.try_send(action.0).unwrap();
        }
    }
}
