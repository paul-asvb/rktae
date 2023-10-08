use bevy::{input::keyboard::KeyboardInput, prelude::*};
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

// /// This system prints out all keyboard events as they come in
// fn print_keyboard_event_system(mut keyboard_input_events: EventReader<KeyboardInput>) {
//     for event in keyboard_input_events {
//         info!("{:?}", event);
//     }
// }

fn keyboard_input(keys: Res<Input<KeyCode>>) {
    if keys.pressed(KeyCode::Left) {
        println!("left");
    }
    if keys.pressed(KeyCode::Right) {
        println!("right");
    }
    if keys.just_pressed(KeyCode::Space) {
        // Space was pressed
        println!("space");
    }
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
        .insert(Collider::ball(5.0))
        .insert(Restitution::coefficient(1.7))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 400.0, 0.0)));
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

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Rocket {
    Left,
    Middle,
    Right,
}
impl Rocket {
    /// Checks if a key that corresponds to this direction has been pressed
    pub fn key_just_pressed(&self, input: &Input<KeyCode>) -> bool {
        let keys = match self {
            Rocket::Left => [KeyCode::Left, KeyCode::J],
            Rocket::Right => [KeyCode::Right, KeyCode::K],
            Rocket::Middle => [KeyCode::Right, KeyCode::K],
        };

        keys.iter().any(|code| input.just_pressed(*code))
    }
}
