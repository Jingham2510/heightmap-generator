use crate::trajectorygen::types::Point;
use crate::trajectorygen::{DetectionMode, PathGenMode, pointgen};
///App updating related to path/trajectory generation
use crate::update::update_shared::{calc_cell_colour, safe_str_to_f64};
use crate::trajectorygen::{edgedetection, types::Direction};




use crate::{
    app::{App}
};
use ratatui::style::Color;
use rustgeomapping::data_types::heightmap::Heightmap;
use rustgeomapping::analysis::analyser::comp_maps;

use std::{env,};

use anyhow::bail;


///Parse the user command related to the heightmap generation
pub fn path_gen_parse_input(app: &mut App, cmd_var : Vec<&str>) -> Result<(), anyhow::Error>{

    //Split the command and variables to the important bits
    let (cmd, var) = (
        cmd_var[0].to_ascii_lowercase(),
        cmd_var[1].to_ascii_lowercase(),
    );
    let opt_var = if cmd_var.len() == 3 {
        cmd_var[2].to_ascii_lowercase()
    } else {
        String::from("none")
    };

   
    match cmd.as_str() {

        //Map loading settings
        "load" =>{
             //Set the value of interest
            match var.as_str() {
                "current" => {
                     //See if a heightmap can be loaded from the filepath
                    let path = format!("{}{}", env::current_dir().unwrap().display(), opt_var);
                    let hmap_result = Heightmap::create_from_file(path);
                    match hmap_result {
                        Ok(hmap) => {
                            app.path_gen_info.current_map = hmap;
                            app.path_gen_info.current_map_fp= opt_var;

                        }
                        Err(_e) => {
                        app.curr_error = String::from("Invalid heightmap filepath");
                        bail!("cmd error")
                        }
                    }            
                
                }
                "target" => {
                     //See if a heightmap can be loaded from the filepath
                    let path = format!("{}{}", env::current_dir().unwrap().display(), opt_var);
                    let hmap_result = Heightmap::create_from_file(path);
                    match hmap_result {
                        Ok(hmap) => {
                            app.path_gen_info.target_map = hmap;
                            app.path_gen_info.target_map_fp= opt_var;

                        }
                        Err(_e) => {
                        app.curr_error = String::from("Invalid heightmap filepath");
                        bail!("cmd error")
                    }
                    }
                },
         
            

            _ => {
                app.curr_error = String::from("Invalid load target");
                bail!("cmd error")
            }
            }
        }

        //Seutp the bits and bobs
        "set" =>{
            match var.as_str(){
                //Set the detection mode
                "detmode" =>{
                    match opt_var.as_str() {

                        "simple" => {
                            app.path_gen_info.detect_mode = DetectionMode::SIMPLE;
                            app.path_gen_info.detect_info = DetectionMode::SIMPLE.get_default_settings();
                        }

                        "scattershot" =>{
                            app.path_gen_info.detect_mode = DetectionMode::SCATTERSHOT;
                            app.path_gen_info.detect_info = DetectionMode::SCATTERSHOT.get_default_settings();
                        }

                        "voronoi" =>{
                            app.path_gen_info.detect_mode = DetectionMode::VORONOI;
                            app.path_gen_info.detect_info = DetectionMode::VORONOI.get_default_settings();
                        }

                        _ => {
                            app.curr_error = String::from("invalid detection mode");
                            bail!("Invalid detection mode");
                        }
                    }
                }


                //Check to see if its a mode option
                _ => {
                    //See if it is a setting in the shapes setting hashmap
                    if app.path_gen_info.detect_info.contains_key(&var) {
                        let new_val = safe_str_to_f64(app, opt_var);
                        if new_val.is_ok() {
                            app.path_gen_info.detect_info
                                .entry(var)
                                .insert_entry(new_val.unwrap());
                        } else {
                            bail!("cmd error")
                        }
                    }else if app.path_gen_info.path_info.contains_key(&var) {
                        let new_val = safe_str_to_f64(app, opt_var);
                        if new_val.is_ok() {
                            app.path_gen_info.path_info
                                .entry(var)
                                .insert_entry(new_val.unwrap());
                        } else {
                            bail!("cmd error")
                        }
                    } 
                    else {
                        app.curr_error = String::from("Invalid set option!");
                        bail!("cmd error")
                    }
                }
            }
        }



        //Generate things (difference map, points, path etc)
        "generate" => {

            match var.as_str() {

                //Create the difference map between the provided maps
                "difference" =>{
                    create_diff_map(app)?;                                    

                }

                //Generate the points for the trajectory
                "points" =>{
                    if !app.path_gen_info.diff_map_generated{
                        app.curr_error = String::from("No difference map generated");
                        bail!("no diff map")
                    }

                    //Detect the deformation shapes and generate the inner points
                    match app.path_gen_info.detect_mode{
                        DetectionMode::SIMPLE =>{
                            //Create the edge shapes
                            app.path_gen_info.detected_shapes = vec![edgedetection::simple(&app.path_gen_info.difference_map)];

                            //Render the edge shapes
                            app.path_gen_info.edge_cells = render_edge_shapes(app);

                            //Generate the points
                            app.path_gen_info.generated_points = pointgen::simple(&app.path_gen_info);
                            //Create the points to render
                            app.path_gen_info.point_cells = render_waypoint_cells(&app.path_gen_info.generated_points);                      


                        }

                        DetectionMode::SCATTERSHOT =>{

                            //Create the edge shapes
                            app.path_gen_info.detected_shapes = vec![edgedetection::simple(&app.path_gen_info.difference_map)];

                            //Render the edge shapes
                            app.path_gen_info.edge_cells = render_edge_shapes(app);

                            //Generate the points
                            app.path_gen_info.generated_points = pointgen::scattershot(&app.path_gen_info);
                            //Create the points to render
                            app.path_gen_info.point_cells = render_waypoint_cells(&app.path_gen_info.generated_points);      

                        }

                        DetectionMode::VORONOI =>{
                            //Create the edge shapes
                            app.path_gen_info.detected_shapes = vec![edgedetection::simple(&app.path_gen_info.difference_map)];

                            //Create the edge shapes
                            app.path_gen_info.edge_cells = render_edge_shapes(app);

                            //Generate the points
                            app.path_gen_info.generated_points = pointgen::voronoi(&app.path_gen_info)
                            
                        }

                    }           




                }

                //Generate the path from the created points
                "path" =>{

                    match app.path_gen_info.path_mode{
                        PathGenMode::TESTING =>{
                            
                        }
                    }
                }

                _=>{
                    app.curr_error = String::from("Invalid generate option");
                    bail!("cmd error")
                }
            }

        }

        //Save functions (Save difference map/points/trajectory)
        "save" =>{

            match var.as_str() {
                
                //Save the difference map
                "difference" =>{
                    let path = format!("{}{}", env::current_dir().unwrap().display(), opt_var);
                    let result = app.path_gen_info.difference_map.save_to_file(&path);
                    match result {
                        Ok(_good) => {}
                        Err(_e) => {
                            app.curr_error = String::from("Failed to save heightmap");
                            bail!("cmd error")
                        }
                    }

                }

                //Create heightmaps from the difference, edge, and points and save them
                //Then use a python script to create (better rendered) versions
                "debug" =>{debug_save(app)?},

                _ =>{app.curr_error = String::from("Invalid save option");
                    bail!("cmd error")
                    }
            }

        }

        _ =>{
            app.curr_error = String::from("Unrecognised command!");
            bail!("cmd error")
        }
    }

    Ok(())


}

