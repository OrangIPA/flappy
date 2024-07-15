use bevy::prelude::*;
use rand::Rng;
use songosewelas::{collide_aabb, AABB};

use crate::{plane::{self, Plane}, GameState, SCALE};

const WIDTH: f32 = 30.;
const HEIGHT: f32 = 254.;

const SPACING: f32 = 200.;
const SPEED: f32 = 100.;
const SPAWN_PERIOD: f32 = 3.6;

pub struct BuildingPlugin;
impl Plugin for BuildingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BuildingSpawn(Timer::from_seconds(
            SPAWN_PERIOD,
            TimerMode::Repeating,
        )))
        .insert_resource(JustStarted(false))
        .add_systems(Update, update)
        .add_systems(Update, spawn_building);
    }
}

#[derive(Component)]
struct Building;

#[derive(Resource)]
struct BuildingSpawn(Timer);

#[derive(Resource)]
struct JustStarted(bool);

fn update(
    mut commands: Commands,
    time: Res<Time>,
    mut is_play: ResMut<GameState>,
    mut building_query: Query<(Entity, &mut Transform), With<Building>>,
    plane_query: Query<&Transform, (With<Plane>, Without<Building>)>,
    window_query: Query<&Window>,
) {
    if *is_play != GameState::Play {
        return;
    }

    let window = window_query.single();
    let plane = plane_query.single();

    let plane_aabb = AABB {
        min: (
            plane.translation.x - plane::WIDTH * SCALE / 2.,
            plane.translation.y - plane::HEIGHT * SCALE / 2.,
        ),
        max: (
            plane.translation.x + plane::WIDTH * SCALE / 2.,
            plane.translation.y + plane::HEIGHT * SCALE / 2.,
        ),
    };

    for mut building in building_query.iter_mut() {
        building.1.translation.x -= SPEED * time.delta_seconds();
        if building.1.translation.x < -window.width() {
            commands.entity(building.0).despawn();
        }

        let building_aabb = AABB {
            min: (
                building.1.translation.x - WIDTH * SCALE / 2.,
                building.1.translation.y - HEIGHT * SCALE / 2.,
            ),
            max: (
                building.1.translation.x + WIDTH * SCALE / 2.,
                building.1.translation.y + HEIGHT * SCALE / 2.,
            ),
        };

        if collide_aabb(&plane_aabb, &building_aabb) {
            *is_play = GameState::GameOver;
        }
    }
}

fn spawn_building(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    is_play: Res<GameState>,
    mut timer: ResMut<BuildingSpawn>,
    mut just_started: ResMut<JustStarted>,
    query: Query<&Window>,
) {
    if *is_play != GameState::Play {
        timer.0.reset();
        *just_started = JustStarted(true);
        return;
    }

    let should_spawn = timer.0.tick(time.delta()).just_finished() || just_started.0;

    if just_started.0 {
        *just_started = JustStarted(false);
    }

    let window = query.single();

    if should_spawn {
        let mut rng = rand::thread_rng();
        let mut offset = (rng.gen::<f32>() * 80.).round() * SCALE;
        if rand::random() {
            offset = -offset;
        }

        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("gedung.png"),
                transform: Transform::from_translation(Vec3::new(
                    window.width() / 2. + WIDTH * SCALE / 2.,
                    -(HEIGHT * SCALE / 2. + SPACING / 2. + offset),
                    0.,
                ))
                .with_scale(Vec3::new(SCALE, SCALE, 1.)),
                ..default()
            },
            Building,
        ));

        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("gedung.png"),
                transform: Transform::from_translation(Vec3::new(
                    window.width() / 2. + WIDTH * SCALE / 2.,
                    HEIGHT * SCALE / 2. + SPACING / 2. - offset,
                    0.,
                ))
                .with_scale(Vec3::new(SCALE, -SCALE, 1.)),
                ..default()
            },
            Building,
        ));
    }
}
