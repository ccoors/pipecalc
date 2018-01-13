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

#![recursion_limit = "128"]

#[macro_use]
extern crate stdweb;
#[macro_use]
extern crate yew;

extern crate pipecalc;

use stdweb::unstable::TryInto;

use yew::html::*;

struct Context {}

#[derive(Default)]
struct Model {
    show: bool,
    air_pressure: f64,
    air_density: f64,
    temperature: f64,
    standard_pitch: f64,
    first_octave: i8,
    last_octave: i8,
    intonation_number: f64,
    mouth_ratio: f64,
    cutup_ratio: f64,
    tnm_distance: f64,
    stopped: bool,

    speed_of_sound: f64,
    air_speed: f64,
}

enum Msg {
    Calculate,
}

fn get_f64(id: &str, default: f64) -> f64 {
    let value = js! { return $(@{id}).val(); };
    let value: String = value.try_into().unwrap_or(String::new());
    value.parse().unwrap_or(default)
}

fn get_i8(id: &str, default: i8) -> i8 {
    let value = js! { return $(@{id}).val(); };
    let value: String = value.try_into().unwrap_or(String::new());
    value.parse().unwrap_or(default)
}

fn get_value(id: &str, default: &str) -> String {
    let value = js! { return $(@{id}).val(); };
    value.try_into().unwrap_or(String::from(default))
}

fn get_checked(id: &str, default: bool) -> bool {
    let value = js! { return $(@{id}).prop("checked"); };
    value.try_into().unwrap_or(default)
}

fn update(_context: &mut Context, model: &mut Model, msg: Msg) {
    match msg {
        Msg::Calculate => {
            let pressure = get_f64("#pressure", 0.0);
            let pressure_unit = get_value("#pressure_unit", "");
            let pressure = match pressure_unit.as_ref() {
                "mmh2o" => pipecalc::mmH2O_to_pa(pressure),
                "pa" => pressure,
                _ => 0.0,
            };

            let air_temperature = get_f64("#air_temperature", 0.0);
            let air_density = get_f64("#air_density", 0.0);
            let standard_pitch = get_f64("#standard_pitch", 0.0);

            let first_octave = get_i8("#first_octave", -10);
            let last_octave = get_i8("#last_octave", -10);

            let intonation_number = get_f64("#intonation_number", 0.0);
            let mouth_ratio = get_f64("#mouth_ratio", 0.0);
            let cutup_ratio = get_f64("#cutup_ratio", 0.0);
            let tnm_distance = get_f64("#tnm_distance", 0.0);

            let stopped = get_checked("#stopped", false);

            if pressure < 0.01 || air_temperature < -273.1 || air_density < 0.01
                || standard_pitch < 0.01 || first_octave > last_octave
                || first_octave < -3 || last_octave < -3 || intonation_number <= 0.0
                || mouth_ratio <= 0.0 || mouth_ratio > 1.0 || cutup_ratio <= 0.0
            {
                js! {
                    alert("Invalid input");
                }
            } else {
                model.show = true;
                model.air_pressure = pressure;
                model.temperature = air_temperature;
                model.air_density = air_density;
                model.standard_pitch = standard_pitch;
                model.intonation_number = intonation_number;
                model.mouth_ratio = mouth_ratio;
                model.cutup_ratio = cutup_ratio;
                model.first_octave = first_octave;
                model.last_octave = last_octave;
                model.tnm_distance = tnm_distance;
                model.stopped = stopped;

                model.speed_of_sound = pipecalc::speed_of_sound(model.temperature);
                model.air_speed = pipecalc::air_speed(model.air_pressure, model.air_density);
            }
        }
    }
}

fn add_header(
    title: &str,
    unit_html: &str,
    unit_csv: &str,
    html: &mut String,
    csv: &mut String,
    last: bool,
) {
    html.push_str(&format!(
        "<th>{} $\\left[{}\\right]$</th>",
        title, unit_html
    ));
    csv.push_str(&format!("\"{} [{}]\"", title, unit_csv));
    if !last {
        csv.push_str(",");
    }
}

fn add_entry(value: &str, html: &mut String, csv: &mut String, last: bool) {
    html.push_str(&format!("<td>{}</td>", value));
    if last {
        csv.push_str(value);
    } else {
        csv.push_str(&format!("{},", value));
    }
}

