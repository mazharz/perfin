pub mod cli;
pub mod entry;

use crate::cli::act;
use std::env;

fn main() {
    act(env::args());
}
