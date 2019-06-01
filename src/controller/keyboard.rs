use std::sync::mpsc;
use std::thread;

use crate::controller::{InputController, InputEvent};
use crate::music::{Pitch, PitchClass};
use crossterm_input::{self, RawScreen, InputEvent as TermInputEvent, KeyEvent};

#[derive(Debug)]
pub struct KeyboardController {  }

impl KeyboardController
{
    pub fn new() -> Self
    {
        return KeyboardController{  };
    }
}

impl InputController for KeyboardController
{
    fn register_and_start(&mut self, sender: mpsc::Sender<InputEvent>)
    {
        thread::spawn(move || {
            let _screen = RawScreen::into_raw_mode();
            let input = crossterm_input::input();
            let mut stdin = input.read_sync();

            while let Some(TermInputEvent::Keyboard(KeyEvent::Char(c))) = stdin.next()
            {
                let pitch = match c {
                    'a' => Pitch::new(PitchClass::C,  4),
                    'w' => Pitch::new(PitchClass::Db, 4),
                    's' => Pitch::new(PitchClass::D,  4),
                    'e' => Pitch::new(PitchClass::Eb, 4),
                    'd' => Pitch::new(PitchClass::E,  4),
                    'f' => Pitch::new(PitchClass::F,  4),
                    't' => Pitch::new(PitchClass::Gb, 4),
                    'g' => Pitch::new(PitchClass::G,  4),
                    'y' => Pitch::new(PitchClass::Ab, 4),
                    'h' => Pitch::new(PitchClass::A,  4),
                    'u' => Pitch::new(PitchClass::Bb, 4),
                    'j' => Pitch::new(PitchClass::B,  4),
                    'k' => Pitch::new(PitchClass::C,  5),
                    _   => {
                        sender.send(InputEvent::Quit).expect("Failed sending quit event");
                        break;
                    }
                };
                sender.send(InputEvent::NoteOn(pitch)).expect("Failed sending note event");
            }
        });
    }
}
