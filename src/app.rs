use std::collections::HashMap;
use rustgeomapping::data_types::heightmap::Heightmap;

///Types of deformation
#[derive(Debug, Default)]
pub enum DeformType {
    #[default]
    NONE,
    LINE,
    CIRCLE,
    RECTANGLE,
}

impl DeformType {
    //Current valid types of iteration (ignoring NONE)
    pub const VARIANTS: [DeformType; 3] =
        [DeformType::LINE, DeformType::CIRCLE, DeformType::RECTANGLE];

    pub fn get_default_settings(&self) -> HashMap<String, f64> {
        match self {
            DeformType::LINE => HashMap::from([(String::from("length"), 500.0f64)]),
            DeformType::CIRCLE => HashMap::from([(String::from("radius"), 100.0f64)]),
            DeformType::RECTANGLE => HashMap::from([
                (String::from("width"), 500.0f64),
                (String::from("length"), 500.0f64),
            ]),
            _ => HashMap::from([(String::from("Shouldn't be accessible"), 00.0f64)]),
        }
    }
}

impl ToString for DeformType {
    fn to_string(&self) -> String {
        let deform_str = match self {
            DeformType::LINE => "Line",
            DeformType::CIRCLE => "Circle",
            DeformType::RECTANGLE => "Rectangle",
            _ => "None",
        };

        return String::from(deform_str);
    }
}

/// Application.
#[derive(Default)]
pub struct App {
    /// should the application exit?
    pub should_quit: bool,

    ///Deformation type
    pub deform_type: DeformType,
    ///Deformation setup info and associated variables
    pub deform_settings: HashMap<String, f64>,

    ///Deformation position and rotation (degrees)
    pub deform_center: [f64; 2],
    pub deform_rotation: f64,
    pub deform_thickness: f64,

    ///Current CLI input
    pub curr_error: String,
    pub curr_input: String,

    ///Currently loded heightmap
    pub loaded_hmap : Heightmap,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set should_quit to true to quit the application.
    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}