fn create_diff_map(app : &mut App) -> Result<(), anyhow::Error>{

    //Check that two maps are loaded
    if app.path_gen_info.current_map_fp.is_empty() || app.path_gen_info.target_map_fp.is_empty(){
        app.curr_error = String::from("Load both maps first!");
        bail!("cmd error")
    }

    //Update the heightmap that represents the difference between the target and the current map
    let result = comp_maps(&app.path_gen_info.current_map, &app.path_gen_info.target_map);   


    
    match result{
        Ok(map) =>{
            //Load the map into the path generate structure
            app.path_gen_info.difference_map = map.clone();

            app.path_gen_info.difference_width = app.path_gen_info.difference_map.width();
            app.path_gen_info.difference_height = app.path_gen_info.difference_map.height();



            //Clear the current cell colours for the canvas
            app.path_gen_info.diff_map_cells.clear();

            let max = app.path_gen_info.difference_map.get_max();
            let min = app.path_gen_info.difference_map.get_min();

            let mut row_cnt = 0.0;
            let mut col_cnt = 0.0;
            //Heightmap drawing function - reverse to draw and match pyplot
            for row in app.path_gen_info.difference_map.cells().into_iter().rev() {
                for cell in row {
                    
                    let cell_colour = calc_cell_colour(max, min, &cell, 1);

                    app.path_gen_info.diff_map_cells.push((col_cnt, row_cnt, cell_colour));
                    col_cnt += 1.0;
                }
                col_cnt = 0.0;
                row_cnt += 1.0;
            }                            

            //Flag that a difference map has been generated
            app.path_gen_info.diff_map_generated = true;
            Ok(())
        }
        Err(e) =>{
            app.curr_error = format!("Failed to compare maps - {}", e);
            bail!(e)
            }


    }
}


