mod plane;
mod building;

use bevy::prelude::*;

pub const SCALE: f32 = 3.;

#[derive(Resource, PartialEq)]
pub enum GameState {
    Play, NotPlay, GameOver
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgba(0.3, 0.5, 1., 1.)))
        .insert_resource(GameState::NotPlay)
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(plane::PlanePlugin)
        .add_plugins(building::BuildingPlugin)
        .add_systems(Startup, init)
        .run();
}

fn init(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

