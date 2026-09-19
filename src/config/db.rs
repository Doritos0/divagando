use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use std::sync::{Mutex, OnceLock};

static DB_INSTANCE: OnceLock<Mutex<SqliteConnection>> = OnceLock::new();

pub fn get_engine() -> &'static Mutex<SqliteConnection> {
    DB_INSTANCE.get_or_init(|| {
        dotenv().ok();
        let db_name = env::var("BD_NAME").expect("BD_NAME debe estar configurada en .env");
        let conn = SqliteConnection::establish(&db_name)
            .unwrap_or_else(|e| panic!("Error al conectar con la base de datos {db_name}: {e}"));
        Mutex::new(conn)
    })
}