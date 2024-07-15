use bevy::prelude::*;

use crate::{GameState, SCALE};

const GRAVITY: f32 = 3000.;
const FLAP_SPEED: f32 = 750.;

pub const WIDTH: f32 = 46.;
pub const HEIGHT: f32 = 30.;

pub struct PlanePlugin;
impl Plugin for PlanePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init).add_systems(Update, update);
    }
}

#[derive(Component, Default, Clone, Debug)]
pub struct Plane {
    pub speed: f32,
}

fn init(mut commands: Commands, asset_server: Res<AssetServer>, query: Query<&Window>) {
    let window = query.single();
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("pesawat.png"),
            transform: Transform::from_scale(Vec3::new(SCALE, SCALE, 1.)).with_translation(
                Vec3::new(-window.width() / 2. + WIDTH * SCALE + 80., 0., 0.),
            ),
            ..default()
        },
        Plane::default(),
    ));
}

fn update(
    mut query: Query<(&mut Transform, &mut Plane)>,
    time: Res<Time>,
    mut is_play: ResMut<GameState>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    let mut plane = query.single_mut();

    if keyboard.just_pressed(KeyCode::Space) {
        plane.1.speed = FLAP_SPEED;
        *is_play = GameState::Play;
    }

    if *is_play != GameState::Play {
        return;
    }

    plane.0.translation.y += plane.1.speed * time.delta_seconds();
    plane.1.speed -= GRAVITY * time.delta_seconds();

    plane.0.rotation =
        Quat::from_euler(EulerRot::XYZ, 0., 0., (plane.1.speed / 1000.).atan() / 1.5);
}
