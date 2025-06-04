# Aesop Trials

A mythological-themed 2D roguelike dungeon crawler inspired by the Trials of King Hercules, featuring procedurally generated dungeons, turn-based combat, and Greek mythology elements.

## Phase 1 Implementation

This initial implementation includes:

- Basic game loop with Bevy states (Loading, MainMenu, Playing, Paused, GameOver)
- Simple grid-based movement system with WASD/arrow keys
- Basic tile rendering with placeholder graphics (colored squares)
- Player character with basic stats
- Simple rectangular room generation
- Basic collision detection to prevent walking through walls
- Procedural dungeon generation with multiple connected rooms

## Getting Started

### Prerequisites

- Rust and Cargo (https://rustup.rs/)

### Running the Game

```bash
# Clone the repository
git clone <repository-url>
cd aesop_trials

# Build and run the game
cargo run
```

### Controls

- WASD or Arrow Keys: Move the player
- Space: Start the game from the main menu
- Escape: Pause/Resume the game

## Project Structure

The project follows the structure outlined in the development plan:

- `src/main.rs`: Entry point and app setup
- `src/core/`: Core game systems (game states, input handling, rendering)
- `src/entities/`: Entity definitions (player, enemies, items)
- `src/systems/`: Game systems (collision detection)
- `src/world/`: World generation (dungeon, rooms, tiles)
- `src/ui/`: User interface components
- `src/plugins/`: Bevy plugins for various game systems

## Next Steps

The next phase (Phase 2) will focus on implementing:

- Turn-based combat mechanics
- Enemy AI with basic pathfinding
- Damage calculation system
- Health/death mechanics
- Basic enemy types
- Combat UI elements