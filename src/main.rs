use std::fmt::Display;
use std::fs::OpenOptions;
use std::io::{Write, Cursor, stdout};

use console::Term;
use include_dir::{include_dir, Dir, File};
use inquire::{MultiSelect, Select};

static TEMPLATE_MAINSTREAM_DIR: Dir = include_dir!("templates/mainstream");

enum Location {
    FILE,
    OUTPUT,
}

impl Location {
    fn variants() -> Vec<Location> {
        vec![Location::FILE, Location::OUTPUT]
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let desc = match self {
            Location::FILE => ".gitignore file",
            Location::OUTPUT => "stdout",
        };

        write!(f, "{}", desc)
    }
}

fn main() {
    let files = TEMPLATE_MAINSTREAM_DIR
        .files()
        .filter(|f| f.path().file_stem().is_some());

    let mut file_names: Vec<String> = Vec::new();

    for f in files.filter(|f| f.path().file_name().is_some()) {
        file_names.push(
            f.path()
                .file_stem()
                .unwrap()
                .to_str()
                .to_owned()
                .unwrap()
                .to_string(),
        );
    }

    let stdout = Term::stdout();
    let stderr = Term::stderr();

    stdout.clear_screen().unwrap();

    let selection = MultiSelect::new("Select the desired technology", file_names)
        .with_formatter(&|a| format!("{} different fruits", a.len()))
        .prompt()
        .unwrap();

    println!("Generating .gitignore for: {:?}", selection.join(", "),);

    let location = Select::new("Where do you like to get?", Location::variants())
        .prompt()
        .unwrap();

    let mut output: String = String::new();

    for tech in selection {
        let ignore_content = TEMPLATE_MAINSTREAM_DIR
            .get_file(format!("{}.gitignore", tech))
            .unwrap();

        output.push_str(format!("\n\n######### Start of automatic generated ignores for {} ##########\n", tech ).as_ref());
        output.push_str(ignore_content.contents_utf8().unwrap());
        output.push_str(format!("\n######### End of automatic generated ignores for {} ##########\n", tech ).as_ref());
    }

    match location {
        Location::OUTPUT => {
            stdout.write_line("------ OUTPUT: -------").unwrap();
            stdout.write_line(output.as_ref()).unwrap();
        }
        Location::FILE => {
            let mut file = OpenOptions::new()
                .append(true)
                .open(".gitignore")
                .unwrap();

            if let Err(e) = file.write_all(output.as_bytes()) {
                stderr.write_line(format!("Couldn't write to file: {}", e).as_ref()).unwrap();
            }
        }
    }
}
