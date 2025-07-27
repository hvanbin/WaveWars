use bevy::prelude::*;
use rand::Rng;
use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Resource, Clone)]
struct GameConfig {
    game: GameSettings,
    display: DisplaySettings,
}

#[derive(Deserialize, Clone)]
struct GameSettings {
    grid_width: i32,
    grid_height: i32,
    cell_size: f32,
    shield_columns: i32,
    clock_rate: f32,
    trail_length: i32,
}

#[derive(Deserialize, Clone)]
struct DisplaySettings {
    window_width: f32,
    window_height: f32,
    window_title: String,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            game: GameSettings {
                grid_width: 20,
                grid_height: 20,
                cell_size: 30.0,
                shield_columns: 1,
                clock_rate: 2.0,
                trail_length: 5,
            },
            display: DisplaySettings {
                window_width: 1200.0,
                window_height: 800.0,
                window_title: "Wave Wars".to_string(),
            },
        }
    }
}

#[derive(Component)]
struct Wave {
    wave_type: WaveType,
    leftward: bool,
    counter: i32,
    target_pos: Vec3,
    previous_pos: Vec3,
    trail_history: Vec<Vec3>, // Store previous positions for trail
}

#[derive(Clone, Copy, PartialEq)]
enum WaveType {
    Sawtooth,
    Square,
    Triangle,
}

#[derive(Component)]
struct Player {
    id: u8, // 1 for left player, 2 for right player
}

#[derive(Component)]
struct SpawnButton {
    wave_type: WaveType,
    row: i32,
    player_id: u8,
}

#[derive(Component)]
struct SpawnerColumn {
    player_id: u8,
}

#[derive(Component)]
struct Explosion {
    timer: f32,
}

#[derive(Component)]
struct Trail {
    timer: f32,
    max_lifetime: f32,
    trail_index: i32, // Which trail segment this is (0 = most recent, higher = older)
}

#[derive(Resource)]
struct GameState {
    winner: u8, // 0 = no winner, 1 = player 1, 2 = player 2, 3 = tie
    ai_enabled: bool,
    clock_rate: f32,
    clock_timer: f32,
    selected_wave_type: WaveType, // Currently selected wave type for player
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            winner: 0,
            ai_enabled: true,
            clock_rate: 2.0,
            clock_timer: 0.0,
            selected_wave_type: WaveType::Sawtooth,
        }
    }
}

impl GameState {
    fn from_config(config: &GameConfig) -> Self {
        Self {
            winner: 0,
            ai_enabled: true,
            clock_rate: config.game.clock_rate,
            clock_timer: 0.0,
            selected_wave_type: WaveType::Sawtooth,
        }
    }
}

#[derive(Resource)]
struct WaveSpawnQueue {
    left_spawns: Vec<(WaveType, i32)>,
    right_spawns: Vec<(WaveType, i32)>,
}

impl Default for WaveSpawnQueue {
    fn default() -> Self {
        Self {
            left_spawns: Vec::new(),
            right_spawns: Vec::new(),
        }
    }
}

fn main() {
    // Load configuration from TOML file
    let config = load_config();
    
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: config.display.window_title.clone(),
                resolution: (config.display.window_width, config.display.window_height).into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(config.clone())
        .insert_resource(GameState::from_config(&config))
        .init_resource::<WaveSpawnQueue>()
        .add_systems(Startup, (setup_camera, setup_board, setup_ui))
        .add_systems(
            Update,
            (
                handle_input,
                update_game_clock,
                spawn_queued_waves,
                move_waves,
                handle_wave_collisions,
                update_explosions,
                update_trails,
                update_spawner_colors,
                ai_player,
                check_win_condition,
            ),
        )
        .run();
}

