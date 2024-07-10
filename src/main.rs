use crate::app::*;
use bevy::prelude::*;
mod app;


fn main() {
    let mut app = create_app();
    let add_camera_fun = |mut commands: Commands| {
        commands.spawn(Camera2dBundle::default());
    };
    app.add_systems(Startup, add_camera_fun);
    app.add_plugins(DefaultPlugins);

    app.run();
}
