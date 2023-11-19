use std::f32::consts::PI;

use bevy::{prelude::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}, render::camera};
use bevy_rapier2d::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy::window::PrimaryWindow;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
        .init_resource::<CurrentSuika>()
        //.init_resource::<SuikasToBeDeleted>()
        //.init_resource::<PositionsForNewSuikas>()
        .add_systems(PreStartup, setup)
        .add_systems(Startup, setup_graphics)
        .add_systems(Startup, setup_world)
        .add_systems(Update, (input_handler, move_paddle))
        .add_systems(Update, get_suika_collisions)
        //.add_systems(Update, delete_suikas)
        //.add_systems(Update, add_new_suikas)
        //.add_systems(Update, suika_query)
        //.add_systems(FixedUpdate, rapier_context_query)
        .run();
}

#[derive(Resource, Default)]
struct SuikaData {
    meshes: Vec<Mesh2dHandle>,
    colors: Vec<Handle<ColorMaterial>>,
    sizes: Vec<f32>
}

#[derive(Resource, Default)]
struct SuikasToBeDeleted(Vec<Entity>);

#[derive(Resource, Default)]
struct PositionsForNewSuikas(Vec<Vec2>);

#[derive(Resource, Default)]
struct CurrentSuika(SuikaTypes);

#[derive(Default, Clone, Copy, Component, Debug, PartialEq)]
enum SuikaTypes {
    #[default]
    Apple,
    Orange,
    Grapefruit
}

#[derive(Component)]
struct SpawnPaddle;

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    let mut suika_meshes = Vec::new();
    let mut suika_colors = Vec::new();
    let mut suika_sizes = Vec::new();

    // APPLES
    suika_sizes.push(10.);
    suika_meshes.push(meshes.add(shape::Circle::new(suika_sizes[SuikaTypes::Apple as usize]).into()).into());
    suika_colors.push(materials.add(ColorMaterial::from(Color::RED))); 

    //ORANGES
    suika_sizes.push(50.);
    suika_meshes.push(meshes.add(shape::Circle::new(suika_sizes[SuikaTypes::Orange as usize]).into()).into());
    suika_colors.push(materials.add(ColorMaterial::from(Color::ORANGE))); 

    //GRAPEFRUITS
    suika_sizes.push(75.);
    suika_meshes.push(meshes.add(shape::Circle::new(suika_sizes[SuikaTypes::Grapefruit as usize]).into()).into());
    suika_colors.push(materials.add(ColorMaterial::from(Color::PINK))); 

    let suika_material = SuikaData {
        meshes: suika_meshes,
        colors: suika_colors,
        sizes: suika_sizes
    };

    commands.insert_resource(suika_material);
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

fn spawn_suika(commands: &mut Commands, suika_data: &Res<SuikaData>, suika_type: SuikaTypes, position: Vec2) {
    let size: f32;
    let color: Handle<ColorMaterial>;
    let mesh: Mesh2dHandle;

    size = suika_data.sizes[suika_type as usize].clone();
    color = suika_data.colors[suika_type as usize].clone();
    mesh = suika_data.meshes[suika_type as usize].clone();

    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(size))
        .insert(Restitution::coefficient(0.7))
        .insert(GravityScale(5.))
        .insert(MaterialMesh2dBundle {
            mesh: mesh,
            material: color,
            ..default()
        })
        .insert(Velocity {
            linvel: Vec2::new(10., -1000.),
            ..default()
        })
        .insert(TransformBundle::from(Transform::from_xyz(position.x, position.y, 0.0)))
        .insert(suika_type)
        .insert(Ccd::enabled())
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
            current_suika.0 = get_new_suika(current_suika.0);
            spawn_suika(&mut commands, &suika_data, current_suika.0, Vec2::new(world_position.x, 400.));
            println!("Spawned {:?} at {}", current_suika.0, world_position.x)
        } else {
            println!("Cursor is not in the game window.");
        }
    }
}

fn get_new_suika(current_suika: SuikaTypes) -> SuikaTypes{
    match current_suika {
        SuikaTypes::Apple => SuikaTypes::Orange,
        SuikaTypes::Orange => SuikaTypes::Grapefruit,
        SuikaTypes::Grapefruit => SuikaTypes::Apple,
    }
}

fn delete_suikas(mut commands: Commands, mut to_be_deleted: ResMut<SuikasToBeDeleted>) {
    while !to_be_deleted.0.is_empty() {
        match to_be_deleted.0.pop() {
            Some(id) => {
                commands.entity(id).despawn();
            },
            None => {
                // do nothing if empty
            },
        }
    }
}

fn add_new_suikas(mut commands: Commands, suika_data: Res<SuikaData>, mut current_suika: ResMut<CurrentSuika>, mut positions: ResMut<PositionsForNewSuikas>) {
    while !positions.0.is_empty() {
        match positions.0.pop() {
            Some(position) => {
                spawn_suika(&mut commands, &suika_data, current_suika.0, position);
            },
            None => {
                // do nothing if empty
            },
        }
    }
}

fn get_bigger_suika(current_suika: SuikaTypes) -> SuikaTypes{
    match current_suika {
        SuikaTypes::Apple => SuikaTypes::Orange,
        SuikaTypes::Orange => SuikaTypes::Grapefruit,
        SuikaTypes::Grapefruit => SuikaTypes::Apple
    }
}

fn get_suika_collisions(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    suika_query: Query<&SuikaTypes, &Transform>,
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

                        /*
                        deleted.0.push(*id1);
                        deleted.0.push(*id2);
                        if let Ok(transform) = suika_query.get_component::<Transform>(*id1){
                            new.0.push(Vec2::new(transform.translation.x, transform.translation.y));
                        }
                        */
                    }
                }
            }
        }
    }
}

fn suika_query(suika_query: Query<&SuikaTypes>) {
    for suika in suika_query.iter() {
        println!("{:?}", suika);
    }
}

fn rapier_context_query(rapier_context: Res<RapierContext>) {
    println!("{}", rapier_context.integration_parameters.erp);
}