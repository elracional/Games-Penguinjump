//Uso modulo stdweb para cast variables
use stdweb::unstable::TryInto;

//Declaración de módulos a utilizar
use crate::canvas::Canvas;
use crate::direction::Direction;
use crate::sprite::Sprite;
use crate::gamemap::GameMap;
use crate::state::State;
use crate::gamestate::GameState;

//Creación de structura para coordenadas
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Coord(u32, u32);

//Estructura del personaje
#[derive(Debug)]
pub struct Character {
    coord: Coord, //Coordendas
    sprites: [Vec<Sprite>; 4], //Sprites según su estado
    state: State, //Estado
    pub last_state: State, //Estado anterior
    stt_ix: usize, //Estado indice del sprite
    height: u32, //Altura canvas
    width: u32, //Ancho canvas
    dir: Direction, //Dirección del personaje
    last_dir: Direction, //Ultima dirección del personaje

    pub game_state: GameState, //Estao del juego

    it: usize, //Contador, para llevar control de las reglas del juego, avanza en cada frame
    max_it: usize, //Valor maximo del contador

    speed: usize, //Velocidad del movimiento (en relación a it) de las plataformas

    map_start: GameMap, //Mapa de start
    map: GameMap, //Mapa de juego
    map_end: GameMap, //Mapa de game over

    jump_h: u8, //Contador bloques de salto
    max_jump_h: u8, //Maxima altura de salto

    down_key: bool, //Guarda si se deja presionada la dirección hacia abajo

    lifes: u8, //Numero de vidas

    score: u16, //Socre del juego

    //Variables para impresion de funciones en textare
    p_dir: String, //Dirección
    p_state: String, //Estado
    lt_p_dir: String, //Ultima dirección
    lt_p_state: String, //Ultimo estado
}

impl Character {
    //Funcion de crear nuevo personaje
    pub fn new(width: u32, height: u32) -> Character {
        let coord = Coord(9, 11); //Coordenadas iniciales
        let mut sprites = [ //Spites de cada estado
            Vec::new(), //Stand
            Vec::new(), //Walk
            Vec::new(), //Jump
            Vec::new(), //Crouch
        ];

        //Carga los sprites desde el archivo
        let hash = Sprite::hash_from_text(String::from(include_str!("files/sprites.txt")));

        //Agrega los sprites de Stand
        let spte: Sprite = hash.get("Stand1").unwrap().deref();
        let spte2: Sprite = hash.get("Stand2").unwrap().deref();
        sprites[0].push(spte);
        sprites[0].push(spte2);

        //Agrega los sprites de Walk
        let spte: Sprite = hash.get("Walk1").unwrap().deref();
        let spte2: Sprite = hash.get("Walk2").unwrap().deref();
        sprites[1].push(spte);
        sprites[1].push(spte2);

        //Agrega los sprites de Jump
        let spte: Sprite = hash.get("Jump1").unwrap().deref();
        sprites[2].push(spte);
        let spte: Sprite = hash.get("Jump1").unwrap().deref();
        sprites[2].push(spte);
        let spte: Sprite = hash.get("Jump2").unwrap().deref();
        sprites[2].push(spte);
        let spte: Sprite = hash.get("Jump2").unwrap().deref();
        sprites[2].push(spte);

        //Agrega los sprites de Crouch
        let spte: Sprite = hash.get("Crouch").unwrap().deref();
        sprites[3].push(spte);

        //Inicializa todas las varibles
        Character {
            coord,
            sprites,

            state: State::Stand,
            last_state: State::Stand,
            stt_ix: 0,

            game_state: GameState::Start,

            height,
            width,

            dir: Direction::Right,
            last_dir: Direction::Right,

            it: 0,
            max_it: 255,

            speed: 128,

            //Carga de mapas
            map_start: GameMap::from_text(String::from(include_str!("files/mapStart.txt"))),
            map: GameMap::from_text(String::from(include_str!("files/map.txt"))),
            map_end: GameMap::from_text(String::from(include_str!("files/mapEnd.txt"))),

            jump_h: 0,
            max_jump_h: 4,
            lifes: 2,
            score: 0,

            down_key: false,

            p_dir: "".to_string(),
            p_state: "".to_string(),
            lt_p_dir: "".to_string(),
            lt_p_state: "".to_string(),
        }
    }

