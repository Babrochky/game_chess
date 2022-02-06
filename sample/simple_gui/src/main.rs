#![warn(missing_docs)]

//! Sample shows how to use Egui into Bevy window.

use bevy::render::pass::ClearColor;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};

const DISPLAY_HEIGHT : f32 = 300.0;
const DISPLAY_WIDTH : f32 = 300.0;

//

fn main()
{
  App::build()
  .insert_resource( ClearColor( Color::rgb( 0.04, 0.04, 0.04 ) ) )
  .insert_resource( WindowDescriptor
  {
    title : "Draw text".to_string(),
    width : DISPLAY_WIDTH,
    height : DISPLAY_HEIGHT,
    resizable : false,
    ..Default::default()
  })
  .add_plugins( DefaultPlugins )
  // add Egui plugin
  .add_plugin( EguiPlugin )
  .add_system( setup.system() )
  .run();
}

//

fn setup(egui_context : Res<EguiContext>)
{
  // initialize Egui window
  egui::Window::new("Menu").show(egui_context.ctx(), |ui| {
    // add labels inside Egui window
    let button = ui.button("Start Game");
    if button.clicked()
    {}
    ui.label("Game");
    ui.label("Options ");
  });
}
