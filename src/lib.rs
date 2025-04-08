use levenshtein::levenshtein;
use rusqlite::{params, Connection, Error, Result};
use std::fs;
use std::path::Path;

// TODO: Use the database to the full capacity!

#[derive(Debug)]
pub struct Dealer {
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    pub country_code: String,
    pub phone_num: String,
}

#[derive(Debug)]
pub struct Product {
    pub brand_name: String,
    pub product_name: String,
    pub item_name: String,
    pub pack_name: String,
}

pub struct Store {
    connection: Connection,
}

impl Store {
    pub fn build(path: &Path, new: bool) -> Result<Self> {
        let connection = Connection::open(path)?;

        if new {
            connection.execute_batch(
                "
                PRAGMA foreign_keys = ON;

                CREATE TABLE category (
                    category_id INTEGER PRIMARY KEY,
                    name TEXT NOT NULL CHECK (LENGTH(name) > 0),
                    parent_id INT,
                    FOREIGN KEY (parent_id) REFERENCES category(category_id) ON DELETE SET NULL
                );

                CREATE TABLE item (
                    item_id INTEGER PRIMARY KEY,
                    name TEXT NOT NULL CHECK (LENGTH(name) > 0),
                    category_id INT DEFAULT NULL,
                    FOREIGN KEY (category_id) REFERENCES category(category_id) ON DELETE SET NULL
                );

                CREATE TABLE brand (
                    brand_id INTEGER PRIMARY KEY,
                    name TEXT UNIQUE NOT NULL CHECK (LENGTH(name) > 0)
                );

                CREATE TABLE product (
                    product_id INTEGER PRIMARY KEY,
                    name TEXT NOT NULL CHECK (LENGTH(name) > 0),
                    pack_name TEXT NOT NULL CHECK (LENGTH(pack_name) > 0),
                    item_id INT NOT NULL,
                    brand_id INT NOT NULL,
                    FOREIGN KEY (item_id) REFERENCES item(item_id) ON DELETE CASCADE,
                    FOREIGN KEY (brand_id) REFERENCES brand(brand_id) ON DELETE CASCADE
                );

                CREATE TABLE phone (
                    phone_id INTEGER PRIMARY KEY,
                    country_code TEXT NOT NULL CHECK (LENGTH(country_code) > 0),
                    phone_number TEXT NOT NULL CHECK (LENGTH(phone_number) > 0)
                );

                CREATE TABLE address (
                    address_id INTEGER PRIMARY KEY,
                    house_num TEXT,  -- Optional
                    street_name TEXT, -- Optional
                    locality_name TEXT, -- Optional
                    city_name TEXT NOT NULL CHECK (LENGTH(city_name) > 0),
                    district_name TEXT NOT NULL CHECK (LENGTH(district_name) > 0),
                    pin_code TEXT NOT NULL CHECK (LENGTH(pin_code) > 0),
                    state TEXT NOT NULL CHECK (LENGTH(state) > 0),
                    country TEXT NOT NULL CHECK (LENGTH(country) > 0)
                );

                CREATE TABLE dealer (
                    dealer_id INTEGER PRIMARY KEY,
                    first_name TEXT NOT NULL CHECK (LENGTH(first_name) > 0),
                    middle_name TEXT,
                    last_name TEXT NOT NULL CHECK (LENGTH(last_name) > 0)
                );

                CREATE TABLE dealer_location (
                    dealer_id INT NOT NULL,
                    address_id INT NOT NULL,
                    description TEXT,
                    PRIMARY KEY (dealer_id, address_id),
                    FOREIGN KEY (dealer_id) REFERENCES dealer(dealer_id) ON DELETE CASCADE,
                    FOREIGN KEY (address_id) REFERENCES address(address_id) ON DELETE CASCADE
                );

                CREATE TABLE dealer_contact (
                    dealer_id INT NOT NULL,
                    phone_id INT NOT NULL,
                    description TEXT,
                    PRIMARY KEY (dealer_id, phone_id),
                    FOREIGN KEY (dealer_id) REFERENCES dealer(dealer_id) ON DELETE CASCADE,
                    FOREIGN KEY (phone_id) REFERENCES phone(phone_id) ON DELETE CASCADE
                );

                CREATE TABLE dealer_price (
                    product_id INT NOT NULL,
                    dealer_id INT NOT NULL,
                    price NUMERIC NOT NULL CHECK (price >= 0),
                    time_stamp DATETIME DEFAULT CURRENT_TIMESTAMP,
                    PRIMARY KEY (product_id, dealer_id, time_stamp),
                    FOREIGN KEY (product_id) REFERENCES product(product_id) ON DELETE CASCADE,
                    FOREIGN KEY (dealer_id) REFERENCES dealer(dealer_id) ON DELETE CASCADE
                );
                ",
            )?;
        }

        Ok(Self { connection })
    }

