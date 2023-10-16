use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, setup_graphics)
        .add_systems(Startup, setup_physics)
        .add_systems(Update, print_position_system)
        .add_systems(Update, keyboard_input)
        .run();
}

fn setup_graphics(mut commands: Commands) {
    // Add a camera so we can see the debug-render.
    commands.spawn(Camera2dBundle::default());
}

fn setup_physics(mut commands: Commands) {
    /* Create the ground. */
    commands
        .spawn(Collider::cuboid(500.0, 50.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -100.0, 0.0)));

    /* Create the bouncing ball. */
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(10.5))
        .insert(ColliderMassProperties::Density(2.0))
        .insert(Restitution::coefficient(1.7))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 400.0, 0.0)))
        .insert(ExternalForce {
            force: Vec2::new(0.0, 0.0),
            torque: 0.0,
        });
}

#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
}

fn print_position_system(query: Query<&Position>) {
    for position in &query {
        println!("position: {} {}", position.x, position.y);
    }
}

fn keyboard_input(keys: Res<Input<KeyCode>>, mut mm: Query<&mut ExternalForce>) {
    if keys.just_pressed(KeyCode::Space) {

        // Space was pressed
    }
    if keys.just_released(KeyCode::B) {
        // Left Ctrl was released
    }
    if keys.pressed(KeyCode::Left) {
        for mut position in mm.iter_mut() {
            position.force = Vec2::new(-0.1, 0.8);
        }
    }
    if keys.pressed(KeyCode::Right) {
        for mut position in mm.iter_mut() {
            position.force = Vec2::new(0.5, 1.0);
        }
    }
    // we can check multiple at once with `.any_*`
    if keys.any_pressed([KeyCode::M, KeyCode::K]) {
        // Either the left or right shift are being held down
    }
    if keys.any_just_pressed([KeyCode::Delete, KeyCode::Back]) {
        // Either delete or backspace was just pressed
    }
}
