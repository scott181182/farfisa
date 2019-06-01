use std::str::FromStr;


#[allow(dead_code)]
#[derive(Debug)]
pub enum PitchClass
{
    A, Bb, B, C, Db, D, Eb, E, F, Gb, G, Ab
}
impl PitchClass
{
    pub fn to_pitch_number(&self) -> i32
    {
        match &self {
            PitchClass::A  => 9,
            PitchClass::Bb => 10,
            PitchClass::B  => 11,
            PitchClass::C  => 0,
            PitchClass::Db => 1,
            PitchClass::D  => 2,
            PitchClass::Eb => 3,
            PitchClass::E  => 4,
            PitchClass::F  => 5,
            PitchClass::Gb => 6,
            PitchClass::G  => 7,
            PitchClass::Ab => 8,
        }
    }
}
impl FromStr for PitchClass
{
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()>
    {
        match s.to_lowercase().as_str()
        {
            "a"  => Ok(PitchClass::A),
            "bb" => Ok(PitchClass::Bb),
            "b"  => Ok(PitchClass::B),
            "c"  => Ok(PitchClass::C),
            "db" => Ok(PitchClass::Db),
            "d"  => Ok(PitchClass::D),
            "eb" => Ok(PitchClass::Eb),
            "e"  => Ok(PitchClass::E),
            "f"  => Ok(PitchClass::F),
            "gb" => Ok(PitchClass::Gb),
            "g"  => Ok(PitchClass::G),
            "ab" => Ok(PitchClass::Ab),
            _    => Err(())
        }
    }
}

#[derive(Debug)]
pub struct Pitch
{
    class: PitchClass,
    octave: u8
}

impl Pitch
{
    pub fn new(class: PitchClass, octave: u8) -> Self
    {
        return Pitch { class, octave };
    }

    pub fn to_midi_key(&self) -> i32
    {
        return 12 + (12 * self.octave as i32) + self.class.to_pitch_number();
    }
}



// TODO

// impl FromStr for Pitch
// {
//     type Err = ();

//     fn from_str(s: &str) -> Result<Self, ()>
//     {
//         match s.to_lowercase().as_str()
//         {
//             "a"  => Ok(PitchClass::A),
//             "bb" => Ok(PitchClass::Bb),
//             "b"  => Ok(PitchClass::B),
//             "c"  => Ok(PitchClass::C),
//             "db" => Ok(PitchClass::Db),
//             "d"  => Ok(PitchClass::D),
//             "eb" => Ok(PitchClass::Eb),
//             "e"  => Ok(PitchClass::E),
//             "f"  => Ok(PitchClass::F),
//             "gb" => Ok(PitchClass::Gb),
//             "g"  => Ok(PitchClass::G),
//             "ab" => Ok(PitchClass::Ab),
//             _    => Err(())
//         }
//     }
// }
