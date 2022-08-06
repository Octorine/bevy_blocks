
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum GameState {
    MainMenu,
    PauseMenu,
    Level,
    GameOver,
}
