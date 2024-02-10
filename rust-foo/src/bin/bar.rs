use std::process;

fn main() {
    println!("bar.rs executed");

    let ls_output = process::Command::new("ls")
        .arg("-l")
        .arg("-a")
        .output()
        .expect("failed to execute process");

    let string_output = String::from_utf8_lossy(&ls_output.stdout).to_string();

    println!("{}", string_output);
}

