//Estados en que puede estar el juego
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum GameState {
    Start,
    Play,
    GameOver,
}