fn load_config() -> GameConfig {
    match fs::read_to_string("config.toml") {
        Ok(contents) => {
            match toml::from_str(&contents) {
                Ok(config) => config,
                Err(e) => {
                    eprintln!("Error parsing config.toml: {}", e);
                    eprintln!("Using default configuration");
                    GameConfig::default()
                }
            }
        }
        Err(_) => {
            eprintln!("config.toml not found, using default configuration");
            GameConfig::default()
        }
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_board(mut commands: Commands, config: Res<GameConfig>) {
    // Create board grid visualization - using Y for vertical in Bevy 2D (z = -2.0 for background)
    let half_board = config.game.grid_width / 2;
    for x in -half_board..half_board {
        for y in -half_board..half_board {
            let pos = Vec3::new(x as f32 * config.game.cell_size, y as f32 * config.game.cell_size, -2.0);
            commands.spawn((
                Sprite {
                    color: Color::srgb(0.2, 0.2, 0.2),
                    custom_size: Some(Vec2::new(config.game.cell_size - 1.0, config.game.cell_size - 1.0)),
                    ..default()
                },
                Transform::from_translation(pos),
            ));
        }
    }

    // Calculate shield position based on grid size - move left player shields one column left
    let shield_x = half_board - 1 - config.game.shield_columns;
    
    // Create individual shield blocks for left player (Player 1) - moved one column left
    for shield_col in 0..config.game.shield_columns {
        for y in -half_board..half_board {
            commands.spawn((
                Sprite {
                    color: Color::srgb(0.5, 0.5, 0.8),
                    custom_size: Some(Vec2::new(config.game.cell_size - 2.0, config.game.cell_size - 2.0)),
                    ..default()
                },
                Transform::from_translation(Vec3::new(
                    (-shield_x - shield_col - 1) as f32 * config.game.cell_size, // -1 to move left
                    y as f32 * config.game.cell_size, 
                    -1.0
                )),
                Player { id: 1 },
            ));
        }
    }

    // Create individual shield blocks for right player (Player 2)
    for shield_col in 0..config.game.shield_columns {
        for y in -half_board..half_board {
            commands.spawn((
                Sprite {
                    color: Color::srgb(0.8, 0.5, 0.5),
                    custom_size: Some(Vec2::new(config.game.cell_size - 2.0, config.game.cell_size - 2.0)),
                    ..default()
                },
                Transform::from_translation(Vec3::new(
                    (shield_x + shield_col) as f32 * config.game.cell_size, 
                    y as f32 * config.game.cell_size, 
                    -1.0
                )),
                Player { id: 2 },
            ));
        }
    }

    // Create spawner columns behind shields - move left spawner to the edge (zeroth column)
    let spawner_x = half_board - 1;
    for y in -half_board..half_board {
        // Left spawner column (Player 1) - moved to the leftmost edge
        commands.spawn((
            Sprite {
                color: Color::srgb(1.0, 0.5, 0.5), // Default to Sawtooth color
                custom_size: Some(Vec2::new(config.game.cell_size - 4.0, config.game.cell_size - 4.0)),
                ..default()
            },
            Transform::from_translation(Vec3::new(
                -half_board as f32 * config.game.cell_size, // Move to leftmost column (zeroth column)
                y as f32 * config.game.cell_size, 
                0.0
            )),
            SpawnerColumn { player_id: 1 },
        ));

        // Right spawner column (Player 2)
        commands.spawn((
            Sprite {
                color: Color::srgb(0.6, 0.3, 0.3), // Darker Sawtooth color for AI
                custom_size: Some(Vec2::new(config.game.cell_size - 4.0, config.game.cell_size - 4.0)),
                ..default()
            },
            Transform::from_translation(Vec3::new(
                spawner_x as f32 * config.game.cell_size, 
                y as f32 * config.game.cell_size, 
                0.0
            )),
            SpawnerColumn { player_id: 2 },
        ));
    }
}

fn setup_ui(mut commands: Commands, config: Res<GameConfig>) {
    // Calculate spawner column position (same as in setup_board)
    let spawner_x = config.game.grid_width / 2 - 1;
    let half_board = config.game.grid_width / 2;
    
    // Create spawn buttons for left player at the leftmost column - full grid height
    for row in -half_board..half_board {
        let button_pos = Vec3::new(
            -half_board as f32 * config.game.cell_size, // Match the leftmost spawner column position
            (row as f32) * config.game.cell_size, 
            2.0 // z=2.0 for top layer above spawner columns
        );

        commands.spawn((
            Sprite {
                color: Color::srgb(1.0, 1.0, 1.0), // White/neutral color - will be overridden by selected wave type
                custom_size: Some(Vec2::new(20.0, 20.0)),
                ..default()
            },
            Transform::from_translation(button_pos),
            SpawnButton {
                wave_type: WaveType::Sawtooth, // Default, but will use selected wave type
                row,
                player_id: 1,
            },
        ));
    }

    // Create spawn buttons for right player at the right spawner column - full grid height
    for row in -half_board..half_board {
        let button_pos = Vec3::new(
            spawner_x as f32 * config.game.cell_size, 
            (row as f32) * config.game.cell_size, 
            2.0 // z=2.0 for top layer above spawner columns
        );

        commands.spawn((
            Sprite {
                color: Color::srgb(0.6, 0.6, 0.6), // Darker neutral color for AI
                custom_size: Some(Vec2::new(20.0, 20.0)),
                ..default()
            },
            Transform::from_translation(button_pos),
            SpawnButton {
                wave_type: WaveType::Sawtooth, // Default for AI
                row,
                player_id: 2,
            },
        ));
    }

    // UI Text
    commands.spawn((
        Text::new("Wave Wars - Click buttons to spawn waves! Use WASD to switch wave types."),
        TextLayout::new_with_justify(JustifyText::Left),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
    ));
}

fn handle_input(
    mouse_button: Res<ButtonInput<MouseButton>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    buttons: Query<(&Transform, &SpawnButton)>,
    mut spawn_queue: ResMut<WaveSpawnQueue>,
    mut game_state: ResMut<GameState>,
) {
    // Handle keyboard input for wave type selection
    if keyboard.just_pressed(KeyCode::KeyW) {
        game_state.selected_wave_type = WaveType::Sawtooth;
    } else if keyboard.just_pressed(KeyCode::KeyA) {
        game_state.selected_wave_type = WaveType::Square;
    } else if keyboard.just_pressed(KeyCode::KeyS) {
        game_state.selected_wave_type = WaveType::Triangle;
    }
    
    // Handle mouse input for spawning waves
    if mouse_button.just_pressed(MouseButton::Left) {
        let window = windows.single();
        if let Some(cursor_pos) = window.cursor_position() {
            let (camera, camera_transform) = camera_q.single();
            
            if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) {
                // Check if clicked on a spawn button
                for (transform, button) in buttons.iter() {
                    let button_pos = transform.translation.truncate();
                    let distance = world_pos.distance(button_pos);
                    
                    if distance < 20.0 {
                        // Only allow left player (player 1) to spawn manually
                        if button.player_id == 1 || !game_state.ai_enabled {
                            if button.player_id == 1 {
                                // Use selected wave type instead of button wave type
                                spawn_queue.left_spawns.push((game_state.selected_wave_type, button.row));
                            } else {
                                spawn_queue.right_spawns.push((button.wave_type, button.row));
                            }
                        }
                        break;
                    }
                }
            }
        }
    }
}

fn update_game_clock(
    mut game_state: ResMut<GameState>,
    time: Res<Time>,
) {
    game_state.clock_timer += time.delta_secs();
}

fn spawn_queued_waves(
    mut commands: Commands,
    mut spawn_queue: ResMut<WaveSpawnQueue>,
    mut game_state: ResMut<GameState>,
    config: Res<GameConfig>,
) {
    if game_state.clock_timer >= game_state.clock_rate {
        // Spawn left waves (human player - leftward=true means going rightward)
        for (wave_type, row) in spawn_queue.left_spawns.drain(..) {
            spawn_wave(&mut commands, wave_type, row, true, &config);
        }

        // Spawn right waves (AI player - leftward=false means going leftward)
        for (wave_type, row) in spawn_queue.right_spawns.drain(..) {
            spawn_wave(&mut commands, wave_type, row, false, &config);
        }

        game_state.clock_timer = 0.0;
    }
}

fn spawn_wave(
    commands: &mut Commands,
    wave_type: WaveType,
    row: i32,
    leftward: bool,
    config: &GameConfig,
) {
    // Spawn waves at the spawner column positions (where the buttons are now)
    let half_board = config.game.grid_width / 2;
    let spawner_x = config.game.grid_width / 2 - 1;
    let spawn_x = if leftward { 
        -half_board as f32 * config.game.cell_size  // Leftmost column for human player
    } else { 
        spawner_x as f32 * config.game.cell_size    // Original position for AI player
    };
    let pos = Vec3::new(spawn_x, row as f32 * config.game.cell_size, 1.0); // z=1.0 above spawner columns
    
    // Color by wave type, with brightness indicating player side
    let brightness = if leftward { 1.0 } else { 0.6 }; // Human brighter, AI darker
    let color = match wave_type {
        WaveType::Sawtooth => Color::srgb(brightness, brightness * 0.5, brightness * 0.5), // Red
        WaveType::Square => Color::srgb(brightness * 0.5, brightness, brightness * 0.5),   // Green
        WaveType::Triangle => Color::srgb(brightness * 0.5, brightness * 0.5, brightness), // Blue
    };

    commands.spawn((
        Sprite {
            color,
            custom_size: Some(Vec2::new(30.0, 30.0)),
            ..default()
        },
        Transform::from_translation(pos),
        Wave {
            wave_type,
            leftward,
            counter: 0,
            target_pos: pos,
            previous_pos: pos,
            trail_history: Vec::new(),
        },
    ));
}

fn move_waves(
    mut commands: Commands,
    mut waves: Query<(&mut Wave, &mut Transform)>,
    mut game_state: ResMut<GameState>,
    config: Res<GameConfig>,
) {
    // Update wave movement logic only when clock reaches the rate (once per cycle)
    if game_state.clock_timer >= game_state.clock_rate {
        for (mut wave, transform) in waves.iter_mut() {
            // Create trail at current position before moving
            spawn_trail(&mut commands, transform.translation, &wave);
            
            // Calculate movement based on wave type and counter
            let counter = wave.counter;
            match wave.wave_type {
                WaveType::Sawtooth => {
                    // Sawtooth: move forward every other turn, alternate up/down each turn
                    if counter % 2 == 0 {
                        shift_x(&mut wave, 1, &config); // Only move forward on even turns
                    }
                    shift_y(&mut wave, counter % 2 == 1, &config); // Alternate up/down
                    wave.counter += 1;
                }
                WaveType::Square => {
                    // Square: up, forward, down, forward pattern
                    match counter % 4 {
                        0 => shift_y(&mut wave, true, &config),  // up
                        1 => shift_x(&mut wave, 1, &config),     // forward
                        2 => shift_y(&mut wave, false, &config), // down
                        3 => shift_x(&mut wave, 1, &config),     // forward
                        _ => {}
                    }
                    wave.counter += 1;
                }
                WaveType::Triangle => {
                    // Triangle: forward and alternate up/down
                    let should_move_up = counter % 2 == 0;
                    shift_x(&mut wave, 1, &config);
                    shift_y(&mut wave, should_move_up, &config);
                    wave.counter += 1;
                }
            }
        }
    }

    // Smooth interpolation towards target position
    for (wave, mut transform) in waves.iter_mut() {
        let target = wave.target_pos;
        let distance = transform.translation.distance(target);
        if distance > 0.1 {
            transform.translation = transform.translation.lerp(target, 0.1);
        } else {
            transform.translation = target;
        }
    }
}

fn shift_x(wave: &mut Wave, amount: i32, config: &GameConfig) {
    let direction = if wave.leftward { 1.0 } else { -1.0 };
    // Move by full cell size to align with grid
    wave.target_pos.x += direction * amount as f32 * config.game.cell_size;
}

fn shift_y(wave: &mut Wave, up: bool, config: &GameConfig) {
    let direction = if up { 1.0 } else { -1.0 };
    // Move by full cell size to align with grid
    wave.target_pos.y += direction * config.game.cell_size;
}

fn handle_wave_collisions(
    mut commands: Commands,
    waves: Query<(Entity, &Transform, &Wave)>,
    players: Query<(Entity, &Transform, &Player)>,
    mut game_state: ResMut<GameState>,
    config: Res<GameConfig>,
) {
    let mut waves_to_remove = Vec::new();
    let mut players_to_remove = Vec::new();

    // Check for waves that made it past the shields (win condition)
    for (wave_entity, wave_transform, wave) in waves.iter() {
        let wave_grid_x = (wave_transform.translation.x / config.game.cell_size).round() as i32;
        let win_threshold = config.game.grid_width / 2 - 1;
        
        // Check if wave made it past the shields
        if (wave.leftward && wave_grid_x >= win_threshold) || (!wave.leftward && wave_grid_x <= -win_threshold) {
            // Wave made it past shields - player who sent it wins
            if game_state.winner == 0 {
                game_state.winner = if wave.leftward { 1 } else { 2 };
            }
            waves_to_remove.push(wave_entity);
        }
    }

    // Check wave-wave collisions (only when they end up on the same grid tile)
    let wave_list: Vec<_> = waves.iter().collect();
    for i in 0..wave_list.len() {
        for j in (i + 1)..wave_list.len() {
            let (entity1, transform1, wave1) = wave_list[i];
            let (entity2, transform2, wave2) = wave_list[j];

            if wave1.leftward != wave2.leftward {
                // Check if waves are on the same grid tile (not just close distance)
                let grid_x1 = (transform1.translation.x / config.game.cell_size).round() as i32;
                let grid_y1 = (transform1.translation.y / config.game.cell_size).round() as i32;
                let grid_x2 = (transform2.translation.x / config.game.cell_size).round() as i32;
                let grid_y2 = (transform2.translation.y / config.game.cell_size).round() as i32;
                
                if grid_x1 == grid_x2 && grid_y1 == grid_y2 {
                    waves_to_remove.push(entity1);
                    waves_to_remove.push(entity2);
                    
                    // Spawn explosion effect
                    spawn_explosion(&mut commands, transform1.translation);
                }
            }
        }
    }

    // Check wave-shield collisions (grid-based)
    for (wave_entity, wave_transform, wave) in waves.iter() {
        for (player_entity, player_transform, player) in players.iter() {
            // Check if wave and shield are on the same grid tile
            let wave_grid_x = (wave_transform.translation.x / config.game.cell_size).round() as i32;
            let wave_grid_y = (wave_transform.translation.y / config.game.cell_size).round() as i32;
            let shield_grid_x = (player_transform.translation.x / config.game.cell_size).round() as i32;
            let shield_grid_y = (player_transform.translation.y / config.game.cell_size).round() as i32;
            
            if wave_grid_x == shield_grid_x && wave_grid_y == shield_grid_y {
                // Wave hit a shield - destroy both and create explosion
                if (wave.leftward && player.id == 2) || (!wave.leftward && player.id == 1) {
                    waves_to_remove.push(wave_entity);
                    players_to_remove.push(player_entity);
                    
                    spawn_explosion(&mut commands, wave_transform.translation);
                }
            }
        }
    }

    // Remove collided entities (deduplicate to avoid double-despawn warnings)
    waves_to_remove.sort();
    waves_to_remove.dedup();
    players_to_remove.sort();
    players_to_remove.dedup();
    
    for entity in waves_to_remove {
        commands.entity(entity).despawn();
    }
    for entity in players_to_remove {
        commands.entity(entity).despawn();
    }
}

fn spawn_explosion(commands: &mut Commands, position: Vec3) {
    commands.spawn((
        Sprite {
            color: Color::srgb(1.0, 0.5, 0.0),
            custom_size: Some(Vec2::new(50.0, 50.0)),
            ..default()
        },
        Transform::from_translation(position),
        Explosion { timer: 0.0 },
    ));
}

fn spawn_trail(commands: &mut Commands, position: Vec3, wave: &Wave) {
    // Color trail based on wave type but more transparent
    let brightness = if wave.leftward { 0.8 } else { 0.4 }; // Dimmer than main wave
    let mut color = match wave.wave_type {
        WaveType::Sawtooth => Color::srgb(brightness, brightness * 0.5, brightness * 0.5), // Red
        WaveType::Square => Color::srgb(brightness * 0.5, brightness, brightness * 0.5),   // Green
        WaveType::Triangle => Color::srgb(brightness * 0.5, brightness * 0.5, brightness), // Blue
    };
    color.set_alpha(0.6); // Make trail semi-transparent

    commands.spawn((
        Sprite {
            color,
            custom_size: Some(Vec2::new(15.0, 15.0)), // Smaller than main wave
            ..default()
        },
        Transform::from_translation(position),
        Trail { 
            timer: 0.0,
            max_lifetime: 1.0, // Trail lasts 1 second
            trail_index: 0, // Most recent trail segment
        },
    ));
}

fn update_explosions(
    mut commands: Commands,
    mut explosions: Query<(Entity, &mut Explosion)>,
    time: Res<Time>,
    game_state: Res<GameState>,
) {
    for (entity, mut explosion) in explosions.iter_mut() {
        explosion.timer += time.delta_secs();
        
        // Remove explosion after one game cycle (clock_rate seconds)
        if explosion.timer >= game_state.clock_rate {
            commands.entity(entity).despawn();
        }
    }
}

fn update_trails(
    mut commands: Commands,
    mut trails: Query<(Entity, &mut Trail, &mut Sprite)>,
    time: Res<Time>,
) {
    for (entity, mut trail, mut sprite) in trails.iter_mut() {
        trail.timer += time.delta_secs();
        
        // Fade out trail over time
        let alpha = 1.0 - (trail.timer / trail.max_lifetime);
        sprite.color.set_alpha(alpha * 0.6); // Start at 0.6 alpha, fade to 0
        
        // Remove trail when lifetime expires
        if trail.timer >= trail.max_lifetime {
            commands.entity(entity).despawn();
        }
    }
}

fn update_spawner_colors(
    mut spawner_columns: Query<(&mut Sprite, &SpawnerColumn)>,
    game_state: Res<GameState>,
) {
    for (mut sprite, spawner) in spawner_columns.iter_mut() {
        if spawner.player_id == 1 {
            // Update left spawner column to match selected wave type
            let brightness = 1.0;
            sprite.color = match game_state.selected_wave_type {
                WaveType::Sawtooth => Color::srgb(brightness, brightness * 0.5, brightness * 0.5), // Red
                WaveType::Square => Color::srgb(brightness * 0.5, brightness, brightness * 0.5),   // Green
                WaveType::Triangle => Color::srgb(brightness * 0.5, brightness * 0.5, brightness), // Blue
            };
        }
        // Right spawner column (AI) keeps its default color for now
    }
}

fn ai_player(
    mut spawn_queue: ResMut<WaveSpawnQueue>,
    game_state: Res<GameState>,
    config: Res<GameConfig>,
) {
    if game_state.ai_enabled && game_state.clock_timer >= game_state.clock_rate * 0.8 {
        let mut rng = rand::thread_rng();
        
        // Only spawn one wave per turn, not multiple
        if rng.gen_bool(0.3) && spawn_queue.right_spawns.is_empty() {
            let wave_type = match rng.gen_range(0..3) {
                0 => WaveType::Sawtooth,
                1 => WaveType::Square,
                _ => WaveType::Triangle,
            };
            let half_board = config.game.grid_width / 2;
            let row = rng.gen_range(-half_board..half_board);
            spawn_queue.right_spawns.push((wave_type, row));
        }
    }
}

fn check_win_condition(
    game_state: Res<GameState>,
    mut commands: Commands,
) {
    if game_state.winner != 0 {
        let winner_text = match game_state.winner {
            1 => "Player 1 (Left) Wins!",
            2 => "Player 2 (Right) Wins!",
            3 => "It's a Tie!",
            _ => "Game Over",
        };

        commands.spawn((
            Text::new(winner_text),
            TextLayout::new_with_justify(JustifyText::Center),
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(300.0),
                left: Val::Px(400.0),
                ..default()
            },
        ));
    }
}
