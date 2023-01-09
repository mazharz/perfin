use crate::date::date;
use crate::transaction::Transaction;
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
            let entry =
                Transaction::add(description, date, postings).expect("Failed to create entry!");
            dbg!(entry);
        }
        None => {}
    }
}
