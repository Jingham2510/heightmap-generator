///UI widgets used in heightmap generation


use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::Style,
    symbols,
    text::{Line, Span},
    widgets::{
        Block, Borders, Paragraph, Wrap,
        canvas::{Canvas},
    },
};

use crate::app::{App, DeformType, HeightmapShape};
use crate::ui::ui_shared;

///The main page of the heightmap generator
/// Consists of 4 widgets - Title, Control Info, Heightmap Renderer, CLI
pub fn main_page(app: &mut App, frame: &mut Frame, widget: Rect) {
    //Define the main window layout
    let outer_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(90),
            Constraint::Percentage(10),
        ])
        .split(widget);

    //Inner layout that displays the control panel and rendered heightmap
    let main_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(outer_layout[0]);

    //Control panel layout
    let cntrl_panel_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(main_layout[0]);


    //Determine the overlay line
    let overlay_line = if app.hmap_gen_info.overlay_on {
        Line::from("Display overlay: [X]")
    } else {
        Line::from("Display overlay: [ ]")
    };

    let auto_gen_line = if app.hmap_gen_info.auto_generate {
        Line::from("Auto generate: [X]")
    } else {
        Line::from("Auto generate: [ ]")
    };

    //Render the standard control panel
    let cntrl_panel_text = vec![
        Line::from(format!(
            "Pos X: {}       Pos Y: {}",
            app.hmap_gen_info.core_settings.deform_center[0], app.hmap_gen_info.core_settings.deform_center[1]
        )),
        Line::from(format!(
            " Rotation (degs): {}",
            app.hmap_gen_info.core_settings.deform_rotation
        )),
        Line::from(format!(
            "Thickness (mm): {}",
            app.hmap_gen_info.core_settings.deform_thickness
        )),
        Line::from(format!("Depth (mm): {}", app.hmap_gen_info.core_settings.deform_depth)),
        overlay_line,
        auto_gen_line,
    ];

    frame.render_widget(
        Paragraph::new(cntrl_panel_text)
            .block(
                Block::new()
                    .borders(Borders::ALL)
                    .title(Line::from(Span::styled(
                        "Shape positioning",
                        Style::new().bold(),
                    ))),
            )
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true }),
        cntrl_panel_layout[0],
    );

    //Render the deformation control panel
    match app.hmap_gen_info.core_settings.deform_type {
        DeformType::NONE => none_cntrl_panel(frame, cntrl_panel_layout[1]),
        DeformType::LINE | DeformType::CIRCLE | DeformType::RECTANGLE => {
            shape_cntrl_panel(app, frame, cntrl_panel_layout[1])
        }
    }

    //Render the heightmap
    if app.hmap_gen_info.hmap_loaded {
        render_heightmap(app, frame, main_layout[1])
    } else {
        frame.render_widget(
            Paragraph::new(Line::from(Span::styled(
                "Load a heightmap",
                Style::new().bold(),
            )))
            .block(Block::new().borders(Borders::ALL))
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true }),
            main_layout[1],
        );
    }

    ui_shared::render_CLI(app, frame, outer_layout[1])
   
}

///Control panel for telling the user to select a shape
fn none_cntrl_panel(frame: &mut Frame, widget: Rect) {
    let msg = Line::from("Select a shape using 'set shape [shape]'").centered();

    frame.render_widget(
        Paragraph::new(msg).block(Block::new().borders(Borders::ALL).title(Line::from(
            Span::styled("Shape settings", Style::new().bold()),
        ))),
        widget,
    );
}
///The control panel for creating a line
fn shape_cntrl_panel(app: &mut App, frame: &mut Frame, widget: Rect) {
    //Set the title
    let mut setting_lines = vec![
        Line::from(Span::styled(
            format!("Deformation: {}", app.hmap_gen_info.core_settings.deform_type.to_string()),
            Style::new().bold(),
        ))
        .centered(),
        Line::from(Span::styled(
            "Units are mm unless otherwise specified".to_string(),
            Style::new().bold(),
        ))
        .centered(),
    ];

    //Go through each entry in the hashmap and add it as a line to the paragraph to be rendered
    for (setting, value) in &app.hmap_gen_info.deform_settings {
        setting_lines.push(Line::from(format!("{}: {}", setting, value)))
    }

    frame.render_widget(
        Paragraph::new(setting_lines).block(Block::new().borders(Borders::ALL).title(Line::from(
            Span::styled("Shape settings", Style::new().bold()),
        ))),
        widget,
    )
}

///Render the current heightmap onto a canvas
fn render_heightmap(app: &mut App, frame: &mut Frame, widget: Rect) {
    //Create the heightmap layout widget
    let data_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(6), Constraint::Percentage(94)])
        .split(widget);

    //Display the heightmap data in mm
    frame.render_widget(
        Paragraph::new(Line::from(format!(
            "Stats: Min depth:{}mm Max depth:{}mm  Pixel height:{} Pixel Width:{}",
            app.hmap_gen_info.hmap_min,
            app.hmap_gen_info.hmap_max,
            app.hmap_gen_info.loaded_hmap.height() + 1,
            app.hmap_gen_info.loaded_hmap.width() + 1
        )))
        .block(Block::new().borders(Borders::ALL)),
        data_layout[0],
    );

    //Display the precreated points
    //This method of pre-calculating saves a lot of energy when refreshing the frame

    let canvas = Canvas::default()
        .block(Block::bordered().title("Current heightmap"))
        .x_bounds([0.0, app.hmap_gen_info.loaded_hmap.width() as f64])
        .y_bounds([0.0, app.hmap_gen_info.loaded_hmap.height() as f64])
        .marker(symbols::Marker::HalfBlock) //Half block displays the resolution the best
        .paint(|ctx| {
            ctx.draw(&HeightmapShape {
                cells: &app.hmap_gen_info.hmap_cells,
            });


            if app.hmap_gen_info.overlay_on {
                ctx.draw(&HeightmapShape {
                    cells: &app.hmap_gen_info.generated_cells,
                })
            }
        });

    //Create the canvas
    frame.render_widget(canvas, data_layout[1])
    
}


