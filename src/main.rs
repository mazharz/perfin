pub mod account;
pub mod cli;
pub mod config;
pub mod date;
pub mod env;
pub mod fs;
pub mod posting;
pub mod random;
pub mod transaction;

use crate::cli::act;

fn main() {
    act();
}
