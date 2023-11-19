use std::f32::consts::PI;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy::window::PrimaryWindow;
use rand::Rng;

const SUIKA_TIERS: usize = 12;
const MAX_SUIKA_SIZE: f32 = 200.;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
        .init_resource::<CurrentSuika>()
        .add_systems(PreStartup, setup)
        .add_systems(Startup, setup_graphics)
        .add_systems(Startup, setup_world)
        .add_systems(Update, (input_handler, move_paddle))
        .add_systems(Update, get_suika_collisions)
        //.add_systems(FixedUpdate, rapier_context_query)
        .run();
}

#[derive(Resource, Default)]
struct SuikaData(Vec<Suika>);

struct Suika {
    size_ratio: f32,
    suika_sprite: Handle<Image>
}

#[derive(Resource, Default)]
struct CurrentSuika(SuikaTier);

#[derive(Default, Clone, Copy, Component, Debug, PartialEq)]
struct SuikaTier(usize);

#[derive(Component)]
struct SpawnPaddle;

fn create_new_suika(
    asset_server: &Res<AssetServer>,
    size: f32,
    tier: SuikaTier
) -> Suika {
    let suika = Suika {
        size_ratio: size,
        suika_sprite: match tier.0 {
            0 => asset_server.load("south_korea.png"),
            1 => asset_server.load("poland.png"),
            2 => asset_server.load("germany.png"),
            3 => asset_server.load("japan.png"),
            4 => asset_server.load("south_africa.png"),
            5 => asset_server.load("india.png"),
            6 => asset_server.load("australia.png"),
            7 => asset_server.load("brazil.png"),
            8 => asset_server.load("china.png"),
            9 => asset_server.load("usa.png"),
            10 => asset_server.load("canada.png"),
            11 => asset_server.load("russia.png"),
            _ => {panic!("NOT A VALID TIER")}
        }
    };
    return suika;
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut suika_data = SuikaData{0: Vec::<Suika>::new()};

    // Tier0
    suika_data.0.push(create_new_suika(&asset_server, 0.1, SuikaTier{0: 0}));

    // Tier1
    suika_data.0.push(create_new_suika(&asset_server,  0.2, SuikaTier{0: 1}));

    // Tier2
    suika_data.0.push(create_new_suika(&asset_server,  0.3, SuikaTier{0: 2}));

    // Tier3
    suika_data.0.push(create_new_suika(&asset_server,  0.4, SuikaTier{0: 3}));
    
    // Tier4
    suika_data.0.push(create_new_suika(&asset_server,  0.5, SuikaTier{0: 4}));

    // Tier5
    suika_data.0.push(create_new_suika(&asset_server,  0.55, SuikaTier{0: 5}));

    // Tier6
    suika_data.0.push(create_new_suika(&asset_server,  0.6, SuikaTier{0: 6}));

    // Tier7
    suika_data.0.push(create_new_suika(&asset_server,  0.65, SuikaTier{0: 7}));

    // Tier8
    suika_data.0.push(create_new_suika(&asset_server,  0.7, SuikaTier{0: 8}));

    // Tier9
    suika_data.0.push(create_new_suika(&asset_server,  0.8, SuikaTier{0: 9}));

    // Tier10
    suika_data.0.push(create_new_suika(&asset_server,  0.9, SuikaTier{0: 10}));

    // Tier11
    suika_data.0.push(create_new_suika(&asset_server,  1.0, SuikaTier{0: 11}));

    commands.insert_resource(suika_data);
}

fn setup_graphics(mut commands: Commands) {
    // Add a camera so we can see the debug-render.
    commands.spawn(Camera2dBundle::default());
}

fn setup_world(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    // spawns spawner
    commands.spawn((MaterialMesh2dBundle {
        mesh: meshes.add(shape::RegularPolygon::new(50., 3).into()).into(),
        material: materials.add(ColorMaterial::from(Color::PURPLE)),
        transform: Transform::from(Transform {
            translation: Vec3::new(0., 300., 1.),
            rotation: Quat::from_rotation_z(PI),
            scale: Vec3::new(0.5, 0.25, 1.),
        }),
        ..default()
    },
    SpawnPaddle,
    ));

    //left wall
    commands
        .spawn(Collider::cuboid(50., 500.))
        .insert(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Box::new(100., 1000., 1.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            ..default()
        })
        .insert(TransformBundle::from(Transform::from_xyz(-500., 200.0, 0.0)));

    //right wall
    commands
        .spawn(Collider::cuboid(50., 500.))
        .insert(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Box::new(100., 1000., 1.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            ..default()
        })
        .insert(TransformBundle::from(Transform::from_xyz(500.0, 200.0, 0.0)));

    //ground
    commands
        .spawn(Collider::cuboid(500.0, 50.0))
        .insert(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Box::new(1000., 100., 1.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            ..default()
        })
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -300.0, 0.0)));
}

