use bevy::prelude::*;

/// Game states for state management
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    Loading,
    MainMenu,
    Playing,
    Paused,
    GameOver,
}

/// Main game plugin that sets up the core game loop and states
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state::<GameState>()
            .add_systems(Startup, setup)
            .add_systems(Update, (
                transition_to_main_menu.run_if(in_state(GameState::Loading)),
                auto_transition_to_playing.run_if(in_state(GameState::MainMenu)),
            ));
    }
}

/// Initial setup for the game
fn setup(mut commands: Commands) {
    // Set up camera
    commands.spawn(Camera2dBundle::default());
    
    // Debug message
    info!("Game initialized");
}

/// Transition from Loading to MainMenu state
fn transition_to_main_menu(
    mut next_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
    mut timer: Local<Option<f32>>,
) {
    // Initialize timer if it doesn't exist
    if timer.is_none() {
        *timer = Some(0.0);
        info!("Starting transition to main menu");
    }
    
    // Update timer
    if let Some(t) = timer.as_mut() {
        *t += time.delta_seconds();
        
        // After 1 second, transition to MainMenu
        if *t > 1.0 {
            next_state.set(GameState::MainMenu);
            info!("Transitioned to MainMenu state");
        }
    }
}

/// Automatically transition to Playing state after a short delay
fn auto_transition_to_playing(
    mut next_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
    mut timer: Local<Option<f32>>,
) {
    // Initialize timer if it doesn't exist
    if timer.is_none() {
        *timer = Some(0.0);
    }
    
    // Update timer
    if let Some(t) = timer.as_mut() {
        *t += time.delta_seconds();
        
        // After 0.5 seconds, transition to Playing
        if *t > 0.5 {
            next_state.set(GameState::Playing);
            info!("Transitioned to Playing state");
        }
    }
}