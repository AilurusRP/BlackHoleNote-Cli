use base64::encode;
use rpassword::read_password;
use std::fs;
use std::io::Write;

fn console_reader() -> String {
    println!("Input your notes here, or input \"exit()\" to leave:");
    let console_input = read_password().expect("Failed to read input!");
    if console_input == "exit()" {
        return console_input;
    }
    text_print_and_save(console_input);
    String::new()
}

fn text_print_and_save(mut text: String) {
    let mut file = load_file();
    text = encode(text);
    text.push('\n');
    println!("{}", text);
    file.write_all(text.as_bytes()).expect("Failed to write!");
}

fn load_file() -> std::fs::File {
    fs::OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open("./note.txt")
        .expect("Failed to load file!")
}

fn main() {
    loop {
        if console_reader() == "exit()" {
            break;
        };
    }
}
