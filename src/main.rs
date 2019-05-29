extern crate fluidsynth;

use std::thread;
use std::time::Duration;

fn main() {
    println!("Hello, fluidsynth!");

    let mut settings = fluidsynth::settings::Settings::new();
    let mut syn = fluidsynth::synth::Synth::new(&mut settings);
    syn.sfload("/Users/scottfasone/Downloads/GeneralUser GS 1.471/GeneralUser GS v1.471.sf2", 1);
    let _adriver = fluidsynth::audio::AudioDriver::new(&mut settings, &mut syn);

    let major_gamut = vec![0, 2, 4, 5, 7, 9, 11, 12];
    for note in major_gamut {
        let key = 60 + note;
        syn.noteon(0, key, 80);
        thread::sleep(Duration::from_millis(500));
        syn.noteoff(0, key);
    }
}
