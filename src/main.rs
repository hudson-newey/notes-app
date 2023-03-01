use std::path::Path;
use std::fs;
use std::io;

fn main() {
    const INIT_CONSOLE_TEXT: &str = "
        ===================
        ===Notes Program===
        ===================
    ";
    const NOTES_FILE_NAME: &str = "notes.txt";
    const INITIAL_FILE_HEADERS: &str = "Notes:";

    println!("{}", INIT_CONSOLE_TEXT);

    if ! Path::new(NOTES_FILE_NAME).exists() {
        write_to_file(NOTES_FILE_NAME, INITIAL_FILE_HEADERS);
    }

    loop {
        let file_contents: String = read_file(NOTES_FILE_NAME);
        println!("{}", file_contents);

        let new_line: &str =&String::new();
        io::stdin().read_line(&mut new_line.to_string()).expect("failed to readline");

        write_to_file(NOTES_FILE_NAME, new_line)
    }
}

fn read_file(path: &str) -> String {
    return fs::read_to_string(path)
        .expect("Should have been able to read the file");
}

fn write_to_file(path: &str, content: &str) {
    fs::write(path, content)
        .expect("Unable to write file");
}
