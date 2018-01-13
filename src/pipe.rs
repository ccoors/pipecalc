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

use util;

pub struct Pipe {
    stopped: bool,
    frequency: f64,
    intonation_number: f64,
    radius: f64,
    mouth_ratio: f64,
    cutup_ratio: f64,
    air_temperature: f64,
    air_density: f64,
    wind_pressure: f64,
}

#[derive(Debug)]
pub struct PipeProperties {
    // in m
    pub resonator_length: f64,

    // in m
    pub theoretical_resonator_length: f64,

    // in m
    pub mouth_height: f64,

    // in m
    pub mouth_width: f64,

    // in m^2
    pub mouth_area: f64,

    // in m/s
    pub air_speed: f64,

    // in m^3/s
    pub air_consumption_rate: f64,

    // in m
    pub jet_thickness: f64,

    // in m^2
    pub cross_section: f64,

    // in m
    pub circumference: f64,

    // in m
    pub pipe_depth: f64,

    // in W
    pub sound_power: f64,

    // in m
    pub air_hole_diameter: f64,
}

impl Pipe {
    pub fn new() -> Pipe {
        Pipe {
            stopped: false,
            frequency: 0.0,
            intonation_number: 2.0,
            radius: 0.0,
            mouth_ratio: 0.25,
            cutup_ratio: 0.0,
            air_temperature: 20.0,
            air_density: 0.0,
            wind_pressure: 0.0,
        }
    }

    pub fn set_frequency(&mut self, frequency: f64) -> &mut Self {
        self.frequency = frequency;
        self
    }

    pub fn set_radius(&mut self, radius: f64) -> &mut Self {
        self.radius = radius;
        self
    }

    pub fn set_mouth_ratio(&mut self, mouth_ratio: f64) -> &mut Self {
        self.mouth_ratio = mouth_ratio;
        self
    }

    pub fn set_cutup_ratio(&mut self, cutup_ratio: f64) -> &mut Self {
        self.cutup_ratio = cutup_ratio;
        self
    }

    pub fn set_stopped(&mut self, stopped: bool) -> &mut Self {
        self.stopped = stopped;
        self
    }

    pub fn set_intonation_number(&mut self, intonation_number: f64) -> &mut Self {
        self.intonation_number = intonation_number;
        self
    }

    pub fn set_air_temperature(&mut self, air_temperature: f64) -> &mut Self {
        self.air_temperature = air_temperature;
        self
    }

    pub fn set_air_density(&mut self, air_density: f64) -> &mut Self {
        self.air_density = air_density;
        self
    }

    pub fn set_wind_pressure(&mut self, wind_pressure: f64) -> &mut Self {
        self.wind_pressure = wind_pressure;
        self
    }

    // air_temperature in C, air_density in kg/m^3, wind_pressure in Pa
    pub fn get_dimensions(&self) -> PipeProperties {
        let speed_of_sound = util::speed_of_sound(self.air_temperature); // in m/s
        let air_speed = util::air_speed(self.wind_pressure, self.air_density); // in m/s

        let i = self.intonation_number;
        let f = self.frequency; // in Hz

        let cross_section = f64::consts::PI * self.radius.powi(2);
        let circumference = f64::consts::PI * self.radius * 2.0;
        let mouth_width = circumference * self.mouth_ratio;
        let mouth_height = mouth_width * self.cutup_ratio;
        let mouth_area = mouth_height * mouth_width;
        let pipe_depth = cross_section / mouth_width;

        let jet_thickness =
            0.001 * (f.powi(2) * i.powi(2) * (10.0 * mouth_height).powi(3)) / air_speed.powi(2);

        let sound_power = 0.001 * f64::consts::PI * (self.air_density / speed_of_sound) * f.powi(2)
            * (1.7 * (jet_thickness * speed_of_sound * f * mouth_area * mouth_area.sqrt()).sqrt())
                .powi(2);

        let air_consumption_rate = air_speed * mouth_width * jet_thickness;

        let wavelength = speed_of_sound / f;
        let theoretical_resonator_length = if self.stopped {
            wavelength / 4.0
        } else {
            wavelength / 2.0
        };

        let resonator_length = if self.stopped {
            (-0.73 * (f * cross_section - 0.342466 * speed_of_sound * mouth_area.sqrt()))
                / (f * mouth_area.sqrt())
        } else {
            (-0.73
                * (f * cross_section + 0.465753 * f * mouth_area.sqrt() * cross_section.sqrt()
                    - 0.684932 * speed_of_sound * mouth_area.sqrt()))
                / (f * mouth_area.sqrt())
        };

        let air_hole_diameter =
            2.0 * ((mouth_width * jet_thickness * 10.0).sqrt() / f64::consts::PI);

        PipeProperties {
            resonator_length,
            theoretical_resonator_length,
            mouth_height,
            mouth_width,
            mouth_area,
            air_speed,
            air_consumption_rate,
            jet_thickness,
            cross_section,
            circumference,
            pipe_depth,
            sound_power,
            air_hole_diameter,
        }
    }
}

#[cfg(test)]
mod tests {
    use temperament::Temperament;

    use super::*;
    use super::super::*;
    use super::super::util::*;

    #[allow(non_snake_case)]
    #[test]
    fn test_simple_pipe() {
        let temp = Temperament::new_default_equal();
        let pitch = Pitch::from(0, 5);
        let steps = pitch.difference_to(&temp.get_base_note());
        assert_eq!(steps, 3.0);
        let frequency = temp.get_frequency(steps); // a' + 3 = c''

        let mmH2O = 60.0;
        let pressure = util::mmH2O_to_pa(mmH2O);

        let radius = pitch.get_tnm_radius();
        let mut pipe = Pipe::new();
        pipe.set_frequency(frequency);
        pipe.set_stopped(false);
        pipe.set_intonation_number(2.0);
        pipe.set_mouth_ratio(0.25);
        pipe.set_cutup_ratio(0.3);
        pipe.set_radius(radius);
        pipe.set_air_temperature(20.0);
        pipe.set_air_density(1.2);
        pipe.set_wind_pressure(pressure);

        let dimensions = pipe.get_dimensions();
        assert!(approx(dimensions.resonator_length, 0.275, 0.01));
        assert!(approx(dimensions.theoretical_resonator_length, 0.328, 0.01));
        assert!(approx(dimensions.mouth_height, 0.00766, 0.001));
        assert!(approx(dimensions.mouth_width, 0.025, 0.01));
        assert!(approx(dimensions.pipe_depth, 0.0325, 0.01));
        assert!(approx(dimensions.mouth_area, 0.00019573, 0.0001));
        assert!(approx(dimensions.air_speed, 31.3155, 0.01));
        assert!(approx(
            dimensions.air_consumption_rate,
            0.000401943302,
            0.00001,
        ));
        assert!(approx(dimensions.jet_thickness, 0.0005024, 0.00001));
        assert!(approx(dimensions.cross_section, 0.000830712, 0.00001));
        assert!(approx(dimensions.circumference, 0.102171, 0.001));
    }
}
