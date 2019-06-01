use std::sync::mpsc;

use crate::music::Pitch;

#[cfg(feature = "kb")]
pub mod keyboard;
#[cfg(feature = "rpi")]
pub mod gpio;

pub enum InputEvent
{
    NoteOn(Pitch),
    NoteOff(Pitch),
    Quit
}
pub trait InputController
{
    fn register_and_start(&mut self, sender: mpsc::Sender<InputEvent>);
}
