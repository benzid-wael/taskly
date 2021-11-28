use std::time::SystemTime;

pub type EID = u16;

pub trait Manageable {
    fn id(&self) -> EID;
    fn title(&self) -> String;
    fn set_title(&mut self, title: String);
    fn description(&self) -> String;
    fn set_description(&mut self, description: String);
    fn created_at(&self) -> SystemTime;
}

pub trait Starrable {
    fn star(&mut self);
    fn unstar(&mut self);
}

pub trait Taggable {
    fn add_tag(&mut self, tag: String);
    fn remove_tag(&mut self, tag: String);
}
