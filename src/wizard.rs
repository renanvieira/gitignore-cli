use std::fmt::Display;

use inquire::{MultiSelect, Select};

pub enum Location {
    File,
    Output,
}

impl Location {
    fn variants() -> Vec<Location> {
        vec![Location::File, Location::Output]
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let desc = match self {
            Location::File => ".gitignore file",
            Location::Output => "stdout",
        };

        write!(f, "{}", desc)
    }
}


pub fn ask_tech(file_names: Vec<&str>) -> Vec<&str> {
    MultiSelect::new("Select the desired technology", file_names)
        .with_formatter(&|a| format!("{} items selected", a.len()))
        .prompt()
        .unwrap()
}

pub fn ask_save_location() -> Location {
    Select::new(
        "Where do you like to put the gitignore rules?",
        Location::variants(),
    )
    .prompt()
    .unwrap()
}
