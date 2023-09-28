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

    let condicion = true;

    let tercer_numero = if condicion { 5 } else { 13 };

    println!("El valor de numero es {}", tercer_numero );

    // para un loop infinito se utiliza loop{}
    // y para salir del mismo se utiliza el comando break
    
    let mut contador = 0;
    loop {
        println!("contrador: {}", contador);
        contador += 1;

        if contador == 10 {
            break;
        }
    }

    // también loop puede devolver un valor, ejemplo:

    let mut contador2 = 0;
    let valor = loop {
        contador2 += 1;

        if contador2 == 10 {
            break contador * contador
        }
    };

    println!("El valor del loop es: {}", valor);

    //while se utiliza para hacer un loop en tanto una condición se cumpla

    let matriz = [10,20,30,40,50,60,70,80,90];
    let mut i = 0;

    while i < 9 {
        println!("Valor del array en la posición {}: {}", i+1, matriz[i]);

        i += 1;
    }

    //la forma más eficiente de recorrer un array es con un for iter

    for numero in matriz.iter() {
        println!("Valor del elemento es: {}", numero);
    }

    for numero in 1..=10 {
        println!("Valor del elemento es: {}", numero);
    }

    for numero in (1..=10).rev() {
        println!("Valor del elemento es: {}", numero);
    }
}//
