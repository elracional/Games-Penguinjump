//Declaración uso de módulo
use crate::sprite::Sprite;

//Declaración estructura GameMap
#[derive(Debug)]
pub struct GameMap {
    pub map: [[u8; 20]; 16], //Mapa de 20 x 16
    pub sprites: Vec<Sprite>, //Sprites del mapa
    i: u8, //Contador de files
    pub bg: String, //Color de fondo
}

impl GameMap {
    //Función que crea un nuevo GameMap
    pub fn new() -> GameMap {
        GameMap { //Inicialización de valores
            map: [[0; 20]; 16],
            sprites: Vec::new(),
            i: 0,
            bg: String::new(),
        }
    }

    //Función que crea un nuevo GameMap
    pub fn from_text(text: String) -> GameMap {
        let mut map = GameMap::new(); //Crea el nuevo GameMap

        let mut state = 0; //Inicializa estado
        for line in text.lines() { //Lee todas las lineas del archivo
            match line {
                "BackgroundStart"                               => { state = 3 }, //Guarda background
                "MapStart"                                      => { state = 1 }, //Guarda map
                "SpriteStart"                                   => { state = 2 }, //Guarda sprite
                "" | "MapEnd" | "SpriteEnd" | "BackgroundEnd"   => { state = 0 }, //Finaliza
                _ => {
                    match state { //Segpun el estado que guarda
                        1 => { map.add_line(line) }, //Agrega nueva linea del mapa
                        2 => { map.add_sprite(Sprite::from_line(line.to_owned())) }, //Agrega nuevo sprite
                        3 => { map.bg.push_str(line) }, //Agrega color de fondo
                        _ => (),
                    }
                }
            }
        }
        map //Retorna el nueo GameMap
    }

    //Función que agrega nuevas files del mapa
    pub fn add_line(&mut self, line: &str) {
        for (i, c) in line.chars().enumerate() { //Enumera los valores de la linea como char
            if c >= '0' && c <= '9' { //Si es numero
                self.map[self.i as usize][i] = (c as u8 - '0' as u8) as u8; //Cambia de char a int (ASCII)
            } else if c >= 'A' && c <= 'Z' { //Si es letra (mayúscula)
                self.map[self.i as usize][i] = ((c as u8) - ('A' as u8) + (10 as u8) ) as u8; //Cambia de char a int (ASCII)
            }
        }
        self.i += 1; //Aumenta el contador
    }

    //Función que regresa un nuevo sprite
    pub fn add_sprite(&mut self, spte: Sprite) {
        self.sprites.push(spte);
    }
}