fn render_table(model: &Model) -> (String, String) {
    let mut html = String::new();
    let mut csv = String::new();
    html.push_str(
        r#"<table class="hover">
<thead>
<tr>
<th>Note</th>"#,
    );

    csv.push_str("\"Note\",");

    add_header("Frequency", "\\text{Hz}", "Hz", &mut html, &mut csv, false);
    add_header(
        "Theoretical resonator length",
        "\\text{mm}",
        "mm",
        &mut html,
        &mut csv,
        false,
    );
    add_header(
        "Actual resonator length",
        "\\text{mm}",
        "mm",
        &mut html,
        &mut csv,
        false,
    );
    add_header("Diameter", "\\text{mm}", "mm", &mut html, &mut csv, false);
    add_header(
        "Cross section",
        "\\text{mm}^2",
        "mm^2",
        &mut html,
        &mut csv,
        false,
    );
    add_header(
        "Cutup height",
        "\\text{mm}",
        "mm",
        &mut html,
        &mut csv,
        false,
    );
    add_header(
        "Mouth width",
        "\\text{mm}",
        "mm",
        &mut html,
        &mut csv,
        false,
    );
    add_header("Pipe depth", "\\text{mm}", "mm", &mut html, &mut csv, false);
    add_header(
        "Jet thickness",
        "\\text{mm}",
        "mm",
        &mut html,
        &mut csv,
        false,
    );
    add_header(
        "Minimum air hole diameter",
        "\\text{mm}",
        "mm",
        &mut html,
        &mut csv,
        false,
    );
    add_header(
        "Air consumption rate",
        "\\frac{\\text{m}^3}{\\text{s}}",
        "mm",
        &mut html,
        &mut csv,
        false,
    );
    add_header("Sound power", "\\text{W}", "W", &mut html, &mut csv, true);

    html.push_str(
        r#"</tr>
</thead>
<tbody>"#,
    );

    csv.push_str("\n");

    let temp = pipecalc::Temperament::new_freq_equal(model.standard_pitch);

    for octave in model.first_octave..(model.last_octave + 1) {
        for note in 0..12 {
            let pitch = pipecalc::Pitch::from(note, octave as i32);
            let tnm_pitch = pitch.add(-(model.tnm_distance * 100.0));
            let steps = pitch.difference_to(&temp.get_base_note());
            let frequency = temp.get_frequency(steps as f64);
            let radius = tnm_pitch.get_tnm_radius();

            let mut pipe = pipecalc::Pipe::new();
            pipe.set_frequency(frequency)
                .set_stopped(model.stopped)
                .set_intonation_number(model.intonation_number)
                .set_mouth_ratio(model.mouth_ratio)
                .set_cutup_ratio(model.cutup_ratio)
                .set_radius(radius)
                .set_air_temperature(model.temperature)
                .set_air_density(model.air_density)
                .set_wind_pressure(model.air_pressure);

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

            let mut formatted_note_name = String::from(r#"$\text{"#);
            formatted_note_name.push_str(&format!("{}", note_name));
            formatted_note_name.push_str("}_{");
            formatted_note_name.push_str(&format!("{}", octave));
            formatted_note_name.push_str("}$");

            html.push_str("<tr>");
            html.push_str(&format!("<td>{}</td>", formatted_note_name));
            csv.push_str(&format!("{}{},", note_name, octave));
            add_entry(&format!("{:.2}", frequency), &mut html, &mut csv, false);
            add_entry(
                &format!("{:.4}", dimensions.theoretical_resonator_length * 1000.0),
                &mut html,
                &mut csv,
                false,
            );
            add_entry(
                &format!("{:.4}", dimensions.resonator_length * 1000.0),
                &mut html,
                &mut csv,
                false,
            );
            add_entry(
                &format!("{:.4}", radius * 2.0 * 1000.0),
                &mut html,
                &mut csv,
                false,
            );
            add_entry(
                &format!("{:.4}", dimensions.cross_section * 1000.0 * 1000.0),
                &mut html,
                &mut csv,
                false,
            );
            add_entry(
                &format!("{:.4}", dimensions.mouth_height * 1000.0),
                &mut html,
                &mut csv,
                false,
            );
            add_entry(
                &format!("{:.4}", dimensions.mouth_width * 1000.0),
                &mut html,
                &mut csv,
                false,
            );
            add_entry(
                &format!("{:.4}", dimensions.pipe_depth * 1000.0),
                &mut html,
                &mut csv,
                false,
            );
            add_entry(
                &format!("{:.4}", dimensions.jet_thickness * 1000.0),
                &mut html,
                &mut csv,
                false,
            );
            add_entry(
                &format!("{:.4}", dimensions.air_hole_diameter * 1000.0),
                &mut html,
                &mut csv,
                false,
            );
            add_entry(
                &format!("{:.8}", dimensions.air_consumption_rate),
                &mut html,
                &mut csv,
                false,
            );
            add_entry(
                &format!("{:.8}\n", dimensions.sound_power),
                &mut html,
                &mut csv,
                true,
            );
            html.push_str("</tr>");
        }
    }

    html.push_str("</tbody></table>");
    (html, csv)
}

fn view(model: &Model) -> Html<Msg> {
    if model.show {
        let (html_table, csv) = render_table(model);

        js! {
            var csv_download = "data:text/csv;charset=utf-8," + encodeURIComponent(@{csv});
            $("#output-table").html(@{html_table});
            $("#output-csv-download").html("<a class=\"button success radius\" href=\"" + csv_download + "\" download=\"pipes.csv\">Download results as CSV</a>");
            MathJax.Hub.Queue(["Typeset",MathJax.Hub]);
        }

        html! {
            <div>
                <a class="button", class="primary", class="radius", onclick=|_| Msg::Calculate,>{"Calculate pipe dimensions"}</a>
                <br />
                <p>
                    {"Air pressure: "}{format!("{:.3}", model.air_pressure)}{" Pa"}<br />
                    {"Speed of sound at "}{model.temperature}{" °C: "}{format!("{:.3}", model.speed_of_sound)}{" m/s"}<br />
                    {"Speed of the air at the jet: "}{format!("{:.3}", model.air_speed)}{" m/s"}<br />
                </p>
            </div>
        }
    } else {
        html! {
            <div>
                <a class="button", class="primary", class="radius", onclick=|_| Msg::Calculate,>{"Calculate pipe dimensions"}</a>
            </div>
        }
    }
}

fn main() {
    yew::initialize();
    let mut app = App::new();

    let context = Context {};

    let model = Model::default();

    let mount_class = "mount-point";

    let mount_point = format!(".{}", mount_class);
    app.mount_to(&mount_point, context, model, update, view);
    yew::run_loop();
}
