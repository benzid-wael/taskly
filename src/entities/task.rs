use std::time::SystemTime;

use super::item::{Starrable, Taggable};


#[derive(Debug)]
pub enum Status {
    Todo,
    InProgress,
    OnHold,
    Done,
    Cancelled,
}


pub trait FSM {
    fn _update_status(&mut self, status: Status);

    fn start(&mut self) {
        self._update_status(Status::InProgress);
    }

    fn stop(&mut self) {
        self._update_status(Status::OnHold);
    }

    fn complete(&mut self) {
        self._update_status(Status::Done);
    }

    fn cancel(&mut self) {
        self._update_status(Status::Cancelled);
    }
}


#[derive(Debug)]
pub struct Task {
    id: u64,
    title: String,
    description: String,
    status: Status,
    created_at: SystemTime,
    is_starred: bool,
    tags: Vec<String>,
}


impl Task {
    pub fn new(id: u64, title: String, description: String, status: Status, is_starred: bool, tags: Vec<String>) -> Task {
        Task {
            id,
            title,
            description,
            status,
            created_at: SystemTime::now(),
            is_starred,
            tags,
        }
    }
}

impl Starrable for Task {
    #[allow(dead_code)]
    fn star(&mut self) {
        self.is_starred = true;
    }

    #[allow(dead_code)]
    fn unstar(&mut self) {
        self.is_starred = false;
    }
}

// impl Taggable for Task {
//     fn add_tag(&mut self, tag: String) {
//         self.tags.push(tag);
//     }

//     fn remove_tag(&mut self, tag: String) {
//         self.tags.retain(|t| t != &tag);
//     }
// }

impl FSM for Task {
    fn _update_status(&mut self, status: Status) {
        self.status = status;
    }
}
