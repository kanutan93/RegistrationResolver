use diesel::{Connection, PgConnection};

pub struct DbConfig {
    pub conn: PgConnection
}

impl DbConfig {

    pub fn new(database_url: String) -> DbConfig {
        let conn = PgConnection::establish(&database_url)
            .expect("Can't establish connection to database");

        DbConfig {
            conn
        }
    }
}