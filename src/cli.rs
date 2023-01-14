use std::io::{self, Write};

use crate::fs;
use crate::transaction::Transaction;
use crate::{config::Config, date::date};
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Adds new entry
    Add {
        /// must be a string
        #[arg(short = 'm', long, default_value_t = String::from(""))]
        description: String,

        /// must be a string in format: "YYYY-MM-DD" or customized config format
        #[arg(short, long, default_value_t = date::today_date())]
        date: String,

        /// must be key value pairs: assets:checking $5000 [revenues:salary $-5000]
        postings: Vec<String>,
    },
}

pub fn act() {
    let args = Args::parse();

    match args.command {
        Some(Commands::Add {
            description,
            date,
            postings,
        }) => {
            let transaction =
                Transaction::add(description, date, postings).expect("Failed to create entry!");
            let transaction_string = transaction.format_string();
            println!(
                "This is the transaction to be commited:\n\n{}",
                transaction_string
            );
            let confirmation = get_confirmation("Is that ok?", true);
            if confirmation {
                let file_path = Config::read().journal_file_path;
                fs::write_to_file(file_path, transaction_string);
                println!("Wrote transaction successfully.");
            }
        }
        None => {}
    }
}

pub fn get_confirmation(message: &str, default: bool) -> bool {
    let options = if default { "([y]/n)" } else { "(y/[n])" };
    print!("{} {} ", message, options);
    io::stdout().flush().unwrap();

    let mut answer = String::new();
    let _ = io::stdin().read_line(&mut answer);
    let answer = answer.trim();

    if answer == "y" {
        return true;
    }

    if answer == "n" {
        return false;
    }

    default
}
