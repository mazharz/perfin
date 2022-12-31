use std::env::Args;

use crate::entry::Entry;

// actions:
// - add

// options for add:
// -d STRING: description
// -date YYYY-MM-DD: date

pub fn act(mut args: Args) {
    args.next();

    // TODO: add CLIAction
    let action = args.next().expect("No action was provided!");

    // TODO:
    // add CLIOption struct which takes valueless & valued options
    let option = args.next().expect("No option was provided!");
    let value = args.next().expect("No value was provided!");
    let option2 = args.next().expect("No option was provided!");
    let value2 = args.next().expect("No value was provided!");

    if action == "add" && option == "-d" && option2 == "-date" {
        let entry = Entry::add(value, value2).expect("err");
        dbg!(entry);
    };
}
// mut args: impl Iterator<Item = String>
