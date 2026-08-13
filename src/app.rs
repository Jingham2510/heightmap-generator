
use std::collections::HashMap;

///Types of deformation
#[derive(Debug, Default)]
pub enum DeformType{
    #[default]  NONE,
    LINE,
    CIRCLE,
    SQUARE
}

impl DeformType{
    //Current valid types of iteration (ignoring NONE)
    const VARIANTS: [DeformType; 3] = [
        DeformType::LINE,
        DeformType::CIRCLE,
        DeformType::SQUARE
    ]
}



/// Application.
#[derive(Debug, Default)]
pub struct App{

    /// should the application exit?
    pub should_quit: bool,
    
    ///Deformation type
    pub deform_type: DeformType,
    ///Deformation setup info and associated variables
    pub deform_settings : HashMap<&str, f64>,


    ///Deformation position and rotation (quaternion)
    pub deform_center : [f64; 2],
    pub deform_rotation : f64,



    ///Current CLI input
    pub curr_error : String,
    pub curr_input : String

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