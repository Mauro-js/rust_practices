fn main() {
    let numero = 7;

    if numero < 10 {
        println!("Se cumplió la condición");
    } else {
        println!("No se cumplió la condición");
    }

    let cierto = true;

    if cierto {
        println!("Si cierto");
    } else {
        println!("No cierto");
    }

    let segundo_numero = 7;

    if segundo_numero < 5 {
        println!("El numero es menor a 5");
    } else if segundo_numero < 10 {
        println!("El numero es menor a 10");
    } else {
        println!("El numero es igual o mayor que 10");
    }
}
