///Shared updating methods


use anyhow::bail;


use crate::app::App;
use ratatui::style::Color;



///Attempts to parse user input into f64 -> if invalid prints the error to the console
pub fn safe_str_to_f64(app: &mut App, inp: String) -> Result<f64, anyhow::Error> {
    //Parse the result into an f64 or error
    let inp_f64: Result<f64, _> = inp.parse();

    if inp_f64.is_err() {
        app.curr_error = String::from("Cannot parse variable into a number");
        bail!("Cannot parse into f64!");
    } else {
        Ok(inp_f64.unwrap())
    }
}



///Calculate what colour the cell should be
pub fn calc_cell_colour(max : f32, min : f32, cell_val: &f32, mode :u8) -> Color {
    //Check to see if the cell height is known
    if cell_val.is_nan() {
        Color::Rgb(255, 255, 255)
    } else {

        if mode == 0{//Median

            let range = max - min;
            let median = min + range / 2.0;

            //Turn value into mm depth
            let cell_val = cell_val * 1000.0;

            //Calculate the cell value based on distance from the median
            let (r, g, b) = if cell_val <= median {
                (
                    255.0 * (1.0 - ((cell_val - min) / (median - min))),
                    255.0 * ((cell_val - min) / (median - min)),
                    0.0,
                )
            } else {
                (
                    0.0,
                    255.0 * (1.0 - ((cell_val - median) / (max - median))),
                    255.0 * ((cell_val - median) / (max - median)),
                )
            
            };
            Color::Rgb(r as u8, g as u8, b as u8)
        }else{//Distance from 0
            //If the cells are the same just paint it white
            if *cell_val == 0.0{
                 return Color::Rgb(255, 255, 255)
            }

            let max = max.abs();
            let min = min.abs();

            //Calculate the cell value based on distance from the median
            let (r, g, b) = if *cell_val < 0.0 {
                (
                    255.0 * (1.0 - ((cell_val - min) / (min))),
                    255.0 * ((cell_val - min) / (min)),
                    0.0,
                )
            } else {
                (
                    0.0,
                    255.0 * (1.0 - ((cell_val) / (max))),
                    255.0 * ((cell_val) / (max)),
                )
            
            };
            Color::Rgb(r as u8, g as u8, b as u8)

        }

        

    }
}
