use std::f32::consts::PI;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle, window::{PresentMode, WindowTheme}};
use bevy_rapier2d::{prelude::*, rapier::geometry::CollisionEventFlags};
use bevy::window::PrimaryWindow;
use rand::Rng;

const SUIKA_TIERS: usize = 12;
const MAX_SUIKA_SIZE: f32 = 200.;
const SPAWN_AREA: f32 = 425.;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum AppState {
    #[default]
    InGame,
    Win,
    Lost
}
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Polandball Suika".into(),
                //resolution: (500., 300.).into(),
                mode: bevy::window::WindowMode::BorderlessFullscreen,
                present_mode: PresentMode::AutoVsync,
                window_theme: Some(WindowTheme::Dark),
                enabled_buttons: bevy::window::EnabledButtons {
                    maximize: false,
                    ..Default::default()
                },
                // This will spawn an invisible window
                // The window will be made visible in the make_visible() system after 3 frames.
                // This is useful when you want to avoid the white window that shows up before the GPU is ready to render the app.
                visible: true,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(512.))
        //.add_plugins(RapierDebugRenderPlugin::default())
        //.add_plugins(WorldInspectorPlugin::new())
        .add_state::<AppState>()
        .init_resource::<CurrentSuika>()
        .init_resource::<SuikaSprites>()
        .init_resource::<Scores>()
        .register_type::<ColliderMassProperties>()
        .add_systems(PreStartup, setup)
        .add_systems(Startup, setup_graphics)
        .add_systems(Startup, setup_world)
        .add_systems(Update, (input_handler, move_paddle).run_if(in_state(AppState::InGame)))
        .add_systems(FixedUpdate, get_suika_collisions.run_if(in_state(AppState::InGame)))
        .add_systems(FixedUpdate, win_game.run_if(in_state(AppState::Win)))
        .add_systems(FixedUpdate, game_over.run_if(in_state(AppState::Lost)))
        .add_systems(OnEnter(AppState::Lost), setup_game_over)
        .add_systems(OnEnter(AppState::Win), setup_win)
        .add_systems(OnExit(AppState::Win), cleanup_world)
        .add_systems(OnExit(AppState::Lost), cleanup_world)
        .add_systems(FixedUpdate, update_score.run_if(in_state(AppState::InGame)))
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

#[derive(Component)]
struct SuikaIndicator;

#[derive(Resource, Clone)]
struct SuikaSprites(Vec<Handle<Image>>);

#[derive(Resource, Default)]
struct Scores {
    current_score: i32,
}

#[derive(Component)]
struct GameOverText;

#[derive(Component)]
struct WinGameText;

#[derive(Component)]
struct ScoreText;

impl FromWorld for SuikaSprites {
    fn from_world(world: &mut World) -> Self {
        let mut suika_sprites = SuikaSprites{0: Vec::new()};

        if let Some(asset_server) = world.get_resource::<AssetServer>() {
            suika_sprites.0.push(asset_server.load("south_korea.png"));
            suika_sprites.0.push(asset_server.load("poland.png"));
            suika_sprites.0.push(asset_server.load("germany.png"));
            suika_sprites.0.push(asset_server.load("japan.png"));
            suika_sprites.0.push(asset_server.load("south_africa.png"));
            suika_sprites.0.push(asset_server.load("india.png"));
            suika_sprites.0.push(asset_server.load("australia.png"));
            suika_sprites.0.push(asset_server.load("brazil.png"));
            suika_sprites.0.push(asset_server.load("china.png"));
            suika_sprites.0.push(asset_server.load("usa.png"));
            suika_sprites.0.push( asset_server.load("canada.png"));
            suika_sprites.0.push( asset_server.load("russia.png"));
            return suika_sprites;
        } else {
            panic!("couldnt get asset server????")
        }
    }
}

fn create_new_suika(
    size: f32,
    tier: SuikaTier,
    suika_sprites: &Res<SuikaSprites>
) -> Suika {
    let suika = Suika {
        size_ratio: size,
        suika_sprite: suika_sprites.0[tier.0].clone(),
    };
    return suika;
}

