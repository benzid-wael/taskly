pub trait Manageable {
    fn id(&self) -> u64;
    fn title(&self) -> String;
    fn set_title(&mut self);
    fn description(&self) -> String;
    fn set_description(&mut self);
    fn created_at(&self) -> String;
}

pub trait Starrable {
    fn star(&mut self);
    fn unstar(&mut self);
}

pub trait Taggable {
    fn add_tag(&mut self, tag: String);
    fn remove_tag(&mut self, tag: String);
}