    //Funcion que cambia dirección de sprites del personaje
    pub fn change_dir(&mut self, dir: Direction) {
        if self.last_dir != dir && dir != Direction::None { //Si la dirección cambio y no es a None
            self.invert_side(); //Invierte dirección del sprite
            self.last_dir = dir; //Actuliza la dirección anterior
        }
        self.dir = dir; //Actualiza la dirección actual
    }

    //Funión de actulización de valores, personaje y mapa
    pub fn update(&mut self) {
        if self.game_state == GameState::Start { //Si el juego no esta iniciado regresa
            return
        }

        self.load_hud(); //Carga vidas y puntaje de la partida

        if self.game_state == GameState::GameOver { //Si el juego esta en GameOver regresa
            return
        }

        //Se muestra en el textarea las los movimientos que va haciendo el personaje
        if self.p_dir != self.lt_p_dir || self.p_state != self.lt_p_state { //Si cambio la dirección o el estado
            js! { addCode(@{self.p_state.to_string()}, @{self.p_dir.to_string()}); } //Agrega al texarea el movimiento
            self.lt_p_dir = self.p_dir.to_string(); //Actuliza dirección anterior
            self.lt_p_state = self.p_state.to_string(); //Actualiza estado anterior
        }

        //El el personaje llega hasta abajo de la mapa y no hay plataforma
        if self.coord.1 as usize + 1 == 16 {
            if self.lifes > 0 { //Si aun tiene vidas
                js! { addCode("lifeMinus"); }
                self.lifes-=1; //Resta un vida
                js! { addCode("respawn"); }
                self.coord.1 = 6; //El personaje reaparece arriba
            }
            else { //Si no tiene vidas
                js! { stopMusic(); } //Se detiene la musica
                js! { addCode("gameOver"); }
                self.game_state = GameState::GameOver; //Cambia el estado a GameOver
                return
            }
        }

        //Si la dirección abajo ha sido presionada antes y hay piso
        if self.down_key && !self.no_floor() {
            self.set_state(State::Crouch); //Cambia de estado a Crouch
        }

        //Si esta en estado Crouch
        if self.state == State::Crouch {
            self.p_dir = "".to_string();
            self.p_state = "crouch".to_string();

            if !self.down_key { //Si la tecla hacia abajo ya no esta presionada
                self.set_state(State::Stand); //Regresa a estado Stand
            }

            //Si en la posición hay un corazón (marcado como 4 en el mapa) y las vidas son menores a 4
            if self.map.map[self.coord.1 as usize][self.coord.0 as usize] == 4 && self.lifes < 4 {
                js! { playLife(); }
                js! { addCode("lifePlus"); }
                self.lifes += 1; //Agrega una vida al personaje
                self.map.map[self.coord.1 as usize][self.coord.0 as usize] = 0; //Quita el corazón del mapa
            }
        }

        //Si en la posición del personaje hay un pez (marcado como 5 en el mapa)
        if self.map.map[self.coord.1 as usize][self.coord.0 as usize] == 5 {
            js! { playFish(); }
            js! { addCode("eatFish"); }
            self.score += 1; //Aumenta el score del personaje
            self.map.map[self.coord.1 as usize][self.coord.0 as usize] = 0; //Quita el pez del mapa

            if self.score == 10 { self.speed = 64; } //Cuando se llega a un score de 10 aumenta la velocidad de las plataformas
            if self.score == 50 { self.speed = 32; } //Cuando se llega a un score de 50 aumenta la velocidad de las plataformas
            if self.score == 100 { self.speed = 16; }  //Cuando se llega a un score de 100 aumenta la velocidad de las plataformas
            if self.score == 175 { self.speed = 8; }  //Cuando se llega a un score de 175 aumenta la velocidad de las plataformas
        }

        //Dependiendo del estado actual de personaje (switch)
        match self.state {
            //Si esta en estado Walk
            State::Walk(dir) => {
                if self.it % 2 == 0 { //Actuliza la pantalla cada 2 iteraciones de tiempo
                    self.change_dir(dir); //Cambia la dirección si es necesario
                    let new_coord = match dir { //Segun la dirección
                        Direction::Right => { //Avanza a la derecha
                            self.p_dir = "Right".to_string();
                            Coord( (self.coord.0 + 1) % self.width, self.coord.1 )
                        },
                        Direction::Left => { //Avanza a la izquierda
                            self.p_dir = "Left".to_string();
                            Coord( (self.coord.0 + self.width - 1) % self.width, self.coord.1 )
                        },
                        Direction::None => { //no se mueve
                            self.p_dir = "".to_string();
                            Coord( self.coord.0, self.coord.1 )
                        },
                    };
                    self.p_state = "walk".to_string();

                    self.coord = new_coord; //Cambia las coordenadas el personaje
                    if self.no_floor() { //Si no hay piso en la nueva coordenada
                        self.set_state(State::Fall(dir)); //El estado cambia a Fall
                    }
                }
            },

            //Si esta en estado Jump
            State::Jump(dir) => {
                if self.it % 2 == 0 { //Actuliza la pantalla cada 2 iteraciones de tiempo
                    self.change_dir(dir); //Cambia la dirección si es necesario
                    if self.jump_h < self.max_jump_h { //Si esta en estado Jump y aun no llega a la altura máxima
                        //Si no se sale de la pantalla sube una posición, sino ahpi se mantiene
                        let coord_y = if self.coord.1 + self.height - 1 < self.height { self.coord.1 } else { self.coord.1 - 1 };
                        let new_coord = match dir { //Segun la dirección
                            Direction::Right => { //Salto con dirección a la derecha
                                self.p_dir = "Right".to_string();
                                Coord( (self.coord.0 + 1) % self.width, coord_y )
                            },
                            Direction::Left => { //Salto con dirección a la izquierda
                                self.p_dir = "Left".to_string();
                                Coord( (self.coord.0 + self.width - 1) % self.width, coord_y )
                            },
                            Direction::None => { //Salto solo hacia arriba
                                self.p_dir = "".to_string();
                                Coord( self.coord.0, coord_y )
                            },
                        };

                        self.p_state = "jump".to_string();

                        self.coord = new_coord; //Cambia las coordenadas el personaje
                        self.jump_h += 1; //Aumenta el contador de salto máximo
                    } else { //Si ya llegó a lo más alto del salto, empieza a caer
                        self.set_state(State::Fall(self.dir)); //Empieza a caer con la misma dirección de salto
                        self.jump_h = 0; //Se reinicia el contadro del altura de salto
                    }
                }
            },

            //Si esta en estado Fall
            State::Fall(dir) => {
                if self.it % 2 == 0 { //Actuliza la pantalla cada 2 iteraciones de tiempo
                    self.change_dir(dir); //Cambia la dirección si es necesario
                    if self.no_floor() { //Si aun no tiene piso el personaje en los pies
                        let new_coord = match dir { //Segun la dirección
                            Direction::Right => { //Caida con dirección a la derecha
                                self.p_dir = "Right".to_string();
                                Coord( (self.coord.0 + 1) % self.width, self.coord.1 + 1 )
                            },
                            Direction::Left => { //Caida con dirección a la izquierda
                                self.p_dir = "Left".to_string();
                                Coord( (self.coord.0 + self.width - 1) % self.width, self.coord.1 + 1 )
                            },
                            Direction::None => { //Caida solo hacia abajo
                                self.p_dir = "".to_string();
                                Coord( self.coord.0, self.coord.1 + 1 )
                            },
                        };

                        self.p_state = "fall".to_string();

                        self.coord = new_coord; //Cambia las coordenadas el personaje
                        self.set_state(State::Fall(self.dir)); //Cambia la dirección de la caida
                    }
                    else { //Si ya tiene piso
                        if self.dir == Direction::None { //Si no tiene dirección
                            self.set_state(State::Stand); //Cambia a estado Stand
                        }
                        else { //Si tiene dirección
                            self.set_state(State::Walk(self.dir)); //Cambia a estado Walk con la misma dirección de caida
                        }

                    }
                }
            },

            //Si esta en estado Stand
            State::Stand => {
                self.p_dir = "".to_string();
                self.p_state = "stand".to_string();
                 //Actuliza la pantalla cada 2 iteraciones de tiempo
                if self.it % 2 == 0 && self.no_floor() { //Si ya no tiene piso
                    self.set_state(State::Fall(Direction::None)); //Cambia es tado Fall sin dirección
                }
            },
            _ => (),
        }

        //Este if ayuda a mover las plataformas
        if self.it % self.speed == 3 { //Si el iterador de tiempo cumple la condición, bajan
            let mut new_line:[u8; 20] = [0; 20]; //Nuevo vector para crear la nueva linea del mapa

            for _n in 0..64 { //Prueba 64 posibilidades de plataforma
                let mut flag = true; //Bandera de validación
                let n_platform:u8 = js! { return Math.floor(Math.random() * ( 2 ) + 1); } //Numero de plataformas random
                    .try_into()
                    .unwrap();

                for _i in 0..n_platform { //Segun el numero de plataformas a crear
                    let n:u8 = js! { return Math.floor(Math.random() * ( 4 ) + 2); } //Numero random de ancho de la plataforma
                        .try_into()
                        .unwrap();
                    let x:u8 = js! { return Math.floor(Math.random() * ( @{self.width} )); } //Posición random de la plataforma
                        .try_into()
                        .unwrap();

                    for j in x..(x + n) { //Recorre las posiciones de la nueva plataforma
                        if j >= 20 { continue; } // Si j es mayor o igual a 20, salta a la siguiente interación
                        if new_line[j as usize] == 0 { //Agrega la nueva paltaforma si no hay nada en la posición
                            new_line[j as usize] = 2;
                        } else { new_line[j as usize] = 0; }  //Si hay algo lo cambia a nada
                        if j > 0 && j < 19 { //Si j esta entre 0 y 19
                            //Si poniendo la nueva plataforma en el mapa hubiera ya otra en diagonal
                            if new_line[j as usize] == 2 && (self.map.map[3][j as usize + 1] == 2 || self.map.map[3][j as usize - 1] == 2) {
                                flag =false;  //Pone en falso la bandera
                            }
                        }
                    }
                }

                for i in 0..self.width { //Recorre toda la nueva linea para verificarla
                    if new_line[i as usize] == 2 { //Si hay plataforma en una posición
                        //Pero tambien hay ya en el mapa dos posiciones abajo plataforma (es para que no queden pegadas)
                        if self.map.map[4][i as usize] == 2 || self.map.map[3][i as usize] == 2 {
                            flag = false; //Pone en falso la bandera
                        }
                    }
                }

                if flag { break; } //Si la bandera es verdadera, sale
                new_line = [0; 20]; //Reinicia la nueva linea
            }

            //Agrega la nueva linea al mapa, en la parte superior, por lo que recorre el mapa de abajo para arriba
            for j in 1..(self.height - 3) { //Va a iterar sobre todas las filas del mapa actual para deslizarlo hacia abajo
                for i in 0..self.width { //Recorre todas las columnas
                    //Reemplaza una linea por la que esta arriba de ella
                    self.map.map[self.height as usize - j as usize][i as usize] = self.map.map[self.height as usize - j as usize - 1][i as usize];
                }
            }

            for i in 0..self.width { //Agrega la nueva liena en la fila 3
                self.map.map[3][i as usize] = new_line[i as usize];
            }

            self.coord.1 += 1; //Aumeta la coordenada en y del personaje, para que se mueva con el mapa
        }

        //Este if ayuda a que parezcan sprites de vida en la fila 3 del mapa
        if self.it == 0 { //Cuando el iterador de tiempo es igual a 0
            for _i in 0..3 { //Hara 3 intentos de poner la vida sobre una plataforma
                let rand:u8 = js! { return Math.floor(Math.random() * (@{self.width})); } //Posició random de x
                    .try_into()
                    .unwrap();
                if self.map.map[4][rand as usize] == 2 && self.map.map[3][rand as usize] == 0 { //Si no hay nada en la posición
                    self.map.map[3][rand as usize] = 4; //Pone la vida (sprite 4)
                    break; //Sale del for de intentos
                }
            }
        }

        //Este if ayuda a que aprezcan peces en posiciones aleatorias
        if self.it % 32 == 0 { //Cuando el iterador de tiempo % 32 es igual a 0
            for _i in 0..16 { //Hara 16 intentos de poner el pez sobre una plataforma
                let cx:u8 = js! { return Math.floor(Math.random() * (@{self.width})); } //Posición aleatoria de x
                    .try_into()
                    .unwrap();
                let cy:u8 = js! { return Math.floor(Math.random() * (@{self.height} - 6) + 3); } //Posición aleatoria de y
                    .try_into()
                    .unwrap();
                if self.map.map[1 + cy as usize][cx as usize] == 2 && self.map.map[cy as usize][cx as usize] == 0 { //Si no hay nada en esa posisicón
                    self.map.map[cy as usize][cx as usize] = 5; //Agrega el pez al mapa
                    break; // Sale del for de intentos
                }
            }
        }

        self.it = if self.it < self.max_it { self.it + 1 } else { 0 }; //Aumenta el iterador de tiempo, si llega al maximo se reinicia
    }

