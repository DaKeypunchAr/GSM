use phonenumber::PhoneNumber;
use std::fmt::{Debug, Display, Result};
use std::fs::{self, File};
use std::path::Path;

pub struct Dealer {
    name: String,
    phone_num: PhoneNumber,
}

impl PartialEq for Dealer {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.phone_num == other.phone_num
    }
}

impl Display for Dealer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        write!(f, "Name: {}, Phone Number: {}", self.name, self.phone_num)?;
        Ok(())
    }
}

impl Debug for Dealer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        write!(f, "Name: {}, Phone Number: {}", self.name, self.phone_num)?;
        Ok(())
    }
}

pub struct Store {
    dealers: Vec<Dealer>,
}

impl Store {
    fn new() -> Store {
        Store {
            dealers: Vec::new(),
        }
    }
}

impl PartialEq for Store {
    fn eq(&self, other: &Self) -> bool {
        self.dealers == other.dealers
    }
}

impl Debug for Store {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        write!(f, "Dealers:\n{:#?}", self.dealers)?;
        Ok(())
    }
}

pub fn load_store_data_from(path: &Path) -> Option<Store> {
    if fs::exists(path).expect("Unable to check if {path} exist!") {
        return Some(load_store_data(
            &fs::read_to_string(path).unwrap_or_else(|x| x.to_string()),
        ));
    }

    if fs::exists(path.parent().expect("Unable to know the parent of {path}!"))
        .expect("Unable to check parent of {path}!")
    {
        if let Err(e) = File::create(path) {
            let e = e.kind();
            panic!(
                "Unable to create file at {}! Error: {}!",
                path.to_str().unwrap(),
                e
            );
        }
    } else if let Err(e) =
        fs::create_dir_all(path.parent().expect("Unable to know the parent of {path}!"))
    {
        let e = e.kind();
        panic!(
            "Unable to create file at {}! Error: {}!",
            path.to_str().unwrap(),
            e
        );
    } else if let Err(e) = File::create(path) {
        let e = e.kind();
        panic!(
            "Unable to create file at {}! Error: {}!",
            path.to_str().unwrap(),
            e
        );
    }

    Some(Store::new())
}

fn get_value_from<'a>(line: &'a str, literal: &str) -> &'a str {
    let get_header_end = || {
        for (start_idx, end_idx) in (literal.len()..line.len()).enumerate() {
            if line[start_idx..end_idx] == *literal {
                return Some(end_idx);
            }
        }
        None
    };

    let start = match get_header_end() {
        Some(i) => i,
        None => {
            panic!("Failed to get header end of header {literal} in line {line}!");
        }
    };
    let mut start_idx = None;
    let mut end_idx = None;

    for idx in start..line.len() {
        let ch = line.chars().nth(idx).unwrap();

        if start_idx.is_none() {
            if ch.is_whitespace() {
                continue;
            } else {
                start_idx = Some(idx);
            }
        }

        if end_idx.is_none() {
            if ch.is_alphanumeric() || ch == '+' {
                continue;
            } else {
                end_idx = Some(idx);
                break;
            }
        }
    }

    let start_idx = match start_idx {
        Some(idx) => idx,
        None => {
            panic!("Invalid start_idx of value of {literal} in line {line}!");
        }
    };

    let end_idx = match end_idx {
        Some(idx) => idx,
        None => line.len(),
    };

    &line[start_idx..end_idx]
}

fn get_name_from(line: &str) -> &str {
    const LITERAL_TO_COMPARE: &str = "[name]:";
    get_value_from(line, LITERAL_TO_COMPARE)
}

fn get_phone_num_from(line: &str) -> PhoneNumber {
    const LITERAL_TO_COMPARE: &str = "[phone-num]:";
    let phone_num = get_value_from(line, LITERAL_TO_COMPARE);
    match phonenumber::parse(None, phone_num) {
        Ok(result) => result,
        Err(e) => panic!("Error: {e} from parsing phone-num {phone_num} in line {line}!"),
    }
}

fn parse_dealer(line: &str) -> Dealer {
    Dealer {
        name: get_name_from(line).to_string(),
        phone_num: get_phone_num_from(line),
    }
}

fn load_store_data(contents: &str) -> Store {
    let mut store = Store {
        dealers: Vec::new(),
    };

    enum ParsingMode {
        None,
        Dealers,
    }

    let mut parsing_mode = ParsingMode::None;

    for line in contents.lines() {
        if line.is_empty() {
            continue;
        }

        if line.trim() == "--dealers--" {
            parsing_mode = ParsingMode::Dealers;
            continue;
        }
        match parsing_mode {
            ParsingMode::None => continue,
            ParsingMode::Dealers => {
                store.dealers.push(parse_dealer(line));
            }
        }
    }

    store
}

pub fn get_dealers(store: &Store) -> std::slice::Iter<'_, Dealer> {
    store.dealers.iter()
}

pub fn add_dealer(store: &mut Store, name: &str, phone_num: &PhoneNumber) {
    store.dealers.push(Dealer {
        name: name.to_string(),
        phone_num: phone_num.to_owned(),
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testing_dealer_parse() {
        let example_data = "[name]: duomg, [phone-num]: +14155552671";

        let dealer = parse_dealer(example_data);

        assert_eq!(
            dealer,
            Dealer {
                name: "duomg".to_string(),
                phone_num: phonenumber::parse(None, "+14155552671").unwrap()
            }
        )
    }

    #[test]
    fn testing_loaded_store_data() {
        let example_data = "\n--dealers--
[name]: duomg, [phone-num]: +14155552671";

        let store = load_store_data(example_data);

        let correct_phone_num = phonenumber::parse(None, "+14155552671").unwrap();

        let correct_and_only_dealer = Dealer {
            name: "duomg".to_string(),
            phone_num: correct_phone_num,
        };

        let correct_store = Store {
            dealers: vec![correct_and_only_dealer],
        };

        assert_eq!(store, correct_store);
    }
}
