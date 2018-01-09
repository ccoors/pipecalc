// This file is part of pipecalc.
//
// pipecalc is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// pipecalc is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with pipecalc.  If not, see <http://www.gnu.org/licenses/>.

use std::f64;

pub enum Temperament {
    EQUAL {
        base_frequency: f64,
        base_pitch: Pitch,
    },
}

#[derive(Clone, PartialEq, Debug)]
pub struct Pitch {
    // C = 0, C# = 1, ...
    note: i32,

    // 0 = C0, 1 = C1, ...
    octave: i32,

    // Cents, -100.0 - +100.0
    cents: f64,
}

impl Pitch {
    pub fn from(note: i32, octave: i32) -> Self {
        let mut pitch = Pitch {
            note,
            octave,
            cents: 0.0,
        };
        pitch.normalize();
        pitch
    }

    pub fn from_with_cents(note: i32, octave: i32, cents: f64) -> Self {
        let mut pitch = Pitch {
            note,
            octave,
            cents,
        };
        pitch.normalize();
        pitch
    }

    pub fn add(&self, cents: f64) -> Self {
        let mut pitch = Pitch {
            note: self.note,
            octave: self.octave,
            cents: self.cents + cents,
        };
        pitch.normalize();
        pitch
    }

    pub fn normalize(&mut self) {
        while self.cents >= 100.0 {
            self.cents -= 100.0;
            self.note += 1;
        }

        while self.cents <= -100.0 {
            self.cents += 100.0;
            self.note -= 1;
        }

        while self.note > 11 {
            self.note -= 12;
            self.octave += 1;
        }

        while self.note < 0 {
            self.note += 12;
            self.octave -= 1;
        }
    }

    pub fn difference_to(&self, other: &Pitch) -> f64 {
        let octave_difference = (self.octave - other.octave) as f64;
        let note_difference = (self.note - other.note) as f64;
        let cent_difference = self.cents - other.cents;

        octave_difference * 12.0 + note_difference + cent_difference / 100.0
    }

    pub fn to_frequency(&self, temperament: &Temperament) -> f64 {
        let other_note = temperament.get_base_note();
        let steps = self.difference_to(&other_note);
        temperament.get_frequency(steps as f64)
    }

    pub fn get_tnm_radius(&self) -> f64 {
        let steps = self.difference_to(&Pitch {
            note: 0,
            octave: 2,
            cents: 0.0,
        });

        0.5f64 * 0.15555f64 * 0.957458f64.powf(steps)
    }
}

impl Temperament {
    pub fn new_default_equal() -> Temperament {
        Temperament::EQUAL {
            base_frequency: 440.0,
            base_pitch: Pitch {
                note: 9,
                octave: 4,
                cents: 0.0,
            },
        }
    }

    pub fn new_freq_equal(freq: f64) -> Temperament {
        Temperament::EQUAL {
            base_frequency: freq,
            base_pitch: Pitch {
                note: 9,
                octave: 4,
                cents: 0.0,
            },
        }
    }

    pub fn get_frequency(&self, steps: f64) -> f64 {
        match self {
            &Temperament::EQUAL {
                base_frequency: base,
                ..
            } => base * (2.0f64).powf(steps / 12.0),
        }
    }

    pub fn get_base_note(&self) -> Pitch {
        match self {
            &Temperament::EQUAL {
                base_pitch: ref pitch,
                ..
            } => pitch.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::util::*;

    #[test]
    fn test_pitches() {
        let pitch = Pitch::from(0, 4);
        let pitch = pitch.add(1200.0);
        assert!(
            pitch.difference_to(&Pitch {
                note: 0,
                octave: 5,
                cents: 0.0,
            }) < 0.01
        );

        let pitch = Pitch::from(2, 4);
        let pitch = pitch.add(200.0);
        assert!(
            pitch.difference_to(&Pitch {
                note: 4,
                octave: 4,
                cents: 0.0,
            }) < 0.01
        );

        let pitch = Pitch::from(2, 4);
        let pitch = pitch.add(-200.0);
        assert!(
            pitch.difference_to(&Pitch {
                note: 0,
                octave: 4,
                cents: 0.0,
            }) < 0.01
        );
    }

    #[test]
    fn test_pipe_diameter() {
        assert!(approx(
            Pitch::from(0, 2).get_tnm_radius() * 2.0,
            0.15555,
            0.01,
        ));
        assert!(approx(
            Pitch::from(0, 1).get_tnm_radius() * 2.0,
            0.26169,
            0.01,
        ));
        assert!(approx(
            Pitch::from(0, 3).get_tnm_radius() * 2.0,
            0.09245,
            0.01,
        ));

        assert!(approx(
            Pitch::from(5, 3).get_tnm_radius() * 2.0,
            0.07444,
            0.01,
        ));
    }

    #[test]
    fn test_equal_temperament() {
        let temp = Temperament::new_default_equal();

        assert!(approx(temp.get_frequency(0.0), 440.0, 0.0001));
        assert!(approx(temp.get_frequency(12.0), 880.0, 0.0001));
        assert!(approx(temp.get_frequency(-12.0), 220.0, 0.0001));

        assert!(approx(temp.get_frequency(-1.0), 415.3046975799451, 0.0001));
        assert!(approx(temp.get_frequency(3.0), 523.2511306011972, 0.0001));
    }
}
