use std::env::current_dir;
use std::fs::read_dir;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let something = current_dir()?;
    let entries = read_dir(something)?;
    let mut contents = vec![];

    for entry in entries {
        contents.push(entry?.file_name());
    }

    contents.sort();

    let contents = contents
        .iter()
        .map(|x| x.to_str().unwrap())
        .filter(|x| !x.starts_with("."))
        .collect::<Vec<&str>>()
        .join(" ");

    println!("{}", contents);

    Ok(())
}
