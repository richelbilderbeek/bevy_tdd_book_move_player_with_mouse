use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

pub fn create_app() -> App {
    let mut app = App::new();

    // Only add this plugin in testing.
    // The main app will assume it to be absent
    if cfg!(test) {
        app.add_plugins(bevy::input::InputPlugin);
    }

    app.add_systems(Startup, add_player);
    app.add_systems(Update, (respond_to_mouse_button_press, respond_to_mouse_move));

    // Do not do update, as this will disallow to do more steps
    // app.update(); //Don't!
    app
}

fn add_player(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                scale: Vec3::new(64.0, 32.0, 0.0),
                ..default()
            },
            ..default()
        },
        Player,
    ));
}

fn respond_to_mouse_button_press(
    mut query: Query<&mut Transform, With<Player>>,
    input: Res<ButtonInput<MouseButton>>,
) {
    let mut player_position = query.single_mut();
    if input.pressed(MouseButton::Left) {
        // Do something
        player_position.translation.x += 16.0;
    }
}

fn respond_to_mouse_move(
    mut query: Query<&mut Transform, With<Player>>,
    mut mouse_motion_event: EventReader<MouseMotion>,
) {
    for event in mouse_motion_event.read() {
        let mut player_position = query.single_mut();
        player_position.translation.x += event.delta.x / 20.0;
        player_position.translation.y -= event.delta.y / 20.0;
    }
}


#[cfg(test)]
fn count_n_players(app: &mut App) -> usize {
    let mut query = app.world_mut().query::<&Player>();
    query.iter(app.world()).len()
}

#[cfg(test)]
fn get_player_coordinat(app: &mut App) -> Vec2 {
    let mut query = app.world_mut().query::<(&Transform, &Player)>();
    let (transform, _) = query.single(app.world());
    transform.translation.xy()
}

#[cfg(test)]
fn get_player_scale(app: &mut App) -> Vec2 {
    let mut query = app.world_mut().query::<(&Transform, &Player)>();
    let (transform, _) = query.single(app.world());
    transform.scale.xy()
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::input::keyboard::KeyCode;

    #[test]
    fn test_can_create_app() {
        create_app();
    }

    #[test]
    fn test_empty_app_has_no_players() {
        let mut app = App::new();
        assert_eq!(count_n_players(&mut app), 0);
    }

    #[test]
    fn test_create_app_has_a_player() {
        let mut app = create_app();
        app.update();
        assert_eq!(count_n_players(&mut app), 1);
    }

    #[test]
    fn test_player_is_at_origin() {
        let mut app = create_app();
        app.update();
        assert_eq!(get_player_coordinat(&mut app), Vec2::new(0.0, 0.0));
    }

    #[test]
    fn test_player_has_a_custom_scale() {
        let mut app = create_app();
        app.update();
        assert_eq!(get_player_scale(&mut app), Vec2::new(64.0, 32.0));
    }

    #[test]
    fn test_player_responds_to_mouse_move() {
        let mut app = create_app();
        app.update();
        assert_eq!(get_player_coordinat(&mut app), Vec2::new(0.0, 0.0));

        // Move the mouse
        app.world_mut().send_event(bevy::input::mouse::MouseMotion {
            delta: Vec2::new(100.0, 100.0),
        });

        app.update();
        assert_ne!(get_player_coordinat(&mut app), Vec2::new(0.0, 0.0));
    }

    #[test]
    fn test_player_responds_to_mouse_button_press() {
        let mut app = create_app();
        assert!(app.is_plugin_added::<InputPlugin>());
        app.update();

        // Not moved yet
        assert_eq!(Vec3::new(0.0, 0.0, 0.0), get_player_position(&mut app));

        // Press the left mouse button
        app.world_mut()
            .resource_mut::<ButtonInput<MouseButton>>()
            .press(MouseButton::Left);

        app.update();

        // Position must have changed now
        assert_ne!(Vec3::new(0.0, 0.0, 0.0), get_player_position(&mut app));
    }
}
