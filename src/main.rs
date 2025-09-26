//! rAIce - punkt startowy (Bevy + Rapier 2D).
//! Uwaga: to wciąż “pre-clean” - pełny clean code dopiero po dowiezieniu feature’ów.

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod brain;
mod control;
mod evolution;
mod fitness;
mod sensors;
mod track;
mod vehicle;

use track::TrackSegment;
use vehicle::{grid_start_positions, spawn_vehicle};

/// Aktualnie aktywny tor (przechowuje też parametry szerokości itd.)
#[derive(Resource)]
struct ActiveTrack(track::Track);

/// --- Stałe projektu (łatwo zmieniać w jednym miejscu) ---
const PIXELS_PER_METER: f32 = 50.0;

const WINDOW_TITLE: &str = "rAIce";
const WINDOW_W: f32 = 1280.0;
const WINDOW_H: f32 = 720.0;

const CLEAR_COLOR: Color = Color::rgb(0.05, 0.05, 0.08);
const ROAD_COLOR: Color = Color::rgb(0.11, 0.11, 0.12);
const WALL_COLOR: Color = Color::rgb(0.38, 0.12, 0.12);

const CAMERA_SCALE: f32 = 0.8;
const CAMERA_Z: f32 = 999.0;

const NUM_VEHICLES: usize = 6;
const VEHICLES_PER_ROW: usize = 3;
const VEHICLE_SIZE: Vec2 = Vec2::new(32.0, 18.0);

fn main() {
    App::new()
        // tło
        .insert_resource(ClearColor(CLEAR_COLOR))
        // okno
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: WINDOW_TITLE.into(),
                    resolution: (WINDOW_W, WINDOW_H).into(),
                    // vsync przydatny do ładnego podglądu; do treningu wyłącz
                    present_mode: bevy::window::PresentMode::AutoVsync,
                    ..default()
                }),
                ..default()
            }),
        )
        // fizyka 2D (Rapier)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(
            PIXELS_PER_METER,
        ))
        // Debug kolizji - włącz, gdy chcesz podejrzeć collidery
        // .add_plugins(RapierDebugRenderPlugin::default())
        // scena startowa
        .add_systems(Startup, (setup_camera, setup_track, setup_vehicles).chain())
        .run();
}

/// Ustawienie kamery 2D (ortho) - lekko oddalone.
fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            projection: OrthographicProjection {
                scale: CAMERA_SCALE,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, CAMERA_Z),
            ..default()
        },
        Name::new("Main Camera"),
    ));
}

/// Budowa placeholderowego toru + resource ActiveTrack.
/// Generuje segmenty drogi i dwie ściany (L/R) na każdy segment.
fn setup_track(mut commands: Commands) {
    let track = track::placeholder_track();
    let segments = track.segments();

    let half_width = track.width * 0.5;
    let wall_half = track.wall_thickness * 0.5;

    for (index, segment) in segments.iter().enumerate() {
        spawn_track_segment(&mut commands, segment, track.width, ROAD_COLOR, index);

        // lewa/prawa ściana (offset wzdłuż normalnej i przeciwnie do niej)
        spawn_track_wall(
            &mut commands,
            segment,
            half_width,
            wall_half,
            WALL_COLOR,
            index,
            true,
        );
        spawn_track_wall(
            &mut commands,
            segment,
            half_width,
            wall_half,
            WALL_COLOR,
            index,
            false,
        );
    }

    commands.insert_resource(ActiveTrack(track));
}

/// Render prostokątnego “plastra” drogi dla danego segmentu.
fn spawn_track_segment(
    commands: &mut Commands,
    segment: &TrackSegment,
    width: f32,
    color: Color,
    index: usize,
) {
    // środek sprite’a w punkcie środkowym segmentu, obrót zgodny z kierunkiem segmentu
    let transform = Transform::from_translation(segment.mid.extend(0.0))
        .with_rotation(Quat::from_rotation_z(segment.angle));

    // lekki overshoot długości, żeby segmenty się zakrywały bez “szczelin”
    let size = Vec2::new(segment.length * 1.05, width);

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color,
                custom_size: Some(size),
                ..default()
            },
            transform,
            ..default()
        },
        Name::new(format!("Track Segment {:02}", index)),
    ));
}

/// Budowa ściany (collider + sprite) po lewej lub prawej stronie segmentu.
fn spawn_track_wall(
    commands: &mut Commands,
    segment: &TrackSegment,
    half_width: f32,
    wall_half: f32,
    color: Color,
    index: usize,
    left_side: bool,
) {
    let offset_dir = if left_side { segment.normal } else { -segment.normal };
    let label = if left_side { "L" } else { "R" };

    // pozycja środka ściany (od środka drogi + offset o pół szerokości drogi i pół grubości ściany)
    let translation = segment.mid + offset_dir * (half_width + wall_half);

    let transform = Transform::from_translation(translation.extend(0.1))
        .with_rotation(Quat::from_rotation_z(segment.angle));

    // sprite jako prostokąt - długość segmentu x grubość ściany
    let sprite_size = Vec2::new(segment.length, wall_half * 2.0);

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color,
                custom_size: Some(sprite_size),
                ..default()
            },
            transform,
            ..default()
        },
        // collider pasujący do sprite’a: pół-długość, pół-grubość
        RigidBody::Fixed,
        Collider::cuboid(segment.length * 0.5, wall_half),
        // trochę tarcia, minimalny “odskok”
        Friction::coefficient(1.1),
        Restitution::coefficient(0.05),
        Name::new(format!("Track Wall {} {:02}", label, index)),
    ));
}

/// Spawn siatki pojazdów w okolicy pozycji startowej toru.
fn setup_vehicles(mut commands: Commands, track: Res<ActiveTrack>) {
    let Some(start_pose) = track.0.start_pose() else {
        // brak startu - po prostu nic nie spawnujemy
        return;
    };

    // prosta siatka startowa - odległości liczone od szerokości toru
    let positions = grid_start_positions(
        start_pose,
        NUM_VEHICLES,
        VEHICLES_PER_ROW,
        track.0.width * 0.65, // odstęp wzdłuż toru
        track.0.width * 0.45, // odstęp wszerz
    );

    for (index, (position, heading)) in positions.into_iter().enumerate() {
        let transform = Transform::from_translation(Vec3::new(position.x, position.y, 0.5))
            .with_rotation(Quat::from_rotation_z(heading));

        // unikalny kolor dla każdego auta (HSL czyni cuda)
        let hue = (index as f32 * 55.0) % 360.0;
        let color = Color::hsl(hue, 0.75, 0.55);

        spawn_vehicle(
            &mut commands,
            transform,
            color,
            VEHICLE_SIZE,
            format!("Vehicle {:02}", index + 1),
        );
    }
}
