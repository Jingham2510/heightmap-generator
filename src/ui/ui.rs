use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    style::{Color, Style},
    symbols,
    widgets::Tabs,
};

use crate::{
    app::App, ui::{ui_heightmap_gen, ui_pathmoving_gen}
    };



pub fn render(app: &mut App, frame: &mut Frame) {

    //Construct the page layout
    let layout = Layout::vertical([Constraint::Percentage(2), Constraint::Percentage(98)]);
    let [top, main] = frame.area().layout(&layout);

    //Render the tab titles
    let tabs = Tabs::new(vec!["Heightmap deformation", "Path generation"])
        .style(Color::White)
        .highlight_style(Style::default().magenta().on_black().bold())
        .select(app.tab_no)
        .divider(symbols::line::HEAVY_TRIPLE_DASH_VERTICAL)
        .padding(" ", " ");
    frame.render_widget(tabs, top);

    //Render the page based on the tab selection
    match app.tab_no{
       0 => ui_heightmap_gen::main_page(app, frame,main),
       1 => ui_pathmoving_gen::pathmoving_core(app, frame, main),

       other=>{panic!("Tab number {other} should be impossible!")}
    };
    
}
