#[cfg(feature = "rpi")]
use rppal::gpio::{Gpio, Level};

use crate::controller::{InputController, InputEvent};
use crate::music::{Pitch, PitchClass};

#[cfg(feature = "rpi")]
pub struct GpioController
{
    key_state: [bool; 61]
}

impl GpioController
{
    pub fn new() -> Self
    {
        let key_state = [false; 61];
        return GpioController{key_state};
    }
}
impl InputController for GpioController
{
    fn register_and_start(&mut self, sender: mpsc::Sender<InputEvent>)
    {
        let driver = Gpio::new().expect("Failed loading GPIO");

        let pin_keyin   = driver.get(4) .expect("Failed getting pin 4" ).into_input();
        let mut pin_keysel0 = driver.get(17).expect("Failed getting pin 17").into_output();
        let mut pin_keysel1 = driver.get(27).expect("Failed getting pin 27").into_output();
        let mut pin_keysel2 = driver.get(22).expect("Failed getting pin 22").into_output();
        let mut pin_keysel3 = driver.get(5) .expect("Failed getting pin 5" ).into_output();
        let mut pin_keysel4 = driver.get(6) .expect("Failed getting pin 6" ).into_output();
        let mut pin_keysel5 = driver.get(13).expect("Failed getting pin 13").into_output();

        for i in 0..61
        {
            pin_keysel0.write(if i & 1  > 0 { Level::High } else { Level::Low });
            pin_keysel1.write(if i & 2  > 0 { Level::High } else { Level::Low });
            pin_keysel2.write(if i & 4  > 0 { Level::High } else { Level::Low });
            pin_keysel3.write(if i & 8  > 0 { Level::High } else { Level::Low });
            pin_keysel4.write(if i & 16 > 0 { Level::High } else { Level::Low });
            pin_keysel5.write(if i & 32 > 0 { Level::High } else { Level::Low });

            let new_key_state = pin_keyin.is_high();
            if self.key_state[i] != new_key_state {
                let pitch = Pitch::from_midi_key(i + 12);
                let event = if(new_key_state) {
                    InputEvent::NoteOn(pitch)
                } else {
                    InputEvent::NoteOff(pitch)
                };
                sender.send(event).expect("Failed to send note");
            }
        }
    }
}
