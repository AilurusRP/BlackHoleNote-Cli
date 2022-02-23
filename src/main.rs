use base64::encode;
use rpassword::read_password;
use std::fs;
use std::io::Write;

enum Input {
    Exit,
    Other,
}

fn load_file() -> fs::File {
    fs::OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open("./note.txt")
        .expect("Failed to load file!")
}

fn text_print_and_save(mut text: String) {
    let mut file = load_file();
    text = encode(text);
    text.push('\n');
    println!("{}", text);
    file.write_all(text.as_bytes()).expect("Failed to write!");
}

fn handle_input(console_input: String) -> Input {
    match &console_input[..] {
        "exit()" => Input::Exit,
        _ => {
            text_print_and_save(console_input);
            Input::Other
        }
    }
}

fn console_reader() -> Input {
    println!("Input your notes here, or input \"exit()\" to leave:");
    let console_input = read_password().expect("Failed to read input!");
    handle_input(console_input)
}

fn main() {
    loop {
        if let Input::Exit = console_reader() {
            break;
        }
    }
}
