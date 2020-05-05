//Librerias de stdweb
use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::html_element::CanvasElement;
use stdweb::web::{document, CanvasRenderingContext2d};

//Declaración de módulos importados
use crate::sprite::Sprite;
use crate::gamemap::GameMap;

//Declaracinó de estructura del canvas
pub struct Canvas {
    pub canvas: CanvasElement, //Canvas element
    pub ctx: CanvasRenderingContext2d, //Canvas con renderizado 2D
    scaled_width: u32, //Ancho del juego
    scaled_height: u32, //Altura del juego
    width: u32, //Ancho del canvas
    height: u32, //Altura del canvas
}

impl Canvas {
    //Creación de una nueva referencia al canvas formateado
    pub fn new(attr_id: &str, width: u32, height: u32) -> Canvas {
        let canvas: CanvasElement = document() //Crea una referencia al canvas
            .query_selector(attr_id)
            .unwrap()
            .unwrap()
            .try_into()
            .unwrap();

        let ctx: CanvasRenderingContext2d = canvas.get_context().unwrap(); //Renderizado a 2D

        //Tamaño de la pantalla de juego segun el tamaño del canvas
        let scaled_width = canvas.width() / width;
        let scaled_height = canvas.height() / height;

        Canvas {
            canvas,
            ctx,
            scaled_width,
            scaled_height,
            width,
            height,
        } //Retorna nuevo canvas stdweb
    }

    //Función que dibuja los sprites en el canvas
    pub fn draw_sprite(&self, x: u32, y: u32, spte: &Sprite) {
        //Cambia las coordenadas de tamaño del juego a px
        let x = x * self.scaled_width;
        let y = y * self.scaled_height;

        //Calcula el valor en pixeles de cada sprite
        let width = self.scaled_width as f64 / 8.0;
        let height = self.scaled_height as f64 / 8.0;

        for (i, row) in spte.map.0.iter().enumerate() { //Recorre las filas del sprite
            for (j, pixel) in row.iter().enumerate() { //Recorre las columaas en la fila
                if *pixel == 0 { //Si el valor del pixel es 0
                    continue; //Salta al siguiente
                }
                self.ctx.set_fill_style_color(&spte.colorset[(*pixel-1) as usize]); //Cambia el color del pixel
                self.ctx.fill_rect( //Dibuja el pixel del sprite
                    x as f64 + j as f64 * width,
                    y as f64 + i as f64 * width,
                    width,
                    height
                );
            }
        }
    }

    //Función que dibuja el mapa en el canvas
    pub fn draw_map(&self, map: &GameMap) {
        self.clear_all( map.bg.as_ref() ); //Limpia la patanlla con el color de fondo
        for (i, row) in map.map.iter().enumerate() { //Recorre las filas
            for (j, s) in row.iter().enumerate() { //Recorre las columnas
                if *s == 0 { //Si el valor del sprite es 0
                    continue; //Salta al siguiente
                }

                //Guarda las coordenadas en el canvas
                let x = j as u32;
                let y = i as u32;
                self.draw_sprite(x, y, &map.sprites[(*s-1) as usize]); //Dibuja el sprite correspodiente
            }
        }
    }

    //Función que limpia todo la pantalla
    pub fn clear_all(&self, bg: &str) {
        self.ctx.set_fill_style_color(bg); //Cambia el color
        self.ctx.fill_rect( //Crea un rectangulo del tamaño de la pantalla
            0.0,
            0.0,
            (self.width * self.scaled_width) as f64,
            (self.height * self.scaled_height) as f64,
        );
    }
}