fn setup(mut commands: Commands, suika_sprites: Res<SuikaSprites>) {
    let mut suika_data = SuikaData{0: Vec::<Suika>::new()};

    // Tier0
    suika_data.0.push(create_new_suika(0.1, SuikaTier{0: 0}, &suika_sprites));

    // Tier1
    suika_data.0.push(create_new_suika( 0.2, SuikaTier{0: 1}, &suika_sprites));

    // Tier2
    suika_data.0.push(create_new_suika( 0.3, SuikaTier{0: 2}, &suika_sprites));

    // Tier3
    suika_data.0.push(create_new_suika( 0.4, SuikaTier{0: 3}, &suika_sprites));
    
    // Tier4
    suika_data.0.push(create_new_suika( 0.5, SuikaTier{0: 4}, &suika_sprites));

    // Tier5
    suika_data.0.push(create_new_suika( 0.55, SuikaTier{0: 5}, &suika_sprites));

    // Tier6
    suika_data.0.push(create_new_suika( 0.6, SuikaTier{0: 6}, &suika_sprites));

    // Tier7
    suika_data.0.push(create_new_suika( 0.65, SuikaTier{0: 7}, &suika_sprites));

    // Tier8
    suika_data.0.push(create_new_suika( 0.7, SuikaTier{0: 8}, &suika_sprites));

    // Tier9
    suika_data.0.push(create_new_suika( 0.8, SuikaTier{0: 9}, &suika_sprites));

    // Tier10
    suika_data.0.push(create_new_suika( 0.9, SuikaTier{0: 10}, &suika_sprites));

    // Tier11
    suika_data.0.push(create_new_suika( 1.0, SuikaTier{0: 11}, &suika_sprites));

    commands.insert_resource(suika_data);
}

fn cleanup_world(
    mut commands: Commands,
    suikas_query: Query<Entity, With<SuikaTier>>,
    mut scores: ResMut<Scores>,
    mut score_text_query: Query<&mut Text, With<ScoreText>>,
    game_over_text_query: Query<Entity, With<GameOverText>>,
    win_text_query: Query<Entity, With<WinGameText>>
) {
    for id in suikas_query.iter() {
        commands.entity(id).despawn();
    }

    scores.current_score = 0;
    if let Ok(mut score_text) = score_text_query.get_single_mut() {
        score_text.sections[0].value = String::from("Score: 0");
    }

    if let Ok(game_over_text) = game_over_text_query.get_single() {
        commands.entity(game_over_text).despawn();
    }

    if let Ok(win_text) = win_text_query.get_single() {
        commands.entity(win_text).despawn();
    }
}

fn setup_game_over(mut commands: Commands) {
    commands.spawn(TextBundle::from_section (
        "GAME OVER!",
        TextStyle {
            font_size: 120.,
            color: Color::RED,
            ..default()
        })
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Percent(50.),
            right: Val::Percent(50.),
            ..default()
        }))
        .insert(GameOverText);
}

fn setup_win(mut commands: Commands) {
    commands.spawn(TextBundle::from_section (
        "YOU WIN!",
        TextStyle {
            font_size: 120.,
            color: Color::BLUE,
            ..default()
        }).with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Percent(50.),
            right: Val::Percent(50.),
            ..default()
        }))
    .insert(WinGameText);
}

fn game_over(keys: Res<Input<KeyCode>>, mut next_state: ResMut<NextState<AppState>>) {
    if keys.just_released(KeyCode::Space) {
        next_state.set(AppState::InGame);
    }
}

fn win_game(keys: Res<Input<KeyCode>>, mut next_state: ResMut<NextState<AppState>>) {
    if keys.just_released(KeyCode::Space) {
        next_state.set(AppState::InGame);
    }
}

fn setup_graphics(mut commands: Commands) {
    // Add a camera so we can see the debug-render.
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0., 100., 0.,),
        ..default()
    });
}

fn setup_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut rapier_config: ResMut<RapierConfiguration>,
    mut rapier_context: ResMut<RapierContext>,
    suika_sprites: Res<SuikaSprites>,
    current_suika: Res<CurrentSuika>,
) {
    rapier_config.timestep_mode = TimestepMode::Fixed { dt: 1.0 / 60.0, substeps: 8 };
    //rapier_context.integration_parameters.erp = 1.;
    //rapier_context.integration_parameters.max_penetration_correction = 1000.;
    rapier_context.integration_parameters.max_stabilization_iterations = 16;
    
    // spawns spawner
    commands.spawn((MaterialMesh2dBundle {
        mesh: meshes.add(shape::RegularPolygon::new(50., 3).into()).into(),
        material: materials.add(ColorMaterial::from(Color::WHITE)),
        transform: Transform::from(Transform {
            translation: Vec3::new(0., 500., 1.),
            rotation: Quat::from_rotation_z(PI),
            scale: Vec3::new(0.5, 0.25, 1.),
        }),
        ..default()
    },
    SpawnPaddle,
    ));

    // suika indicator
    commands.spawn(SpriteBundle {
        texture: suika_sprites.0[current_suika.0.0].clone(),
        sprite: Sprite {
            custom_size: Some(Vec2::new(50., 50.)),
            ..default()
        },
        transform: Transform::from_xyz(0., 550., 1.),
        ..default()
    })
    .insert(SpawnPaddle)
    .insert(SuikaIndicator);

    commands.spawn(TextBundle::from_section (
        "Score: 0",
        TextStyle {
            font_size: 120.,
            color: Color::WHITE,
            ..default()
    }))
    .insert(ScoreText);

    //left wall
    commands
        .spawn(Collider::cuboid(50., 400.))
        .insert(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Box::new(100., 800., 1.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            ..default()
        })
        .insert(TransformBundle::from(Transform::from_xyz(-500., 50.0, 0.0)));

    //right wall
    commands
        .spawn(Collider::cuboid(50., 400.))
        .insert(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Box::new(100., 800., 1.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            ..default()
        })
        .insert(TransformBundle::from(Transform::from_xyz(500.0, 50.0, 0.0)));

    //ground
    commands
        .spawn(Collider::cuboid(500.0, 50.0))
        .insert(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Box::new(1000., 100., 1.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            ..default()
        })
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -300.0, 0.0)));

    //kill_zone
    commands
    .spawn(Collider::cuboid(10000., 100.))
    .insert(TransformBundle::from(Transform::from_xyz(0.0, -500.0, 0.0)))
    .insert(Sensor);
}

