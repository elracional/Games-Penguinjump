//Importación del modulo de dirección
use crate::direction::Direction;

//Estados en que puede estar el pesonaje
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum State {
    Stand,
    Walk(Direction),
    Jump(Direction),
    Fall(Direction),
    Crouch,
}