//Go through each edge and determine where to draw the lines
fn render_edge_shapes(app : &App) -> Vec<(f64, f64, Color)>{

    const DRAW_CENTROID : bool = true;

    let mut cells : Vec<(f64, f64, Color)> = vec![];

    //Go through each shape and draw the edge (and the centroid)
    for shape in &app.path_gen_info.detected_shapes{

        //For each edge determine which parts to add to the edge cells
        for edge in shape.edges(){ 
            let (mut y,  x) = edge.pos_f64();
            //remembering that its top to bottom for cell rendering
            y = app.path_gen_info.difference_height as f64 - y;

            for dir in edge.dir(){
                match dir{                
                    Direction::NORTH =>{
                        cells.push((x, y - 0.5, Color::Black));
                    }
                    
                    Direction::NORTHEAST =>{
                        cells.push((x + 0.5, y - 0.5, Color::Black));
                    }
                    Direction::EAST =>{
                        cells.push((x + 0.5, y, Color::Black));
                    }
                    Direction::SOUTHEAST =>{
                        cells.push((x + 0.5, y + 0.5, Color::Black));
                    }
                    Direction::SOUTH =>{
                        cells.push((x, y + 0.5, Color::Black));
                    }
                    Direction::SOUTHWEST =>{
                        cells.push((x - 0.5, y + 0.5, Color::Black));
                    }
                    Direction::WEST =>{
                        cells.push((x - 0.5, y, Color::Black));
                    }
                    Direction::NORTHWEST =>{
                        cells.push((x - 0.5, y - 0.5, Color::Black));
                    }                    
                }
            }
        }


        if DRAW_CENTROID{

            //Create the centroid marker
            let (cent_y, cent_x) = shape.centre().as_xy_f64();

            //cent_y = app.path_gen_info.difference_height as f64 - cent_y;

            cells.push((cent_x - 0.25, cent_y - 0.25, Color::LightCyan));
            cells.push((cent_x + 0.25, cent_y - 0.25, Color::LightCyan));
            cells.push((cent_x - 0.25, cent_y + 0.25, Color::LightCyan));
            cells.push((cent_x+ 0.25, cent_y + 0.25, Color::LightCyan));
        }


    }

    cells

}

///Create the waypoint drawing cells
fn render_waypoint_cells(points : &Vec<Point>) -> Vec<(f64, f64, Color)>{

    //Magic number for now 
    let max = 1000.0;

    let mut cells :Vec<(f64, f64, Color)> = vec![];

    for point in points.iter().rev(){
        cells.push((point.y() as f64, max - point.x() as f64, Color::Black))
    }

    cells

}


///Save all the debug heightmaps 
fn debug_save(app : &mut App) -> Result<(), anyhow::Error>{

    //Check if the difference map exists
    if app.path_gen_info.diff_map_generated{
        let path = format!("{}/debug_out/difference_map", env::current_dir().unwrap().display());
        let result = app.path_gen_info.difference_map.save_to_file(&path);
        match result {
            Ok(_good) => {}
            Err(_e) => {
                app.curr_error = String::from("Failed to save heightmap");
                bail!("cmd error")
            }
        }
    }

    //Create local maps of the shapes
    if !app.path_gen_info.detected_shapes.is_empty(){

        let mut cnt = 0;

        //For each shape create a heightmap that matches the size of the rectangle bounds (+ a little bit)
        for shape in &app.path_gen_info.detected_shapes{


            //Create the heightmap
            let mut shape_map = Heightmap::new(shape.max().x() - shape.min().x() + 1, shape.max().y() - shape.min().y() + 1);

            for edge in shape.edges(){

                let _ = shape_map.set_cell_height(edge.x() - shape.min().x(),edge.y() - shape.min().y(), 100.0);

            }

            //Format the save string and save the edgemap
           let path = format!("{}/debug_out/shape_{}", env::current_dir().unwrap().display(), cnt);
            let result = shape_map.save_to_file(&path);
            match result {
                Ok(_good) => {
                    cnt += 1;
                }
                Err(_e) => {
                    app.curr_error = String::from("Failed to save edgemap");
                    bail!("cmd error")
                }
            }
        }
    }

    //Create a global view of the map with all shapes and points       
    if !app.path_gen_info.generated_points.is_empty(){
        const MAKE_EDGES : bool = false;
       
        //Create an empty heightmap
        let mut point_map = Heightmap::new(app.path_gen_info.difference_map.width(), app.path_gen_info.difference_map.height());


        //Place all the edges in it
        if MAKE_EDGES{
            for shape in &app.path_gen_info.detected_shapes{
            for edge in shape.edges(){
                    let _ = point_map.set_cell_height(edge.x(),edge.y(), 100.0);
                }
            }
        }

        //Place all the points (without the heights)
        for point in &app.path_gen_info.generated_points{
            let _ = point_map.set_cell_height(point.x(), point.y(), 100.0);
        }

        //Format the save string and save the pointmap
        //Format the save string and save the edgemap
            let path = format!("{}/debug_out/waypoints", env::current_dir().unwrap().display());
            let result = point_map.save_to_file(&path);
            match result {
                Ok(_good) => {}
                Err(_e) => {
                    app.curr_error = String::from("Failed to save waypoint map");
                    bail!("cmd error")
                }
            }

    }



    Ok(())

}