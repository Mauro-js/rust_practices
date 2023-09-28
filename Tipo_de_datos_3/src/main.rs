fn main() {
    let mut x:u32 = 5;

    println!("The value of x is {}", x);

    x = 3999999;

    println!("The value of x is {}", x);

    let suma = 58 + 67;
    println!("la suma es: {}", suma);

    let resta = 34343.39 - 788.3;
    println!("la resta es: {}", resta);

    let multiplicacion = 4455.3 * 23.0;
    println!("la multiplicacion es: {}", multiplicacion);

    let division = 34 / 4;
    println!("la division es: {}", division);

    let resto = 34 % 4;
    println!("el resto es: {}", resto);

    let verdadero = true;
    println!("el booleano es: {}", verdadero);

    let falso: bool = false;
    println!("Bool con tipo de dato: {}", falso);

    let a:&str = "a";
    println!("a es: {}", a);

    let tupla:(f32,u8,f32) = (500.32, 25, -56.21);

    let (y,z,l) = tupla;
    println!("y: {}, l: {}", y, l);

    let primero = tupla.0;

    println!("primero es : {}", primero);

    println!("el segundo valor de la tupla es: {}", z);


    let matriz:[i32;6] = [25,25,23,14,23,89];

    let b = matriz[0];

    println!("b es: {}", b);

    let seugnda_matriz = [3;5];
    //let matriz = [3,3,3,3,3];
}
