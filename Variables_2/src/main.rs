fn main() {
    let mut x = 3;
    println!("El valor de x es: {}", x);

    x = 7;

    println!("El valor de x es: {}", x);

    const PI: f32 = 3.1416;

    println!("El valor de PI es: {}", PI);
    
    let y = 3;
    let y = "a";

    println!("Por shadowing el valor de y es: {}", y)
}
