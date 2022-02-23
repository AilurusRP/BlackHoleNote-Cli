use base64::encode;
use rpassword::read_password;
use std::fs;
use std::io::Write;

fn console_reader() -> String {
    println!("Input your notes here, or input \"exit()\" to leave:");

    let console_input = read_password();

    match console_input {
        Ok(val) => {
            if text_process(val) == "exit()" {
                return String::from("exit()");
            }
        }
        Err(val) => println!("{}\n", val),
    }
    String::from("continue")
}

fn text_process(mut text: String) -> String {
    if text == "exit()" {
        return text;
    }
    println!("{}\n", text);
    let mut file = fs::OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open("./note.txt")
        .expect("Failed to open file!");
    text = encode(text);
    text.push('\n');
    file.write_all(text.as_bytes()).expect("Failed to write!");

    String::new()
}

fn main() {
    loop {
        if console_reader() == "exit()" {
            break;
        };
    }
}
