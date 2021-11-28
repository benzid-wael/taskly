use std::time::SystemTime;

use super::item::{Manageable, Starrable, Taggable};


#[derive(Debug)]
pub struct Note {
    /* Common Item fields */
    id: u64,
    title: String,
    description: String,
    created_at: SystemTime,
    is_starred: bool,
    tags: Vec<String>,
    /* Custom Note fields */
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

impl Manageable for Note {
    /* Getters */
    fn id(&self) -> u64 {
        self.id.clone()
    }

    fn title(&self) -> String {
        self.title.clone()
    }

    fn description(&self) -> String {
        self.description.clone()
    }

    fn created_at(&self) -> SystemTime {
        self.created_at.clone()
    }

    /* Setters */

    fn set_title(&mut self, title: String) {
        self.title = title
    }

    fn set_description(&mut self, description: String) {
        self.description = description;
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
