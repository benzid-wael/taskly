mod entities;

use clap::{App, load_yaml, crate_version};
use entities::Note;
use entities::Taggable;


fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml)
                        .version(crate_version!())
                        .get_matches();
    println!("matches: {:#?}", matches);
    let mut note = Note::new(1, String::from("My first note"), String::from(""), false, vec![]);

    note.add_tag("hello".to_string());
    note.add_tag("world".to_string());
    note.add_tag("board".to_string());
    println!("notes: {:#?}", note);
}
