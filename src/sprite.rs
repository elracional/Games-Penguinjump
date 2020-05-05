//Declaración de librerias de la libreia estandar de Rust
use std::fmt;
use std::string::String;
use std::collections::HashMap;

//Declaración de modulo a utilizar
use crate::bitmap::BitMap;

//Estructura de los sprites
#[derive(Debug)]
pub struct Sprite {
    pub map: BitMap, //Bitmap
    pub colorset: [String; 7], //Colores
}

//Implemtanciín de Display
impl fmt::Display for Sprite {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.map.0.iter() {
            for n in line {
                write!(f, "{}, ", n)?;
            }
            writeln!(f, "")?;
        }
        writeln!(f, "")
    }
}

impl Sprite {
    //Nuevo sprite
    pub fn new(colors: Vec<String>, compressed_map: (u64, u64, u64)) -> Sprite {
        //Vector con los colores a utilizar
        let mut colorset = [String::new(),
                            String::new(),
                            String::new(),
                            String::new(),
                            String::new(),
                            String::new(),
                            String::new()
        ];
        for (i, color) in colors.iter().enumerate() { //Recorre los colores dados
            colorset[i].push_str(color); //Los agrega al vector de colores
        }
        Sprite { //Sprite (bitmap, colorset)
            map: BitMap::from_compress(compressed_map),
            colorset,
        }
    }

    //Función que regresa el nuevo sprite segun por nombre
    pub fn deref(&self) -> Sprite {
        let mut colors: Vec<String> = Vec::new(); //Toma los colores
        let compressed_map = self.map.compress_map(); //Crea el bitmap

        for color in self.colorset.iter() { //Agrega los colores al vector
            colors.push(color.to_string());
        }

        Sprite::new(colors, compressed_map) //Regresa un nuevo sprite
    }

    //Lee las lineas desde el archivo de sprites
    pub fn from_line(line: String) -> Sprite {
        let mut colors: Vec<String> = Vec::new(); //Crea el vector de los colores
        let mut compressed_map: [u64; 3] = [0; 3]; //Crea tres variables de 64 bits que son los sprites comprimidos

        let mut data = line.split(":"); //Separa la cadena por :

        data.next().unwrap(); //Salta el nombre del sprite para leer los colores
    {
        let clrs = data.next().unwrap();
        for color in clrs.split("-") { //Agrega los colores al vector, estan separados por -
            colors.push(String::from(color));
        }
    }
    {
        let compress = data.next().unwrap(); //Avanza a los valores u64
        for (i, cmp) in compress.split("-").enumerate() { //Separa las tres varibles separadas por -
            compressed_map[i] = String::from(cmp).parse::<u64>().unwrap(); //Los agrega al vector compressed_map
        }
    }

        Sprite::new(colors, (compressed_map[0], compressed_map[1], compressed_map[2]) ) //Regresa nuevo sprite
    }

    //Función que toma los sprites desde archivo de texto
    pub fn hash_from_text(text: String) -> HashMap<String, Sprite> {
        let mut hash = HashMap::new(); //Crea nuevo hashmap

        for line in text.lines() { //Lee las lineas del archivo
            let key = line.split(":").next().unwrap(); //Separa las lineas por :
            let spte = Sprite::from_line(String::from(line)); //Crea un nuevo sprite con los datos obtenidos

            hash.insert(String::from(key), spte); //Inserta el sprite al hashmap de sprites
        }
        hash //Retorna el hashmap
    }
}
