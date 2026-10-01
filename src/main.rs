/*
By: Hassan Rana
Date: 2026-09-29
Program Details: A program that converts Meters and centimeters to feet and inches for a company that switches between Canada and the US.
*/

mod ui;
mod utils;

//use crate::ui::grid::draw_grid;
use crate::ui::label::Label;
use crate::ui::text_button::TextButton;
use crate::ui::text_input::TextInput;
use macroquad::prelude::*;
/// Set up window settings before the app runs
fn window_conf() -> Conf {
    Conf {
        window_title: "gui_metric_to_imperial".to_string(),
        window_width: 610,
        window_height: 768,
        fullscreen: false,
        high_dpi: true,
        window_resizable: true,
        sample_count: 4, // MSAA: makes shapes look smoother
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut lbl_inches = Label::new(
        "Input your meters or \ncentimeters in the appropreiate \ntext box and then hit \ncalculate to see the converted amount.",
        50.0,
        50.0,
        30,
    );
    let mut input_meters = TextInput::new(50.0, 200.0, 200.0, 50.0, 30.0);
    let mut input_centimeters = TextInput::new(350.0, 200.0, 230.0, 50.0, 30.0);
    let mut btn_exit = TextButton::new(400.0, 700.0, 150.0, 50.0, "EXIT", GRAY, RED, 30);
    let mut btn_calc = TextButton::new(50.0, 450.0, 150.0, 50.0, "CALCULATE", GRAY, GREEN, 30);
    let mut lbl_out = Label::new("Your converted amount will appear here.", 50.0, 325.0, 30);
    input_centimeters.set_prompt("Input Centimeters");
    input_meters.set_prompt("Input Meters");
    btn_exit.with_text_color(BLACK);
    btn_exit.with_hover_text_color(WHITE);
    btn_calc.with_text_color(BLACK);
    input_meters.set_max_chars(14).set_allowed_chars("0123456789.");
    input_centimeters.set_max_chars(16).set_allowed_chars("0123456789.");

    loop {
        clear_background(WHITE);
        //draw_grid(50.0, BROWN);
        if btn_exit.click() {
            break;
        }
        if btn_calc.click() {
            let feet = input_meters.get_text().parse::<f32>();
            let inches = input_centimeters.get_text().parse::<f32>();
            if let Ok(feet) = feet {
                if let Ok(inches) = inches {
                    lbl_out.set_text(format!("Feet: {:.3} \nInches: {:.3}", feet * 3.28084, inches * 0.393701));
                } else {
                    lbl_out.set_text("Invalid input. Please enter a valid number.");
                }
            } else {
                lbl_out.set_text("Invalid input. Please enter a valid number.");
            }
        }
        lbl_inches.draw();
        input_meters.draw();
        lbl_out.draw();
        input_centimeters.draw();
        next_frame().await;
    }
}
