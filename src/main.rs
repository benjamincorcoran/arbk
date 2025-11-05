use rusqlite::{self, Connection};

enum Storage {
    Memory,
    Path(String),
}

enum AppError {
    Sql(rusqlite::Error),
}
impl From<rusqlite::Error> for AppError {
    fn from(e: rusqlite::Error) -> Self {
        AppError::Sql(e)
    }
}

struct Config {
    storage: Storage,
}

struct Application {
    config: Config,
    conn: rusqlite::Connection,
}
impl Application {
    fn new(config: Config) -> Result<Self, AppError> {
        let conn = Self::get_conn(&config)?;
        let _ = Self::check_db(&conn);
        Ok(Self { config, conn })
    }

    fn get_conn(config: &Config) -> rusqlite::Result<rusqlite::Connection> {
        match &config.storage {
            Storage::Memory => rusqlite::Connection::open_in_memory(),
            Storage::Path(p) => rusqlite::Connection::open(p),
        }
    }

    fn check_db(conn: &Connection) -> Result<(), AppError> {
        println!("Checking status of database");
        if !conn.table_exists(Some("main"), "books")? {
            println!("Missing table books");
            Self::init_db(&conn)?;
        }
        Ok(())
    }

    fn init_db(conn: &Connection) -> Result<(), AppError> {
        println!("Initialising database.");
        conn.execute(
            "create table books (
                id integer primary key, 
                title text not null,
                author text not null 
            )",
            [],
        )?;

        println!("Database inited");
        Ok(())
    }
}

fn main() {
    let cfg = Config {
        // storage: Storage::Path("lib.db".to_string()),
        storage: Storage::Memory,
    };
    let app = Application::new(cfg);
}
