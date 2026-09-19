mod config;
use std::io::{self, Write};

fn main() {
    let _engine = config::db::get_engine();
    println!("Conexion a SQLite en rust :)");
    println!("Se realizará prueba de metodo sumar");
    let numero_1: i32 = 8;
    let numero_2: i32 = 92;
    let resultado: i32 = suma(numero_1, numero_2);
    println!("Al resultado de sumar estos numeros es {resultado}");
    menu();
}

fn suma(numero_1: i32, numero_2: i32) -> i32{
    println!("Se entrega el siguiente numero 1: {numero_1}");
    println!("Se entrega el siguiente numero 2: {numero_2}, se sumaran");
    let total: i32 = numero_1 + numero_2;
    return total;
}

fn menu(){
    let mut opcion: String = String::new();
    println!("Menu de opciones:");
    println!("1.- Ingresar nombre");
    println!("2.- Salir");
    io::stdout().flush().expect("Error al ingresas opcion");
    io::stdin().read_line(&mut opcion).expect("Error al leer");
    println!("Pasando string de la opcion a numero...");
    let num_opcion: i32 = opcion.trim().parse().expect("Debe ser un numero");

    if num_opcion == 1{
        ingresar_nombre();
    } else if num_opcion == 2 {
        println!("Adios :(");
    } else {
        println!("Error al ingresar opcion");
    }
}

fn ingresar_nombre() {
    let mut nombre: String = String::new();
    println!("Ingrese nombre: ");
    io::stdout().flush().expect("Error al mostrar mensaje");

    io::stdin().read_line(&mut nombre).expect("Error al leer");
    let nombre: &str = nombre.trim();

    print!("Ingresaste el siguiente nombre: {nombre}");
}