    //Función que dibuja el mapa y los sprites en el canvas
    pub fn draw(&self, canvas: &Canvas) {
        if self.game_state == GameState::Start { //Si el estado del juego es Start
            canvas.draw_map(&self.map_start); //Dibuja el mapa de Start
            return //Regresa
        }

        if self.game_state == GameState::GameOver {  //Si el estado del juego es GameOver
            canvas.draw_map(&self.map_end); //Dibuja el mapa de GameOver
            return//Regresa
        }

        //Si no
        canvas.draw_map(&self.map);  //Dibuja el mapa del juego
        //Dibuja el sprite del personaje en su posición y estado, el siguiete en la lista de sprites
        canvas.draw_sprite(self.coord.0, self.coord.1, &self.sprites[self.stt_ix][self.it % self.sprites[self.stt_ix].len()]);
    }

    //Funcion que invierte el sprite cuando cambia de dirección
    pub fn invert_side(&mut self) {
        for spte_list in self.sprites.iter_mut() { //Toma los sprites del personajes en un iterable
            for spte in spte_list { //Recore los sprites
                spte.map.invert_side(); //Invierte el estado
            }
        }
    }

    //Función que revisa si hay piso o no en la posición del personaje
    pub fn no_floor(&mut self) -> bool {
        self.map.map[self.coord.1 as usize + 1][self.coord.0 as usize] != 2 //Si es distinto de piso (valor 2) regresa true
    }

