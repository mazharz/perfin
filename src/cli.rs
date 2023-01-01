use crate::entry::Entry;
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
        #[arg(short, long)]
        date: String,

        /// debit or credit amount (+/-) prefix determines which
        amount: String,
    },
}

pub fn act() {
    let args = Args::parse();

    match args.command {
        Some(Commands::Add {
            description,
            date,
            amount,
        }) => {
            let entry = Entry::add(description, date, amount).expect("Failed to create entry!");
            dbg!(entry);
        }
        None => {}
    }
}
