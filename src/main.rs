mod config;

fn main() {
    let _engine = config::db::get_engine();
    println!("Conexion a SQLite en rust :)");
}