    //Función que cambia el estado del personaje
    pub fn set_state(&mut self, state: State) {
        self.last_state = self.state; //Actuliza el estado anterior
        self.state = state; //Actualiza el estado actual
        self.stt_ix = match state { //Cambia el indice de estados segun el estado que que se encuentra
            State::Stand => 0,
            State::Walk(_d) => 1,
            State::Jump(_d) => 2,
            State::Fall(_d) => 2,
            State::Crouch => 3,
        };
    }

    //Funcionque imprime en la parte superior de la pantalla las vidas y el puntaje
    pub fn load_hud(&mut self) {
        for i in 0..20 { //Limpia la primera fila del mapa
            self.map.map[0][i as usize] = 0; //La cmabia a cero
        }
        for i in 0..self.lifes { //Segun la cantidad de vidas
            self.map.map[0][i as usize] = 4; //Cambia en el mapa a 4 (codigo del sprite de vidas)
        }

        let mut cp_score = self.score; //Copia el puntaje a otra variable
        let mut scre = [0 as u8; 4]; //Crea un vector de 4 ceros

        for i in 0..4 { //Recorre eso vector
            scre[i as usize] = (cp_score % 10) as u8; //Calcula el valor de unidades, decenas, ...
            cp_score /= 10; //Divide el score entre 10 para calcula el siguienre

            //Se revisan los valores dados a los sprites para determinar que valor corresponde a que numero (0-9)
            if scre[i as usize] <= 3 { //En este caso si el valor es menor o igual a 3, solo s ele suman 6
                scre[i as usize] += 6;
            } else { //Si no
                scre[i as usize] = (('A' as u8) - ('A' as u8) + (6 as u8) + scre[i as usize]) as u8; //Se calcula que valor corresponde (ASCII)
            }

            self.map.map[0][19 - i as usize] = scre[i as usize]; //Se cambia los valores del mapa del juego para que aparezca
            self.map_end.map[0][19 - i as usize] = scre[i as usize] + 18; //Los códigos de los sprites difieren en 18, por lo que se le suman
        }
    }
}

