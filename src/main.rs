mod files;
mod util;

use std::path::Path;

fn main() {
    const NOTES_FILE_NAME: &str = "notes.txt";
    const INITIAL_FILE_HEADERS: &str = "Notes:";

    if ! Path::new(NOTES_FILE_NAME).exists() {
        files::write_to_file(NOTES_FILE_NAME, INITIAL_FILE_HEADERS);
    }

    loop {
        let file_contents: String = files::read_file(NOTES_FILE_NAME);
        clearscreen::clear().expect("Unable to clear screen");
        println!("{}", file_contents);

        println!("Enter a new note: ");
        let new_note = util::get_user_input();

        files::write_to_file(NOTES_FILE_NAME, new_note.as_str());
    }
}