    pub fn get_dealers(&self) -> Result<Vec<Dealer>, Error> {
        self.connection
            .prepare(
                "
                    SELECT d.first_name, d.middle_name, d.last_name, p.country_code, p.phone_number
                    FROM dealer d
                    LEFT JOIN dealer_contact dc ON d.dealer_id = dc.dealer_id
                    LEFT JOIN phone p ON dc.phone_id = p.phone_id
                    ",
            )?
            .query_map((), |row| {
                let first_name = row.get(0)?;
                let middle_name = row.get(1)?;
                let last_name = row.get(2)?;
                let country_code = row.get(3)?;
                let phone_num = row.get(4)?;
                Ok(Dealer {
                    first_name,
                    middle_name,
                    last_name,
                    country_code,
                    phone_num,
                })
            })?
            .collect()
    }

    pub fn add_dealer(
        &mut self,
        first_name: &str,
        middle_name: Option<&str>,
        last_name: &str,
        country_code: &str,
        phone_number: &str,
    ) -> Result<(), Error> {
        let transaction = self.connection.transaction()?;

        transaction.execute(
            "INSERT INTO dealer (first_name, middle_name, last_name) VALUES(?1, ?2, ?3)",
            params![first_name, middle_name, last_name],
        )?;
        let dealer_id = transaction.last_insert_rowid();

        transaction.execute(
            "INSERT OR IGNORE INTO phone (country_code, phone_number) VALUES(?1, ?2)",
            params![country_code, phone_number],
        )?;

        let phone_id: i64 = transaction.query_row(
            "SELECT phone_id FROM phone WHERE country_code = ?1 AND phone_number = ?2",
            params![country_code, phone_number],
            |row| row.get(0),
        )?;

        transaction.execute(
            "INSERT INTO dealer_contact (dealer_id, phone_id) VALUES (?1, ?2)",
            (dealer_id, phone_id),
        )?;

        transaction.commit()?;
        Ok(())
    }

    pub fn get_products(&self) -> Result<Vec<Product>, Error> {
        self.connection
            .prepare(
                "
                    SELECT product.name, product.pack_name, item.name, brand.name
                    FROM product
                    LEFT JOIN item ON item.item_id = product.item_id
                    LEFT JOIN brand ON brand.brand_id = product.brand_id
                    ",
            )?
            .query_map((), |row| {
                let product_name = row.get(0)?;
                let pack_name = row.get(1)?;
                let item_name = row.get(2)?;
                let brand_name = row.get(3)?;
                Ok(Product {
                    product_name,
                    brand_name,
                    item_name,
                    pack_name,
                })
            })?
            .collect()
    }

    pub fn add_product(
        &mut self,
        product_name: &str,
        brand_name: &str,
        item_name: &str,
        pack_name: &str,
    ) -> Result<(), Error> {
        let transaction = self.connection.transaction()?;

        transaction.execute(
            "INSERT OR IGNORE INTO item (name) VALUES(?1)",
            params![item_name],
        )?;

        let item_id: i64 = transaction.query_row(
            "SELECT item_id FROM item WHERE name = ?1",
            params![item_name],
            |row| row.get(0),
        )?;

        transaction.execute(
            "INSERT OR IGNORE INTO brand (name) VALUES(?1)",
            params![brand_name],
        )?;
        let brand_id: i64 = transaction.query_row(
            "SELECT brand_id FROM brand WHERE name = ?1",
            params![brand_name],
            |row| row.get(0),
        )?;

        transaction.execute(
            "INSERT INTO product (name, pack_name, brand_id, item_id) VALUES(?1, ?2, ?3, ?4)",
            params![product_name, pack_name, brand_id, item_id],
        )?;

        transaction.commit()?;
        Ok(())
    }

    pub fn get_best_product_results_for(&self, new_text: &str) -> Result<Vec<Product>, Error> {
        if new_text.is_empty() {
            return Ok(Vec::new());
        }

        let products = self.get_products()?;
        let query = new_text.to_lowercase();

        let mut results: Vec<(Product, usize)> = products
            .into_iter()
            .map(|product| {
                let mut score = 0;

                let product_name = product.product_name.to_lowercase();
                let brand_name = product.brand_name.to_lowercase();
                let item_name = product.item_name.to_lowercase();
                let pack_name = product.pack_name.to_lowercase();

                if product_name == query
                    || brand_name == query
                    || item_name == query
                    || pack_name == query
                {
                    score += 1000;
                }

                if product_name.starts_with(&query)
                    || brand_name.starts_with(&query)
                    || item_name.starts_with(&query)
                    || pack_name.starts_with(&query)
                {
                    score += 500;
                }

                if product_name.contains(&query)
                    || brand_name.contains(&query)
                    || item_name.contains(&query)
                    || pack_name.contains(&query)
                {
                    score += 200;
                }

                let distances = [
                    levenshtein(&product_name, &query),
                    levenshtein(&brand_name, &query),
                    levenshtein(&item_name, &query),
                    levenshtein(&pack_name, &query),
                ];
                let min_distance = *distances.iter().min().unwrap_or(&usize::MAX);

                if min_distance <= 2 {
                    score += 100 - min_distance * 30;
                }

                (product, score)
            })
            .filter(|(_, score)| *score > 0) // Remove low-relevance results
            .collect();

        results.sort_by(|a, b| b.1.cmp(&a.1));
        results.truncate(5);

        // Return only the products
        Ok(results.into_iter().map(|(product, _)| product).collect())
    }

