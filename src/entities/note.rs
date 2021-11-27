use std::time::SystemTime;

use super::item::{Starrable, Taggable};


#[derive(Debug)]
pub struct Note {
    id: u64,
    title: String,
    description: String,
    created_at: SystemTime,
    is_starred: bool,
    tags: Vec<String>,
}


impl Note {
    pub fn new(id: u64, title: String, description: String, is_starred: bool, tags: Vec<String>) -> Note {
        Note {
            id,
            title,
            description,
            created_at: SystemTime::now(),
            is_starred,
            tags,
        }
    }
}

impl Starrable for Note {
    #[allow(dead_code)]
    fn star(&mut self) {
        self.is_starred = true;
    }

    #[allow(dead_code)]
    fn unstar(&mut self) {
        self.is_starred = false;
    }
}

impl Taggable for Note {
    fn add_tag(&mut self, tag: String) {
        self.tags.push(tag);
    }

    fn remove_tag(&mut self, tag: String) {
        self.tags.retain(|t| t != &tag);
    }
}
