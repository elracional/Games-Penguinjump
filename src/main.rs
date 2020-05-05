//Libreria
#[macro_use]
extern crate stdweb;

//Modulos importados
mod canvas;
mod direction;
mod character;
mod sprite;
mod bitmap;
mod state;
mod gamemap;
mod gamestate;

//Uso de objetos de los modulos importados
use canvas::Canvas;
use character::Character;
use gamestate::GameState;

//Impoartar Event Listeners
use stdweb::traits::*;
use stdweb::web::{event::{KeyDownEvent, KeyUpEvent}, IEventTarget};

//Declarción de uso de variables mutables
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    stdweb::initialize(); //Inicializa la libreria

    //Toma el canvas de HTML con referencia mutable
    let canvas = Canvas::new("#canvas", 20, 16);
    let character = Rc::new(RefCell::new(Character::new(20, 16)));

    //KeyDownEvent Listener
    stdweb::web::document().add_event_listener( {
        let character = character.clone();
        move |event: KeyDownEvent| {
            let key = event.key();
            if character.borrow_mut().game_state == GameState::Play { //Si el juego esta corriendo
                match key.as_ref() {
                    "ArrowLeft" | "a" | "A" => character.borrow_mut().arrow_left_down(),
                    "ArrowRight" | "d" | "D" => character.borrow_mut().arrow_right_down(),
                    "ArrowDown" | "s" | "S" => character.borrow_mut().arrow_down_down(),
                    "ArrowUp" | "w" | "W" => character.borrow_mut().arrow_up_down(),
                    _ => {},
                };
            }
        }
    });

    //KeyUpEvent Listener
    stdweb::web::document().add_event_listener( {
        let character = character.clone();
        move |event: KeyUpEvent| {
            let key = event.key();
            match key.as_ref() {
                "ArrowLeft" | "a" | "A" => character.borrow_mut().arrow_left_up(),
                "ArrowRight" | "d" | "D" => character.borrow_mut().arrow_right_up(),
                "ArrowDown" | "s" | "S" => character.borrow_mut().arrow_down_up(),
                " " => character.borrow_mut().start(), //Inicia el juego con la tecla espacio
                _ => {},
            };
        }
    });

    //Funcion principal, que se encarga de ejecutar el juego
    fn game_loop(character: Rc<RefCell<Character>>, canvas: Rc<Canvas>, time: u32){
        stdweb::web::set_timeout(move || {
                game_loop(character.clone(), canvas.clone(), time); //Se llama a si misma otra vez
                character.borrow().draw(&canvas); //Dibuja el mapa en el canvas
                character.borrow_mut().update(); //Actualiza el estado del personaje
            },
            time, //Tiempo que debe pasar para el siguiente frame (ms)
        );
    }

    //Muestra en el textarea la carga del juego
    js! { addCode("load"); }

    //Llama a la funcion recursiva que corre el juego
    game_loop(character, Rc::new(canvas), 45);

    stdweb::event_loop();
}
