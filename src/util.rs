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

#[allow(non_snake_case, dead_code)]
pub fn mmH2O_to_pa(mm_h2o: f64) -> f64 {
    9.80665 * mm_h2o
}

#[allow(dead_code)]
pub fn approx(to_test: f64, expect: f64, margin: f64) -> bool {
    let diff = to_test - expect;
    diff < margin && diff > -margin
}

pub fn speed_of_sound(air_temperature: f64) -> f64 {
    331.6 + 0.6 * air_temperature
}

pub fn air_speed(wind_pressure: f64, air_density: f64) -> f64 {
    (2.0 * (wind_pressure / air_density)).sqrt()
}
