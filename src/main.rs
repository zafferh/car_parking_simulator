use bevy::{prelude::*, window::WindowResolution};
#[derive(Component)]
struct Car {
    color: String,
    position: Position,
}

#[derive(Component)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Resource)]
struct HonkTimer(Timer);

fn create_cars(mut commands: Commands) {
    commands.spawn(Car {
        color: "red".to_string(),
        position: Position { x: (0), y: (0) },
    });
    commands.spawn(Car {
        color: "green".to_string(),
        position: Position { x: (10), y: (10) },
    });
}

/*fn reset_cars(mut query: Query<&mut Car>) {
    for mut car in &mut query {
        car.position.x = 0;
        car.position.y = 0;
    }
}*/

fn honk_cars(time: Res<Time>, mut timer: ResMut<HonkTimer>, query: Query<&Car>) {
    if timer.0.tick(time.delta()).just_finished() {
        for car in &query {
            println!(
                "a {} car at location ({},{}) honks!",
                car.color, car.position.x, car.position.y
            );
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Car Parking Simulator".to_string(),
                resolution: WindowResolution::new(3000, 3000).with_scale_factor_override(1.0),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(CarPlugin)
        .run();
}

pub struct CarPlugin;

impl Plugin for CarPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(HonkTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));
        app.add_systems(Startup, create_cars);
        app.add_systems(Update, honk_cars);
    }
}