    pub fn get_recent_product_results(&self) -> Result<Vec<Product>, Error> {
        // TODO: Implement a table in the database to account for recent searches!
        Ok(Vec::new())
    }

    pub fn get_latest_dealer_price_pairs_for(
        &self,
        product: Product,
    ) -> Result<Vec<(Dealer, u32)>, Error> {
        let item_id: i64 = self.connection.query_row(
            "SELECT item_id FROM item WHERE name = ?1",
            params![product.item_name],
            |row| row.get(0),
        )?;
        let brand_id: i64 = self.connection.query_row(
            "SELECT brand_id FROM brand WHERE name = ?1",
            params![product.brand_name],
            |row| row.get(0),
        )?;
        let product_id: i64 = self.connection.query_row(
            "SELECT product_id FROM product WHERE name = ?1 AND item_id = ?2 AND brand_id = ?3",
            params![product.product_name, item_id, brand_id],
            |row| row.get(0),
        )?;

        self.connection
            .prepare(
                "
        SELECT d.first_name, d.middle_name, d.last_name, p.country_code, p.phone_number, price
        FROM dealer_price dp
        LEFT JOIN dealer d ON d.dealer_id = dp.dealer_id
        LEFT JOIN dealer_contact dc ON d.dealer_id = dc.dealer_id
        LEFT JOIN phone p ON dc.phone_id = p.phone_id
        WHERE product_id = ?1
        AND dp.time_stamp = (
            SELECT MAX(dp2.time_stamp)
            FROM dealer_price dp2
            WHERE dp2.dealer_id = dp.dealer_id
            AND dp2.product_id = dp.product_id
        )
        ",
            )?
            .query_map(params![product_id], |row| {
                let first_name = row.get(0)?;
                let middle_name = row.get(1)?;
                let last_name = row.get(2)?;
                let country_code = row.get(3)?;
                let phone_num = row.get(4)?;
                Ok((
                    Dealer {
                        first_name,
                        middle_name,
                        last_name,
                        country_code,
                        phone_num,
                    },
                    row.get(5)?,
                ))
            })?
            .collect()
    }

    pub fn update_price(&self, product: Product, dealer: Dealer, price: u32) {
        println!(
            "Product: {:#?}\nDealer: {:#?}\nPrice: {:#?}",
            product, dealer, price
        );
        let product_id: i64 = self
            .connection
            .query_row(
                "
SELECT product_id
FROM product
LEFT JOIN brand ON brand.brand_id = product.brand_id
LEFT JOIN item ON item.item_id = product.item_id
WHERE product.name = ?1 AND brand.name = ?2 AND item.name = ?3 AND product.pack_name = ?4",
                params![
                    product.product_name,
                    product.brand_name,
                    product.item_name,
                    product.pack_name
                ],
                |row| row.get(0),
            )
            .unwrap();

        let dealer_id: i64 = self
            .connection
            .query_row(
            "
SELECT dealer.dealer_id
FROM dealer
LEFT JOIN dealer_contact ON dealer.dealer_id = dealer_contact.dealer_id
LEFT JOIN phone ON dealer_contact.phone_id = phone.phone_id
WHERE dealer.first_name = ?1 AND (dealer.middle_name IS NULL OR dealer.middle_name = ?2) AND dealer.last_name = ?3 AND phone.country_code = ?4 AND phone.phone_number = ?5",
        params![dealer.first_name, dealer.middle_name, dealer.last_name, dealer.country_code, dealer.phone_num], |row| row.get(0)).unwrap();

        self.connection
            .execute(
                "INSERT INTO dealer_price (product_id, dealer_id, price) VALUES (?1, ?2, ?3)",
                params![product_id, dealer_id, price],
            )
            .unwrap();
    }
}

pub fn load_store_data_from(path: &Path) -> Result<Store> {
    if fs::exists(path).expect("Unable to check if {path} exist!") {
        return Store::build(path, false);
    }

    if fs::exists(path.parent().expect("Failed to know the parent of a path"))
        .expect("Unable to check parent of path!")
    {
        return Store::build(path, true);
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

    Store::build(path, true)
}

#[cfg(test)]
mod tests {}
