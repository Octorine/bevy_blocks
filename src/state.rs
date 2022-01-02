use bevy::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum GameState {
    MainMenu,
    PauseMenu,
    Level,
    PreLevel,
}
