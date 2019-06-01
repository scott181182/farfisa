use fluidsynth;

use std::sync::mpsc;

use crate::controller::InputEvent;
use crate::synth::Synth;

#[allow(dead_code)]
pub struct FluidSynth
{
    settings:   fluidsynth::settings::Settings,
    pub player: fluidsynth::synth::Synth,
    driver:     fluidsynth::audio::AudioDriver,
    sender:     mpsc::Sender<InputEvent>,
    receiver:   mpsc::Receiver<InputEvent>
}

impl FluidSynth
{
    pub fn new() -> Self
    {
        let mut settings = fluidsynth::settings::Settings::new();
        let mut player = fluidsynth::synth::Synth::new(&mut settings);
        player.sfload("/Users/scottfasone/Downloads/GeneralUser GS 1.471/GeneralUser GS v1.471.sf2", 1);
        let driver = fluidsynth::audio::AudioDriver::new(&mut settings, &mut player);
        let (sender, receiver) = mpsc::channel();

        return FluidSynth{settings, player, driver, sender, receiver};
    }
}

impl Synth for FluidSynth
{
    fn create_listener(&self) -> mpsc::Sender<InputEvent>
    {
        return self.sender.clone();
    }
    fn send_event(&self, event: InputEvent)
    {
        self.sender.send(event).unwrap();
    }
    fn start(&self)
    {
        self.receiver.iter()
            .for_each(|event| {
                match event
                {
                    InputEvent::NoteOn (pitch) => {
                        let midi_key = pitch.to_midi_key();
                        println!("Playing: {:?}, {:?}", pitch, midi_key);
                        self.player.noteon(0, midi_key, 80);
                    },
                    InputEvent::NoteOff(pitch) => {
                        let midi_key = pitch.to_midi_key();
                        println!("Stopping: {:?}, {:?}", pitch, midi_key);
                        self.player.noteoff(0, midi_key);
                    },
                    InputEvent::Quit => ()
                }
            });
    }
}
