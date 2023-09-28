fn main() {
    println!("Hello, world!");

    mi_funcion();

    valor_variable(7, 8);

    let x = 1;

    let j = {
        let k = x + 1;

        k + 1
    };

    println!(" El valor de j es: {}", j);

    let i = trece();

    println!("Elvalor de i es {}", i);
}

fn mi_funcion() {
    println!("Hola Mundo");
}

fn valor_variable(i: i32, j: i32) {
    println!("El valor del parámetro es: {}", i);
    println!("El valor del segundo parámetro es: {}", j)
}

fn trece() -> i8 {
    return 13;
}