use std::sync::mpsc;

use crate::music::Pitch;

pub mod keyboard;

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
