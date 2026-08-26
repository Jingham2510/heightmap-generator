use rustgeomapping::data_types::heightmap::Heightmap;
use std::collections::HashMap;

use ratatui::{
    style::Color,
    widgets::canvas::{Painter, Shape},
};

use crate::pathgen::{DetectionMode, PathGenMode};

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

        String::from(deform_str)
    }
}

///Core settings for creating deformations (i.e. every shape requires it)
pub struct CoreSettings {
    pub deform_type: DeformType,
    pub deform_center: [f64; 2],
    pub deform_rotation: f64,
    pub deform_thickness: f64,
    pub deform_depth: f64,
}

//Default settings for the core deofmration settings
impl Default for CoreSettings {
    fn default() -> Self {
        CoreSettings {
            deform_type: DeformType::default(),
            deform_center: [500.0, 500.0],
            deform_rotation: 0.0,
            deform_thickness: 50.0,
            deform_depth: 400.0,
        }
    }
}

///Custom heightmap shape
pub struct HeightmapShape<'a> {
    // Precomputed once: (x, y, color) per cell
    pub cells: &'a [(f64, f64, Color)],
}
impl<'a> Shape for HeightmapShape<'a> {
    fn draw(&self, painter: &mut Painter) {
        for &(x, y, color) in self.cells {
            if let Some((px, py)) = painter.get_point(x, y) {
                painter.paint(px, py, color);
            }
        }
    }
}


///Information and settings required for creating heightmap deformations
#[derive(Default)]
pub struct hmap_gen_info{
    ///Deformation setup info and associated variables
    pub deform_settings: HashMap<String, f64>,

    ///Deformation position and rotation (degrees)
    pub core_settings: CoreSettings,

    pub overlay_on: bool,



    ///Currently loaded heightmap
    pub hmap_loaded: bool,
    pub loaded_hmap: Heightmap,
    pub hmap_min: f32,
    pub hmap_max: f32,
    pub hmap_fp: String,
    pub hmap_cells: Vec<(f64, f64, Color)>,

    //Generated deformation
    pub generated_hmap: Heightmap,
    pub generated_cells: Vec<(f64, f64, Color)>,
    pub auto_generate: bool,
}


///Information and settings required for generation trajectory paths
#[derive(Default)]
pub struct path_gen_info{
    ///Current heightmap filepath
    pub current_map_fp : String,
    ///Target heightmap filepath
    pub target_map_fp : String,

    ///Current heightmap
    pub current_map : Heightmap,
    /// Target heightmap
    pub target_map : Heightmap,

    ///Difference map (i.e. the difference between the target and the current map)
    pub diff_map_generated : bool,
    pub difference_map : Heightmap,
    pub difference_width : usize,
    pub difference_height : usize,
    pub diff_map_cells: Vec<(f64, f64, Color)>,


    ///Detection mode
    pub detect_mode : DetectionMode,
    pub detect_info : HashMap<String, f64>,

    ///Path generation mode
    pub path_mode : PathGenMode,
    pub path_info : HashMap<String, f64>,

}


///Number of tab pages in the application
 pub const NO_OF_TABS : usize = 1;


/// Application.
#[derive(Default)]
pub struct App {
    /// should the application exit?
    pub should_quit: bool,

    ///Tab selection
    pub tab_no : usize,

    ///Current CLI input
    pub curr_error: String,
    pub curr_input: String,

    ///Heightmap generation information
    pub hmap_gen_info : hmap_gen_info,

    ///Earthmoving path generation information
    pub path_gen_info : path_gen_info,   


  
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
