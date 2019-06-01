use std::str::FromStr;


#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq)]
pub enum PitchClass
{
    A, Bb, B, C, Db, D, Eb, E, F, Gb, G, Ab
}
impl From<u8> for PitchClass
{
    fn from(number: u8) -> PitchClass
    {
        match number % 12 {
            0  => PitchClass::C,
            1  => PitchClass::Db,
            2  => PitchClass::D,
            3  => PitchClass::Eb,
            4  => PitchClass::E,
            5  => PitchClass::F,
            6  => PitchClass::Gb,
            7  => PitchClass::G,
            8  => PitchClass::Ab,
            9  => PitchClass::A,
            10 => PitchClass::Bb,
            11 => PitchClass::B,
            _  => unreachable!()
        }
    }
}
impl PitchClass
{
    fn to_number(&self) -> i32
    {
        match self {
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

#[derive(Debug, PartialEq, Eq)]
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
        return 12 + (12 * self.octave as i32) + self.class.to_number();
    }
    pub fn from_midi_key(key: u8) -> Pitch
    {
        let class  = PitchClass::from(key);
        let octave = (key / 12) - 1;
        return Pitch::new(class, octave);
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



#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_pc_from_u8() {
        assert_eq!(PitchClass::from(0),  PitchClass::C);
        assert_eq!(PitchClass::from(1),  PitchClass::Db);
        assert_eq!(PitchClass::from(6),  PitchClass::Gb);
        assert_eq!(PitchClass::from(10), PitchClass::Bb);
    }
    #[test]
    fn test_pc_to_number() {
        assert_eq!(PitchClass::D.to_number(), 2);
        assert_eq!(PitchClass::F.to_number(), 5);
        assert_eq!(PitchClass::A.to_number(), 9);
        assert_eq!(PitchClass::C.to_number(), 0);
    }
    #[test]
    fn test_pc_from_str() {
        assert_eq!(PitchClass::from_str("C").unwrap(), PitchClass::C);
        assert_eq!(PitchClass::from_str("e").unwrap(), PitchClass::E);
        assert_eq!(PitchClass::from_str("Gb").unwrap(), PitchClass::Gb);
        assert_eq!(PitchClass::from_str("bb").unwrap(), PitchClass::Bb);
    }
    #[test]
    fn test_pitch_from_midi() {
        assert_eq!(Pitch::from_midi_key(12),  Pitch::new(PitchClass::C, 0));
        assert_eq!(Pitch::from_midi_key(57),  Pitch::new(PitchClass::A, 3));
        assert_eq!(Pitch::from_midi_key(64),  Pitch::new(PitchClass::E, 4));
        assert_eq!(Pitch::from_midi_key(107), Pitch::new(PitchClass::B, 7));
    }
    #[test]
    fn test_pitch_to_midi() {
        assert_eq!(Pitch::from_midi_key(12) .to_midi_key(), 12);
        assert_eq!(Pitch::from_midi_key(57) .to_midi_key(), 57);
        assert_eq!(Pitch::from_midi_key(64) .to_midi_key(), 64);
        assert_eq!(Pitch::from_midi_key(107).to_midi_key(), 107);
    }
}