//Funciones que cambian de estado al personaje segun la tecla presionada, son llamadas desde main
impl Character {
    pub fn arrow_left_down(&mut self) { //Si se presiona la tecla a la izquierda
        if let State::Stand = self.state { //Si esta en estado Stand
            self.set_state(State::Walk(Direction::Left)); //Pasa a Walk con dirección a la izquierda
        } else if let State::Jump(_d) = self.state { //Si esta en estado Jump con alguna dirección
            self.set_state(State::Jump(Direction::Left)); //Pasa a Jump con direccion a la izquierda
        } else if let State::Fall(_d) = self.state { //Si esta en estado Fall con alguna dirección
            self.set_state(State::Fall(Direction::Left)); //Pasa a Fall con direccion a la izquierda
        } else if let State::Walk(_d) = self.state { //Si esta en estado Walk con alguna dirección
            self.set_state(State::Walk(Direction::Left)); //Pasa a Walk con direccion a la izquierda
        }
    }

    pub fn arrow_right_down(&mut self) { //Si se presiona la tecla a la derecha
        if self.state == State::Stand { //Si esta en estado Stand
            self.set_state(State::Walk(Direction::Right)); //Pasa a Walk con dirección a la derecha
        } else if let State::Jump(_d) = self.state { //Si esta en estado Jump con alguna dirección
            self.set_state(State::Jump(Direction::Right)); //Pasa a Jump con direccion a la derecha
        } else if let State::Fall(_d) = self.state { //Si esta en estado Fall con alguna dirección
            self.set_state(State::Fall(Direction::Right)); //Pasa a Fall con direccion a la derecha
        } else if let State::Walk(_d) = self.state { //Si esta en estado Walk con alguna dirección
            self.set_state(State::Walk(Direction::Right)); //Pasa a Walk con direccion a la derecha
        }
    }