fn update_score(scores: Res<Scores>, mut score_text_query: Query<&mut Text, With<ScoreText>>) {
    if let Ok(mut score_text) = score_text_query.get_single_mut() {
        score_text.sections[0].value = format!("Score: {}", scores.current_score);
    }
}

fn spawn_suika(commands: &mut Commands, suika_data: &Res<SuikaData>, suika_tier: SuikaTier, position: Vec2, scores: &mut ResMut<Scores>, next_state: &mut ResMut<NextState<AppState>>,
) {

    if suika_tier.0 == SUIKA_TIERS-1 {
        //WIN GAME ONGOD
        next_state.set(AppState::Win);
        return;
    }

    let size: f32;
    let suika_sprite: Handle<Image>;

    size = suika_data.0[suika_tier.0].size_ratio.clone() * MAX_SUIKA_SIZE;
    suika_sprite = suika_data.0[suika_tier.0].suika_sprite.clone();

    scores.current_score += ((suika_tier.0+1) * 10) as i32;
    println!("score: {}", scores.current_score);

    let mut rng = rand::thread_rng();

    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(size))
        .insert(ColliderMassProperties::Mass(size * 10312930000.))
        //.insert(ColliderMassProperties::Density(size * 121321321310000.))
        .insert(Restitution::coefficient(0.))
        .insert(GravityScale(100.))
        .insert(Damping { linear_damping: 1.0, angular_damping: 0.0 })
        .insert(SpriteBundle {
            texture: suika_sprite,
            sprite: Sprite {
                custom_size: Some(Vec2::new(size * 2.15, size * 2.15)),
                ..default()
            },
            ..default()
        })
        .insert(Velocity {
            linvel: Vec2::new(rng.gen::<f32>() * 2. - 1., 0.),
            ..default()
        })
        .insert(Friction {
            coefficient: 1.0,
            combine_rule: CoefficientCombineRule::Min
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
        for mut paddle in paddle_query.iter_mut() {
            let new_paddle_position = world_position.x.clamp(-SPAWN_AREA, SPAWN_AREA);
            paddle.translation.x = new_paddle_position;
        }
    } else {
        // println!("Cursor is not in the game window.");
    }
}

fn input_handler(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    suika_data: Res<SuikaData>,
    mut current_suika: ResMut<CurrentSuika>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    mut suika_indicator_query: Query<&mut Handle<Image>, With<SuikaIndicator>>,
    suika_sprites: Res<SuikaSprites>,
    mut scores: ResMut<Scores>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if keys.just_released(KeyCode::Space) {
        let (camera, camera_transform) = camera_query.single();
        let window = window_query.single();
        if let Some(world_position) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
        {
            let spawn_position = world_position.x.clamp(-SPAWN_AREA, SPAWN_AREA);
            spawn_suika(&mut commands, &suika_data, current_suika.0, Vec2::new(spawn_position, 1000.), &mut scores, &mut next_state);
            println!("Spawned {:?} at {}", current_suika.0, world_position.x);
            current_suika.0 = get_new_suika();
            let mut suika_image = suika_indicator_query.single_mut();
            *suika_image = suika_sprites.0[current_suika.0.0].clone();
        } else {
            // println!("Cursor is not in the game window.");
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
    mut scores: ResMut<Scores>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    // this is hella sketchy and breaks alot if bad rng
    for collision_event in collision_events.read() {
        if let CollisionEvent::Started(id1, id2, flags) = collision_event {
            if *flags == CollisionEventFlags::SENSOR {
                next_state.set(AppState::Lost);
                return;
            }

            if let Ok(suika1) = suika_query.get(*id1) {
                if let Ok(suika2) = suika_query.get(*id2) {
                    if *suika1 == *suika2 {
                        if let Ok(transform) = suika_query.get_component::<Transform>(*id1){
                            spawn_suika(&mut commands, &suika_data, get_bigger_suika(*suika1), Vec2::new(transform.translation.x, transform.translation.y), &mut scores, &mut next_state)
                        }

                        commands.entity(*id1).despawn();
                        commands.entity(*id2).despawn();
                    }
                }
            }
        }
    }
}