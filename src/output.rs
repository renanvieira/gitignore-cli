use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::process;

use console::{style, Term};
use inquire::Confirm;

pub fn write_to_stdout(output: &str, stdout: &Term) {
    let msg = format!("------ {} -------", style("OUTPUT:").bold());

    stdout.write_line(&msg).unwrap();
    stdout.write_line(output.as_ref()).unwrap();
}

pub fn write_to_file(output: &str, stdout: &Term, stderr: &Term) {
    if Path::new(".gitignore").exists() {
        let file_exists_confirmation =
            Confirm::new(".gitignore already exists, do you want to append the new data to it?")
                .prompt();

        if file_exists_confirmation.is_err() {
            stdout.write_line("Exiting...").unwrap();
            process::exit(exitcode::OK);
        }
    }

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(".gitignore")
        .unwrap();

    if let Err(e) = file.write_all(output.as_bytes()) {
        stderr
            .write_line(format!("Couldn't write to file: {}", e).as_ref())
            .unwrap();
    }

    stdout
        .write_line(format!("{}", style("Done!").bold().green()).as_ref())
        .unwrap();
}
