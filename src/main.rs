//// 2023/09/30 // 9:15 //

// Métodos

// Los métodos son similares a las funciones: se declaran con la palabra clave fn y
// su numbre; pueden tener parámetros y un valor de retorno, y contienen código
// que se ejecuta cuando se invoca.

// Sin embargo, los métodos son diferentes de las funciones en que se definen
// dentro del contexto de una estructura (o una enumeración o un objeto de rasgo),
// y su primer parámetro es siempre self, que representa el instancia de la
// estructura en la que se está llamando al método.

struct Rectangulo {
    ancho: u32,
    alto: u32,
}

impl Rectangulo {
    fn area(&self) -> u32 {
        self.ancho * self.alto
    }

    // fn puede_contener(&self, otro: &Rectangulo) -> bool {
    //     self.ancho > otro.ancho && self.alto > otro.ancho
    // }
}

impl Rectangulo {
    fn puede_contener(&self, otro: &Rectangulo) -> bool {
        self.ancho > otro.ancho && self.alto > otro.ancho
    }

    fn cuadrado(lado: u32) -> Rectangulo {
        Rectangulo {
            ancho: lado,
            alto: lado,
        }
    }
}

fn main() {

    let rectangulo1 = Rectangulo {
        ancho: 35,
        alto: 50,
    };

    let rectangulo2 = Rectangulo {
        ancho: 45,
        alto: 60,
    };

    let rectangulo3 = Rectangulo {
        ancho: 30,
        alto: 60,
    };

    println!("El área del rectángulo es: {}", rectangulo1.area());
    println!("¿Puede contener rectángulo2 a rectángulo1? {}", rectangulo2.puede_contener(&rectangulo1));
    println!("¿Puede contener rectángulo3 a rectángulo1? {}", rectangulo3.puede_contener(&rectangulo1));


    // Funciones asociadas

    // Podemos definir funciones dentro de los bloques impl que no toman como
    // parámetros a self. Estas se denominan funciones asociadas porque están
    // asociadas con las estructura. Sigues siendo funciones, no métodos, porque no
    // tienen una instancia de la estructura con la que trabajar.

    // En vídeos anteriores, usamos String::from, que es una función asociada.

    // Las funciones asociadas se utilizan a menudo para constructores que devolverán
    // una nueva isntancia de la estructura.


    let cuadrado1 = Rectangulo::cuadrado(7);
    println!("El alto del cuadrado1 es: {}", cuadrado1.alto);

}
