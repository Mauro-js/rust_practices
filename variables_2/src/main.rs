fn main() {
    let mut _x = 3;
    println!("El valor de x es: {}", _x);

    _x = 7;

    println!("El valor de x es: {}", _x);

    const PI: f32 = 3.1416;

    println!("El valor de PI es: {}", PI);
    
    let _y = 3;
    let _y = "a";

    println!("Por shadowing el valor de y es: {}", _y)
}