fn spawn_suika(commands: &mut Commands, suika_data: &Res<SuikaData>, suika_tier: SuikaTier, position: Vec2) {
    let size: f32;
    let suika_sprite: Handle<Image>;

    size = suika_data.0[suika_tier.0].size_ratio.clone() * MAX_SUIKA_SIZE;
    suika_sprite = suika_data.0[suika_tier.0].suika_sprite.clone();

    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(size))
        //.insert(ColliderMassProperties::Mass(size * 100000.))
        //.insert(ColliderMassProperties::Density(size * 10000.))
        .insert(Restitution::coefficient(0.))
        //.insert(GravityScale(500.))
        .insert(Damping { linear_damping: 0.0, angular_damping: 0.0 })
        .insert(SpriteBundle {
            texture: suika_sprite,
            sprite: Sprite {
                custom_size: Some(Vec2::new(size * 2.15, size * 2.15)),
                ..default()
            },
            ..default()
        })
        .insert(Velocity {
            linvel: Vec2::new(10., -1000.),
            ..default()
        })
        .insert(TransformBundle::from(Transform::from_xyz(position.x, position.y, 0.0)))
        .insert(suika_tier)
        //.insert(Ccd::enabled())
        .insert(ActiveEvents::COLLISION_EVENTS);
}

fn move_paddle(
    mut paddle_query: Query<&mut Transform, With<SpawnPaddle>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera2d>>
) {
    let (camera, camera_transform) = camera_query.single();
    let window = window_query.single();
    if let Some(world_position) = window.cursor_position()
    .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
    .map(|ray| ray.origin.truncate())
    {
        let mut paddle = paddle_query.single_mut();
        paddle.translation.x = world_position.x;
    } else {
        println!("Cursor is not in the game window.");
    }
}

fn input_handler(
    mut commands: Commands,
    mouse_button_input: Res<Input<MouseButton>>,
    suika_data: Res<SuikaData>,
    mut current_suika: ResMut<CurrentSuika>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
) {
    if mouse_button_input.just_released(MouseButton::Left) {
        let (camera, camera_transform) = camera_query.single();
        let window = window_query.single();
        if let Some(world_position) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
        {
            current_suika.0 = get_new_suika();
            spawn_suika(&mut commands, &suika_data, current_suika.0, Vec2::new(world_position.x, 400.));
            println!("Spawned {:?} at {}", current_suika.0, world_position.x)
        } else {
            println!("Cursor is not in the game window.");
        }
    }
}

fn get_new_suika() -> SuikaTier{
    let mut rng = rand::thread_rng();
    let s = SuikaTier{0: rng.gen_range(0..3)};
    return s;
}

fn get_bigger_suika(current_suika: SuikaTier) -> SuikaTier{
    if current_suika.0 < (SUIKA_TIERS - 1) {
        return SuikaTier{0: current_suika.0 + 1};
    } else {
        return SuikaTier{0: SUIKA_TIERS - 1}
    }
}

fn get_suika_collisions(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    suika_query: Query<&SuikaTier, &Transform>,
    suika_data: Res<SuikaData>,
) {
    for collision_event in collision_events.read() {
        if let CollisionEvent::Started(id1, id2, _) = collision_event {
            if let Ok(suika1) = suika_query.get(*id1) {
                if let Ok(suika2) = suika_query.get(*id2) {
                    if suika1 == suika2 {
                        if let Ok(transform) = suika_query.get_component::<Transform>(*id1){
                            spawn_suika(&mut commands, &suika_data, get_bigger_suika(*suika1), Vec2::new(transform.translation.x, transform.translation.y))
                        }

                        commands.entity(*id1).despawn();
                        commands.entity(*id2).despawn();
                    }
                }
            }
        }
    }
}

fn rapier_context_query(rapier_context: Res<RapierContext>) {
    println!("{}", rapier_context.integration_parameters.erp);
}