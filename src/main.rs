use std::io;

fn main() {

    let mut numero = String::new();
    let mut operacion = String::new();

    println!("Indicame dos numeros separados por un espacio: ");

    io::stdin().read_line(&mut numero)
        .ok()
        .expect("Error");

    let mut numero = numero.split_whitespace();

    let number1: i32 = numero.next().unwrap().parse().unwrap();
    let number2: i32 = numero.next().unwrap().parse().unwrap();

    println!("Cual operacion deseas realizar:\n\
        suma(1)\n\
        resta(2)\n\
        multiplicacion(3)\n\
        division(4)");

    io::stdin().read_line(&mut operacion)
        .ok()
        .expect("Error");

    let operacion: i32 = operacion.trim().parse().unwrap();

    match operacion {
        1 => println!("La suma es: {}", number1 + number2),
        2 => println!("La resta es: {}", number1 - number2),
        3 => println!("La multiplicacion es: {}", number1 * number2),
        4 => println!("La division es: {}", number1 / number2),
        _ => println!("La opcion no es valida")
    }
}