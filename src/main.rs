pub mod cli;
pub mod config;
pub mod date;
pub mod posting;
pub mod random;
pub mod transaction;

use crate::cli::act;

fn main() {
    act();
}
