use super::app_commands::Command;
use super::ask_input;
use general_store_manager::Store;

pub fn print_command_list() {
    println!("\nYou can choose one of the following commands: ");

    for cmd in Command::cmds() {
        println!("{}: {}", cmd.to_u8(), cmd.to_str());
    }
}

pub fn print_dealers(store: &Store) {
    let mut dealers = store.get_dealers().unwrap().into_iter().peekable();

    if dealers.peek().is_none() {
        println!("There are no dealers yet!");
        return;
    }

    println!("Printing dealers!");
    for (i, dealer) in dealers.enumerate() {
        println!("{}: {}", i + 1, dealer);
    }
}

pub fn add_dealer_after_input(store: &mut Store) {
    let name = ask_input(
        "Give a valid non-empty name (using alphabets and an _ for spaces) for the dealer!\n> ",
        |c, out| {
            if !c.is_alphabetic() && c != '.' {
                if c == ' ' {
                    out.push('_');
                    return true;
                }
                return false;
            }
            out.push(c);
            true
        },
    );

    loop {
        let phone_num = ask_input(
            "Give a valid country-code included phone number!\n> ",
            |c, out| {
                if !c.is_numeric() && c != '+' {
                    return false;
                }
                out.push(c);
                true
            },
        );

        let phone_num = match phonenumber::parse(None, phone_num) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("Error: {e}! Retry!");
                continue;
            }
        };

        store.add_dealer(&name, &phone_num);
        println!("Added the dealer successfully!");
        break;
    }
}
