use bevy::prelude::*;
use std::f32::consts::{FRAC_PI_2, PI};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0,
        })
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut animations: ResMut<Assets<AnimationClip>>,
) {
    // Camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(0.0, 0.0, -4.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // The animation API uses the `Name` component to target entities
    let card = Name::new("card");
    let texture_handle = asset_server.load("example.png");
    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(texture_handle),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..default()
    });

    // Creating the animation
    let mut animation = AnimationClip::default();
    // A curve can modify a single part of a transform, here the translation
    // There can be more than one curve targeting the same entity path
    animation.add_curve_to_path(
        EntityPath {
            parts: vec![card.clone()],
        },
        VariableCurve {
            keyframe_timestamps: vec![0.0, 0.15, 0.3, 0.45, 0.6],
            keyframes: Keyframes::Rotation(vec![
                Quat::from_axis_angle(Vec3::X, 0.0),
                Quat::from_axis_angle(Vec3::X, FRAC_PI_2),
                Quat::from_axis_angle(Vec3::X, PI),
                Quat::from_axis_angle(Vec3::X, FRAC_PI_2),
                Quat::from_axis_angle(Vec3::X, 0.0),
            ]),
        },
    );

    // Create the animation player, and set it to repeat
    let mut player = AnimationPlayer::default();
    player.play(animations.add(animation)).repeat();

    // Create the scene that will be animated
    // First entity is the planet
    let card_shape = shape::Box::new(1.0, 1.0, 0.0);
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(card_shape)),
            material: material_handle,
            ..default()
        })
        // Add the Name component, and the animation player
        .insert_bundle((card, player));
}
