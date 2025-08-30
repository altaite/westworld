mod creature;

use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::prelude::shape;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use creature::{Creature, Position};
use rand::prelude::*;

const NUM_CREATURES: usize = 20;

#[derive(Resource, Default)]
struct ComputationCounter(u32);

#[derive(Component)]
struct StatsText;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, FrameTimeDiagnosticsPlugin))
        .init_resource::<ComputationCounter>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (random_movement, camera_zoom, camera_pan, update_stats),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // camera
    commands.spawn(Camera2dBundle::default());

    // spawn creatures
    let mut rng = thread_rng();
    for _ in 0..NUM_CREATURES {
        let position = Position::new(rng.gen_range(-200.0..200.0), rng.gen_range(-200.0..200.0));
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);
        let creature = Creature::new(position, angle);
        commands.spawn((
            creature,
            MaterialMesh2dBundle {
                mesh: meshes
                    .add(Mesh::from(shape::RegularPolygon {
                        radius: 10.0,
                        sides: 3,
                    }))
                    .into(),
                material: materials.add(ColorMaterial::from(Color::rgb(
                    rng.gen(),
                    rng.gen(),
                    rng.gen(),
                ))),
                transform: Transform::from_xyz(position.x, position.y, 0.0)
                    .with_rotation(Quat::from_rotation_z(angle))
                    .with_scale(Vec3::new(0.5, 1.0, 1.0)),
                ..default()
            },
        ));
    }

    let text_style = TextStyle {
        font: bevy::text::DEFAULT_FONT_HANDLE.typed(),
        font_size: 20.0,
        color: Color::WHITE,
    };
    commands.spawn((
        StatsText,
        TextBundle::from_sections([
            TextSection::new("FPS: ", text_style.clone()),
            TextSection::new("0", text_style.clone()),
            TextSection::new(" | Computations: ", text_style.clone()),
            TextSection::new("0", text_style),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            left: Val::Px(5.0),
            ..default()
        }),
    ));
}

fn random_movement(
    mut query: Query<(&mut Creature, &mut Transform)>,
    time: Res<Time>,
    mut counter: ResMut<ComputationCounter>,
) {
    let mut rng = thread_rng();
    for (mut creature, mut transform) in &mut query {
        let turn = rng.gen_range(-creature.max_turn..creature.max_turn) * time.delta_seconds();
        creature.rotate(turn);
        let distance = rng.gen_range(0.0..creature.max_step) * time.delta_seconds();
        creature.move_forward(distance);
        transform.translation.x = creature.position.x;
        transform.translation.y = creature.position.y;
        transform.rotation = Quat::from_rotation_z(creature.angle);
        counter.0 += 1;
    }
}

fn camera_zoom(
    mut scroll_evr: EventReader<MouseWheel>,
    mut q: Query<&mut OrthographicProjection, With<Camera>>,
) {
    for mut proj in &mut q {
        for ev in scroll_evr.iter() {
            proj.scale = (proj.scale - ev.y * 0.1).clamp(0.1, 10.0);
        }
    }
}

fn update_stats(
    diagnostics: Res<DiagnosticsStore>,
    mut counter: ResMut<ComputationCounter>,
    mut query: Query<&mut Text, With<StatsText>>,
) {
    let fps = diagnostics
        .get(FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|d| d.smoothed())
        .unwrap_or(0.0);
    let computations = counter.0;
    for mut text in &mut query {
        text.sections[1].value = format!("{fps:.1}");
        text.sections[3].value = computations.to_string();
    }
    counter.0 = 0;
}

fn camera_pan(
    mouse_button_input: Res<Input<MouseButton>>,
    mut ev_motion: EventReader<MouseMotion>,
    mut q: Query<(&mut Transform, &OrthographicProjection), With<Camera>>,
) {
    if mouse_button_input.pressed(MouseButton::Right) {
        let (mut transform, projection) = q.single_mut();
        let scale = projection.scale;
        for ev in ev_motion.iter() {
            transform.translation.x -= ev.delta.x * scale;
            transform.translation.y += ev.delta.y * scale;
        }
    }
}
