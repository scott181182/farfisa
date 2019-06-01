use std::sync::mpsc;

use crate::controller::{InputController, InputEvent};

pub mod fluid;

pub trait Synth
{
    fn add_controller(&self, controller: &mut impl InputController)
    {
        controller.register_and_start(self.create_listener());
    }

    fn create_listener(&self) -> mpsc::Sender<InputEvent>;
    fn send_event(&self, event: InputEvent);
    fn start(&self);
}
