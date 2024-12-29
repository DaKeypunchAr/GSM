use phonenumber::PhoneNumber;
use rusqlite::{Connection, Error, Result};
use std::fmt::{self, Debug, Display};
use std::fs;
use std::path::Path;

#[derive(PartialEq, Debug)]
pub struct Dealer {
    name: String,
    phone_num: PhoneNumber,
}

impl Display for Dealer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Name: {}, Phone Number: {}", self.name, self.phone_num)?;
        Ok(())
    }
}

pub struct Store {
    connection: Connection,
}

impl Store {
    fn build(path: &Path) -> Result<Store, Error> {
        let connection = Connection::open(path)?;

        connection.execute(
            "CREATE TABLE IF NOT EXISTS dealers (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                phone_num TEXT NOT NULL
            )",
            (),
        )?;

        Ok(Store { connection })
    }

    pub fn get_dealers(&self) -> Result<Vec<Dealer>, Error> {
        self.connection
            .prepare("SELECT (name, phone_num) FROM dealers")
            .unwrap()
            .query_map((), |row| {
                let name = row.get(0).unwrap();
                let phone_num: String = row.get(1).unwrap();
                let phone_num = match phonenumber::parse(None, phone_num) {
                    Ok(p) => p,
                    Err(_) => unreachable!(),
                };
                Ok(Dealer { name, phone_num })
            })
            .unwrap()
            .collect()
    }

    pub fn add_dealer(&mut self, name: &str, phone_num: &PhoneNumber) {
        if self
            .connection
            .execute(
                "INSERT INTO dealers (name, phone_num) VALUES(?1, ?2)",
                (name, phone_num.to_string()),
            )
            .is_err()
        {
            panic!("Failed to insert into dealers!");
        }
    }
}

pub fn load_store_data_from(path: &Path) -> Result<Store> {
    if fs::exists(path).expect("Unable to check if {path} exist!") {
        return Store::build(path);
    }

    if fs::exists(path.parent().expect("Failed to know the parent of a path"))
        .expect("Unable to check parent of path!")
    {
        return Store::build(path);
    }

    if let Err(e) = fs::create_dir_all(path.parent().expect("Unable to know the parent of {path}!"))
    {
        let e = e.kind();
        panic!(
            "Unable to create file at {}! Error: {}!",
            path.to_str().unwrap(),
            e
        );
    }

    Store::build(path)
}

#[cfg(test)]
mod tests {}
