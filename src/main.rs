use console::{style, Term};
use include_dir::{include_dir, Dir};

use crate::formatter::build_output;
use crate::output::{write_to_file, write_to_stdout};
use crate::wizard::{ask_save_location, ask_tech, Location};

pub mod formatter;
pub mod output;
pub mod wizard;
static TEMPLATE_MAINSTREAM_DIR: Dir = include_dir!("templates/mainstream");

fn main() {
    let file_names = TEMPLATE_MAINSTREAM_DIR
        .files()
        .filter_map(|f| f.path().file_stem())
        .filter_map(|f| f.to_str())
        .collect();

    let stdout = Term::stdout();
    let stderr = Term::stderr();

    stdout.clear_screen().unwrap();

    let selection = ask_tech(file_names);

    println!(
        "Generating .gitignore for: {:?}",
        style(selection.join(", ")).bold()
    );

    let location = ask_save_location();

    let output = build_output(selection, &TEMPLATE_MAINSTREAM_DIR);

    match location {
        Location::Output => write_to_stdout(&output, &stdout),
        Location::File => write_to_file(&output, &stdout, &stderr),
    }
}
