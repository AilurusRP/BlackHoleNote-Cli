use base64::{decode, encode};
use rpassword::read_password;
use std::fs;
use std::io::Write;

enum Input {
    Decode,
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
        "decode()" => Input::Decode,
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

fn decode_notes() {
    let notes = fs::read_to_string("./note.txt").unwrap();
    let notes_list = notes.split('\n').collect::<Vec<&str>>();
    let notes_string = notes_list
        .iter()
        .map(|note_text| String::from_utf8(decode(note_text).unwrap()).unwrap() + "\n")
        .collect::<String>();
    fs::write("decoded_notes.txt", notes_string).unwrap();
}

fn main() {
    loop {
        match console_reader() {
            Input::Decode => decode_notes(),
            Input::Exit => break,
            _ => (),
        }
    }
}
