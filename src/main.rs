

mod controller;
mod music;
mod synth;

use synth::Synth;

fn main()
{

}

#[test]
#[cfg(feature = "kb")]
fn test_keyboard()
{
    let synth = synth::fluid::FluidSynth::new();

    let mut keyboard = controller::keyboard::KeyboardController::new();
    synth.add_controller(&mut keyboard);
    synth.start();
}

// fn init_fluidsynth() -> fluidsynth::synth::Synth
// {
//     let mut settings = fluidsynth::settings::Settings::new();
//     let mut synth = fluidsynth::synth::Synth::new(&mut settings);
//     synth.sfload("/Users/scottfasone/Downloads/GeneralUser GS 1.471/GeneralUser GS v1.471.sf2", 1);
//     let _adriver = fluidsynth::audio::AudioDriver::new(&mut settings, &mut synth);

//     return synth;
//     // let major_gamut = vec![0, 2, 4, 5, 7, 9, 11, 12];
//     // for note in major_gamut {
//     //     let key = 60 + note;
//     //     syn.noteon(0, key, 80);
//     //     thread::sleep(Duration::from_millis(500));
//     //     syn.noteoff(0, key);
//     // }
// }