    pub fn arrow_down_down(&mut self) { //Si se presiona la tecla hacia abajo
        self.down_key = true; //Cambia el estado down_key a true
        if self.state == State::Stand { //Si esta en estado Stadn
            self.set_state(State::Crouch); //Cambia a estado Crouch
        }
    }

    pub fn arrow_up_down(&mut self) { //Si se presiona la tecla hacia arriba
        if self.state == State::Stand {
            js! { playJump(); }
            self.set_state(State::Jump(Direction::None));
        } else if let State::Walk(dir) = self.state {
            js! { playJump(); }
            self.set_state(State::Jump( dir ));
        } else if self.state == State::Crouch {
            js! { addCode("goDown"); }
            self.coord.1 += 1;
            self.set_state(State::Fall(Direction::None));
            js! { playDown(); }
        }
    }

    pub fn arrow_left_up(&mut self) { //Si se suelta la tecla hacia la izquierda
        if let State::Walk(dir) = self.state { //Si esta con estado Walk con alguna dirección
            if dir == Direction::Left { //Y la dirección era a la izquierda
                self.set_state(State::Stand); //Pasa a estado Stand
            }
        } else if let State::Jump(dir) = self.state { //Si esta con estado Jump con alguna dirección
            if dir == Direction::Left { //Y la dirección era a la izquierda
                self.set_state(State::Jump(Direction::None)); //Pasa a estado Jump sin dirección
            }
        } else if let State::Fall(dir) = self.state { //Si esta con estado Fall con alguna dirección
            if dir == Direction::Left { //Y la dirección era a la izquierda
                self.set_state(State::Fall(Direction::None)); //Pasa a estado Fall sin dirección
            }
        }
    }

