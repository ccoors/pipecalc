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

extern crate pipecalc;

use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

use pipecalc::*;

fn main() {
    let path = Path::new("pipes.csv");
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why.description()),
        Ok(file) => file,
    };

    file.write_all(b"Octave,Note,\"Note name\",Frequency,\"Resonator length\",\"Theoretical length\",\"Mouth height\",\"Mouth width\",\"Pipe depth\",\"Jet thickness\",\"Air consumption rate\"\n").expect("Could not write");

    let temp = Temperament::new_default_equal();
    let mm_h2o = 200.0;
    let pressure = pipecalc::mmH2O_to_pa(mm_h2o);

    for octave in -1..7 {
        for note in 0..12 {
            let pitch = Pitch::from(note, octave);
            let steps = pitch.difference_to(&temp.get_base_note());
            let frequency = temp.get_frequency(steps as f64);

            let mut pipe = Pipe::new();
            pipe.set_frequency(frequency)
                .set_stopped(false)
                .set_intonation_number(2.0)
                .set_mouth_ratio(0.275)
                .set_cutup_ratio(0.3)
                .set_radius(pitch.get_tnm_radius())
                .set_air_temperature(20.0)
                .set_air_density(1.2)
                .set_wind_pressure(pressure);

            let dimensions = pipe.get_dimensions();

            let note_name = match note {
                0 => "C",
                1 => "C#",
                2 => "D",
                3 => "D#",
                4 => "E",
                5 => "F",
                6 => "F#",
                7 => "G",
                8 => "G#",
                9 => "A",
                10 => "Bb",
                11 => "B",
                _ => panic!("Invalid note number"),
            };

            file.write_fmt(format_args!(
                "{},{},{}{},{},{},{},{},{},{},{},{}\n",
                octave,
                note,
                note_name,
                octave,
                frequency,
                dimensions.resonator_length,
                dimensions.theoretical_resonator_length,
                dimensions.mouth_height,
                dimensions.mouth_width,
                dimensions.pipe_depth,
                dimensions.jet_thickness,
                dimensions.air_consumption_rate
            )).expect("Could not write");
        }
    }
}
