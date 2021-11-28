use std::time::SystemTime;

use super::item::{Manageable, Starrable, Taggable};


#[derive(Clone)]
#[derive(Debug)]
pub enum Status {
    Todo,
    InProgress,
    OnHold,
    Done,
    Cancelled,
}


pub trait FSM {
    fn status(&self) -> Status;

    fn _update_status(&mut self, status: Status);

    fn start_task(&mut self) {
        self._update_status(Status::InProgress);
    }

    fn stop_task(&mut self) {
        self._update_status(Status::OnHold);
    }

    fn complete_task(&mut self) {
        self._update_status(Status::Done);
    }

    fn cancel_task(&mut self) {
        self._update_status(Status::Cancelled);
    }
}


#[derive(Debug)]
pub struct Task {
    /* Common Item fields */
    id: u64,
    title: String,
    description: String,
    created_at: SystemTime,
    is_starred: bool,
    tags: Vec<String>,
    /* Custom Task fields */
    status: Status,
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

impl Manageable for Task {
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

impl Taggable for Task {
    fn add_tag(&mut self, tag: String) {
        self.tags.push(tag);
    }

    fn remove_tag(&mut self, tag: String) {
        self.tags.retain(|t| t != &tag);
    }
}

impl FSM for Task {
    fn status(&self) -> Status {
        self.status.clone()
    }

    fn _update_status(&mut self, status: Status) {
        self.status = status;
    }
}