    pub fn arrow_right_up(&mut self) { //Si se suelta la tecla hacia la derecha
        if let State::Walk(dir) = self.state { //Si esta con estado Walk con alguna dirección
            if dir == Direction::Right { //Y la dirección era a la derecha
                self.set_state(State::Stand); //Pasa a estado Stand
            }
        } else if let State::Jump(dir) = self.state { //Si esta con estado Jump con alguna dirección
            if dir == Direction::Right { //Y la dirección era a la derecha
                self.set_state(State::Jump(Direction::None)); //Pasa a estado Jump sin dirección
            }
        } else if let State::Fall(dir) = self.state { //Si esta con estado Fall con alguna dirección
            if dir == Direction::Right { //Y la dirección era a la derecha
                self.set_state(State::Fall(Direction::None)); //Pasa a estado Fall sin dirección
            }
        }
    }

    pub fn arrow_down_up(&mut self) { //Si se suelta la tecla hacia abajo
        self.down_key = false; //Cambia el estado down_key a false
        if self.state == State::Crouch { //Si esta en estado Crouch
            self.set_state(State::Stand); //Pasa a estado Stand
        }
    }

    //Función de start, se activa con la tecla espacio
    pub fn start(&mut self) {
        //Si esta en estado Start, cambia a Play e inicia el juego
        if self.game_state == GameState::Start {
            js! { playMusic(); } //Inicia musica de fondo
            js! { addCode("start"); }

            self.game_state = GameState::Play; //Cambia estado del juego
        }

        //En estado GameOver se reinician los valores iniciales del juego
        if self.game_state == GameState::GameOver {
            js! { addCode("reload"); }

            if self.dir == Direction::Left { self.invert_side(); }
            
            self.game_state = GameState::Start;

            //El mapa de juego se vuelve a cargar
            self.map = GameMap::from_text(String::from(include_str!("files/map.txt")));

            self.coord = Coord(9, 11);

            self.state = State::Stand;
            self.last_state = State::Stand;
            self.stt_ix = 0;

            self.dir = Direction::Right;
            self.last_dir = Direction::Right;

            self.jump_h = 0;
            self.speed = 128;
            self.lifes = 2;
            self.score = 0;

            self.down_key = false;

            self.p_dir = "".to_string();
            self.p_state = "".to_string();
            self.lt_p_dir = "".to_string();
            self.lt_p_state = "".to_string();
        }
    }
}
