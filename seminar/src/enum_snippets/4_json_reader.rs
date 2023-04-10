use std::{env, process};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use serde_json::{Value, Error};

enum JsonStatus {
    Valid(Value),
    Invalid(String),
}

fn main() -> std::io::Result<()> {
    let path = env::current_dir()?;
    println!("The current directory is {}", path.display());

    let file_path = Path::new("src/enum_snippets/examples/example.json");

    match read_json_file(file_path) {
        JsonStatus::Valid(json) => println!("File is valid JSON: {}", json),
        JsonStatus::Invalid(reason) => {
            eprintln!("File is not valid JSON: {}", reason);
            process::exit(1)
        },
    }

    Ok(())
}

fn read_json_file(file_path: &Path) -> JsonStatus {
    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(e) => return JsonStatus::Invalid(format!("Could not open file: {}", e)),
    };

    let mut contents = String::new();

    if let Err(e) = file.read_to_string(&mut contents) {
        return JsonStatus::Invalid(format!("Could not read file: {}", e));
    }

    match serde_json::from_str(&contents) {
        Ok(json) => JsonStatus::Valid(json),
        Err(e) => JsonStatus::Invalid(format!("Invalid JSON: {}", e)),
    }
}
