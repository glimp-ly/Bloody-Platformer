use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Bloody Platformer".into(),
                    resolution: (800., 600.).into(),
                    ..default()
                }),
                ..default()
            }),
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
        ))
        .add_systems(Startup, setup_game)
        .add_systems(Update, (player_movement, exit_on_esc, apply_gravity, ground_detection_player, enemy_movement, ground_detection_enemy))
        .run();
}

// Componentes
#[derive(Component)]
struct Player {
    jump_power: f32,
    move_speed: f32,
    is_grounded: bool,
}

#[derive(Component)]
struct Enemy {
    jump_power: f32,
    move_speed: f32,
    is_grounded: bool,
}

#[derive(Component)]
struct Gravity(f32);

#[derive(Component)]
struct Ground;

// Configuración inicial
fn setup_game(
    mut commands: Commands,
    asset_server: Res<AssetServer>, // Para cargar la imagen
) {
    // Cámara
    commands.spawn(Camera2dBundle::default());
    
    // Jugador
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("textures/dio_player.png"), // Ruta a asset
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            sprite: Sprite {
                custom_size: Some(Vec2::new(80.0, 80.0)), // Opcional: ajustar tamaño
                color: Color::WHITE, // Usa WHITE para mantener colores originales
                ..default()
            },
            ..default()
        },
        RigidBody::Dynamic,
        Collider::cuboid(40.0, 40.0),
        Velocity::zero(),
        GravityScale(0.0), // Desactivamos la gravedad de Rapier
        Player {
            jump_power: 650.0,
            move_speed: 200.0,
            is_grounded: false,
        },
        Gravity(-9.8 * 100.0), // Gravedad personalizada
    ));

    //Enemigo
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::BLUE,
                custom_size: Some(Vec2::new(40.0, 40.0)),
                ..default()
            },
            transform: Transform::from_xyz(10.0, 0.0, 0.0),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::cuboid(20.0, 20.0),
        Velocity::zero(),
        GravityScale(0.0),
        Enemy{
            jump_power: 500.0,
            move_speed: 250.0,
            is_grounded: false,
        },
        Gravity(-10.0 * 100.0),
    ));

    // Piso
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::GREEN,
                custom_size: Some(Vec2::new(1000.0, 20.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, -200.0, 0.0),
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid(500.0, 10.0),
        Ground, // Marcamos como suelo
    ));

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::GREEN,
                custom_size: Some(Vec2::new(100.0, 20.0)),
                ..default()
            },
            transform: Transform::from_xyz(-100.0, -100.0, 0.0),
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid(50.0, 10.0),
        Ground, // Marcamos como suelo
    ));

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::GREEN,
                custom_size: Some(Vec2::new(20.0, 20.0)),
                ..default()
            },
            transform: Transform::from_xyz(100.0, 0.0, 0.0),
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid(10.0, 10.0),
        Ground, // Marcamos como suelo
    ));

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::GREEN,
                custom_size: Some(Vec2::new(20.0, 20.0)),
                ..default()
            },
            transform: Transform::from_xyz(100.0, 0.0, 0.0),
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid(10.0, 10.0),
        Ground, // Marcamos como suelo
    ));
}

// Sistema de movimiento para el enemigo
fn enemy_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Velocity, &mut Transform, &mut Enemy)>,    
) {
        for (mut velocity, mut transform, mut enemy) in &mut query {
        let mut direction_x = 0.0;
        let mut jump = false;

        if keyboard_input.pressed(KeyCode::KeyH) || keyboard_input.pressed(KeyCode::Enter) {
            direction_x -= 210.0;
            transform.rotation = Quat::from_rotation_z(std::f32::consts::FRAC_PI_4);
        }
        if direction_x == 0.0 {
            transform.rotation = Quat::IDENTITY;
        }

        // Movimiento horizontal
        velocity.linvel.x = direction_x * enemy.move_speed;
    }
}

// Sistema de movimiento mejorado
fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Velocity, &mut Transform, &mut Player)>,
) {
    for (mut velocity, mut transform, mut player) in &mut query {
        let mut direction_x = 0.0;
        let mut jump = false;

        if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
            direction_x -= 1.0;
            transform.rotation = Quat::from_rotation_z(std::f32::consts::FRAC_PI_4);
        }
        if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
            direction_x += 1.0;
            transform.rotation = Quat::from_rotation_z(-std::f32::consts::FRAC_PI_4);
        }
        if direction_x == 0.0 {
            transform.rotation = Quat::IDENTITY;
        }
        
        if keyboard_input.just_pressed(KeyCode::Space) && player.is_grounded {
            jump = true;
            player.is_grounded = false;
        }

        // Movimiento horizontal
        velocity.linvel.x = direction_x * player.move_speed;
        
        // Salto
        if jump {
            velocity.linvel.y = player.jump_power;
        }
    }
}

// Gravedad personalizada
fn apply_gravity(
    mut query: Query<(&mut Velocity, &Gravity)>,
    time: Res<Time>,
) {
    for (mut velocity, gravity) in &mut query {
        velocity.linvel.y += gravity.0 * time.delta_seconds();
    }
}

// Detectar colisión con el suelo
fn ground_detection_player(
    mut player_query: Query<(&mut Player, &Transform), Without<Ground>>,
    ground_query: Query<&Transform, With<Ground>>,
) {
    for (mut player, player_transform) in &mut player_query {
        player.is_grounded = false;
        for ground_transform in &ground_query {
            if player_transform.translation.y - 15.0 <= ground_transform.translation.y + 10.0 &&
               player_transform.translation.x.abs() < ground_transform.translation.x.abs() + 500.0
            {
                player.is_grounded = true;
                break;
            }
        }
    }
}

fn ground_detection_enemy(
    mut enemy_query: Query<(&mut Enemy, &Transform), Without<Ground>>,
    ground_query: Query<&Transform, With<Ground>>,
) {
   for (mut enemy, enemy_transform) in &mut enemy_query {
        enemy.is_grounded = false;
        for ground_transform in &ground_query {
            if enemy_transform.translation.y - 15.0 <= ground_transform.translation.y + 10.0 &&
               enemy_transform.translation.x.abs() < ground_transform.translation.x.abs() + 500.0
            {
                enemy.is_grounded = true;
                break;
            }
        }
    } 
}

// Salir con ESC
fn exit_on_esc(keyboard_input: Res<ButtonInput<KeyCode>>) {
    if keyboard_input.pressed(KeyCode::Escape) {
        std::process::exit(0);
    }
}