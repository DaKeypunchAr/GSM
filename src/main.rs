use std::io::{self, Write};
use std::path::Path;

pub mod app_commands;
use app_commands::Command;

pub mod app_actions;
use app_actions as act;

use general_store_manager as gsm;

fn main() {
    println!("Welcome to our general store manager!");
    act::print_command_list();
    let mut store =
        gsm::load_store_data_from(Path::new("store.db")).expect("Failed to load store!");

    loop {
        let input_str = read_line("> ");

        if let Some(cmd) = Command::build(&input_str) {
            match cmd {
                Command::Help => act::print_command_list(),
                Command::ShowDealers => act::print_dealers(&store),
                Command::AddDealer => act::add_dealer_after_input(&mut store),
                Command::ShowGoods => todo!(),
                Command::AddGood => todo!(),
                Command::Exit => break,
            }
        } else {
            continue;
        }
    }
    println!("Thanks for using!");
}

fn read_line(question: &str) -> String {
    print!("{question}");
    io::stdout().flush().expect("Failed to flush stdout!");

    let mut input_str = String::new();
    io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line!");

    input_str
}

pub fn ask_input<F>(question: &str, evaluate_char: F) -> String
where
    F: Fn(char, &mut String) -> bool,
{
    loop {
        let input = read_line(question);

        if input.is_empty() {
            continue;
        }

        let input = input.trim();
        let mut output_text = String::new();

        for ch in input.chars() {
            if !evaluate_char(ch, &mut output_text) {
                continue;
            }
        }
        return output_text;
    }
}
