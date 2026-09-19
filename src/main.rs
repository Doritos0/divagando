mod config;

fn main() {
    let _engine = config::db::get_engine();
    println!("Conexion a SQLite en rust :)");
    println!("Se realizará prueba de metodo sumar");
    let numero_1: i32 = 8;
    let numero_2: i32 = 92;
    let resultado: i32 = suma(numero_1, numero_2);
    println!("Al resultado de sumar estos numeros es {resultado}");
}

fn suma(numero_1: i32, numero_2: i32) -> i32{
    println!("Se entrega el siguiente numero 1: {numero_1}");
    println!("Se entrega el siguiente numero 2: {numero_2}, se sumaran");
    let total: i32 = numero_1 + numero_2;
    return total;
}
