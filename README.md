# Wave Wars - Rust/Bevy Edition

A complete rewrite of the Unity Wave Wars game in Rust using the Bevy game engine. This is a strategic grid-based game where players spawn different types of waves to break through their opponent's shields.

## Game Overview

Wave Wars is a turn-based strategy game where two players face off on a grid battlefield. Players spawn waves with different movement patterns to try to break through their opponent's shields and reach the other side of the board.

### Key Features

- **Three Wave Types**: Each with unique movement patterns
  - **Sawtooth Wave** (Red): Moves forward every other turn, alternates up/down each turn
  - **Square Wave** (Green): Follows up → forward → down → forward pattern
  - **Triangle Wave** (Blue): Moves forward and alternates up/down each turn

- **Strategic Gameplay**: 
  - Click spawn buttons to deploy waves
  - Waves collide and destroy each other when they meet
  - Break through opponent shields to win
  - AI opponent provides challenging gameplay

- **Visual Effects**:
  - Smooth wave movement with interpolation
  - Trail effects showing wave paths
  - Explosion effects on collisions
  - Color-coded waves and UI elements

## Controls

- **WASD Keys**: Switch between wave types
  - `W` - Select Sawtooth Wave (Red)
  - `A` - Select Square Wave (Green) 
  - `S` - Select Triangle Wave (Blue)

- **Mouse**: Click spawn buttons to deploy waves
  - Spawn buttons are located directly on the spawner columns (behind shields)
  - Buttons extend across the full grid height for maximum strategic options
  - Selected wave type is shown by spawner column color

## Installation & Running

### Prerequisites

- Rust (latest stable version)
- Cargo (comes with Rust)

### Quick Start

1. Clone or download the project
2. Navigate to the project directory
3. Run the game:

```bash
cargo run --release
```

The game will compile and launch automatically.

## Configuration

The game can be customized via `config.toml`:

```toml
[game]
grid_width = 20          # Board width in cells
grid_height = 20         # Board height in cells  
cell_size = 30.0         # Size of each grid cell in pixels
shield_columns = 1       # Number of shield columns per player
clock_rate = 2.0         # Seconds between wave movement cycles

[display]
window_width = 1200.0    # Game window width
window_height = 800.0    # Game window height
window_title = "Wave Wars"
```

## Game Mechanics

### Wave Movement

Each wave type has a distinct movement pattern executed in discrete steps:

- **Sawtooth**: Alternates between moving forward and changing vertical direction
- **Square**: Follows a 4-step cycle creating square-like patterns  
- **Triangle**: Moves diagonally, creating triangular wave patterns

### Combat System

- Waves from opposing players destroy each other on collision
- Waves destroy shield blocks when they collide
- First wave to reach the opponent's back line wins the game
- Strategic positioning and timing are key to victory

### AI Opponent

The AI player automatically spawns waves with:
- Random wave type selection
- Random row targeting
- Configurable spawn probability (30% chance per turn)

## Technical Implementation

### Architecture

- **Bevy ECS**: Entity Component System for game objects
- **Component-based Design**: Waves, shields, trails, and explosions as entities
- **Resource Management**: Game state and configuration as Bevy resources
- **System-based Logic**: Separate systems for movement, collision, AI, etc.

### Key Systems

- `move_waves`: Handles wave movement patterns and interpolation
- `handle_wave_collisions`: Grid-based collision detection
- `spawn_queued_waves`: Turn-based wave spawning
- `ai_player`: Automated opponent behavior
- `update_trails`: Visual trail effects with fade-out

### Performance Features

- Grid-based collision detection for efficiency
- Entity pooling through despawn/respawn cycles
- Smooth interpolation for visual appeal
- Optimized rendering with z-layering

## Development Notes

This project demonstrates several Rust/Bevy concepts:

- **ECS Architecture**: Clean separation of data and behavior
- **Resource Management**: Configuration and game state handling
- **Event-driven Programming**: Input handling and game events
- **Component Composition**: Flexible entity definitions
- **System Scheduling**: Coordinated game loop execution

### Code Structure

- `main.rs`: Complete game implementation
- `config.toml`: Game configuration
- `Cargo.toml`: Rust dependencies and metadata

## Future Enhancements

Potential improvements for the game:

- [ ] Multiplayer networking support
- [ ] Additional wave types with unique patterns
- [ ] Power-ups and special abilities
- [ ] Campaign mode with progressive difficulty
- [ ] Sound effects and background music
- [ ] Particle effects for enhanced visuals
- [ ] Save/load game state functionality
- [ ] Tournament mode with multiple rounds

## Credits

- Original Unity concept: Wave Wars
- Rust/Bevy implementation: Complete rewrite
- Game Engine: [Bevy](https://bevyengine.org/)
- Language: [Rust](https://www.rust-lang.org/)

## License

This project is open source. Feel free to modify and distribute according to your needs.
