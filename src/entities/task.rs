use std::fmt;
use std::time::SystemTime;
use std::error::Error;

use super::item::{EID, Manageable, Starrable, Taggable};


//// Errors
#[derive(Debug, Clone)]
pub enum FSMErrorKind {
    UnallowedTransition,
}

#[derive(Debug, Clone)]
pub struct FSMError {
    pub message: String,
    pub kind: FSMErrorKind,
}

impl FSMError {
    fn new(message: String, kind: FSMErrorKind) -> FSMError {
        FSMError {
            message,
            kind,
        }
    }
}

impl fmt::Display for FSMError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let prefix = match self.kind {
            FSMErrorKind::UnallowedTransition => "Unallowed transition",
        };
        write!(f, "{}: {}", prefix, self.message)
    }
}

impl Error for FSMError {
    fn description(&self) -> &str {
        &self.message
    }
}


//// Enums
#[derive(Clone)]
#[derive(Debug)]
pub enum Status {
    Todo,
    InProgress,
    OnHold,
    Done,
    Cancelled,
}

macro_rules! transition {
    ( $method:ident, $enum_type:ty, $dest:ident, $($val:ident),+ ) => {
        fn $method(&mut self) -> Result<(), FSMError> {
            match self.status() {
                $(
                    <$enum_type>::$val => {
                        self._update_status(<$enum_type>::$dest);
                        return Ok(());
                    }
                )+
                _ => {
                    let message = format!("Cannot move task to `{:?}` state, current state: `{:?}`", self.status(), <$enum_type>::$dest);
                    println!("[{}]: Unallowed transition: {:?} -> {:?}", stringify!{$method}, self.status(), <$enum_type>::$dest);
                    return Err(FSMError::new(message, FSMErrorKind::UnallowedTransition));
                }
            }
        }
    };
}

pub trait FSM {
    fn status(&self) -> Status;

    fn _update_status(&mut self, status: Status);

    transition!{start, Status, InProgress, Todo, OnHold, Done, Cancelled}
    transition!{stop, Status, OnHold, InProgress}
    transition!{complete, Status, Done, Todo, InProgress, OnHold, Cancelled}
    transition!{cancel, Status, Cancelled, Todo, InProgress, OnHold, Done}
}


#[derive(Debug)]
pub struct Task {
    /* Common Item fields */
    id: EID,
    title: String,
    description: String,
    created_at: SystemTime,
    is_starred: bool,
    tags: Vec<String>,
    /* Custom Task fields */
    status: Status,
}


impl Task {
    pub fn new(id: EID, title: String, description: String, status: Status, is_starred: bool, tags: Vec<String>) -> Task {
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
    fn id(&self) -> EID {
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
