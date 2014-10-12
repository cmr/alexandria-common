extern crate time;

use time::Timespec;

pub struct History {
    pub isbn: String,
    pub student_id: String,
    pub date: Timespec,
    pub action: Action,
}

pub enum Action {
    CheckOut,
    CheckIn,
}

pub struct User {
    pub name: String,
    pub email: String,
    pub student_id: String,
}

pub struct Book {
    pub name: String,
    pub description: String,
    pub isbn: String,
    pub cover_image: Vec<u8>,
    pub available: i16,
    pub quantity: i16,
    pub active_date: Timespec,
    pub permission: Permission,
}

pub enum Permission {
    DontLeaveLibrary,
    FreeToCheckOut,
}
