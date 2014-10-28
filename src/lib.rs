extern crate time;
extern crate serialize;

use time::Timespec;

#[deriving(Encodable, Decodable, Show)]
pub struct History {
    pub isbn: String,
    pub student_id: String,
    pub date: Timespec,
    pub action: Action,
}

#[deriving(Encodable, Decodable, Show, Hash, FromPrimitive)]
pub enum Action {
    CheckOut,
    CheckIn,
}

#[deriving(Encodable, Decodable, Show, Hash)]
pub struct ActionRequest {
    pub action: Action,
    pub isbn: String,
    pub student_id: String
}

#[deriving(Encodable, Decodable, Show, Hash)]
pub struct User {
    pub name: String,
    pub email: String,
    pub student_id: String,
}

#[deriving(Encodable, Decodable, Show)]
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

#[deriving(Encodable, Decodable, Show, Hash, FromPrimitive)]
pub enum Permission {
    DontLeaveLibrary,
    FreeToCheckOut,
}

/// Convert from an i16 (SMALLINT in the database) to an enum variant if it's valid
pub fn enum_from_id<T: FromPrimitive>(id: i16) -> Option<T> {
    FromPrimitive::from_i16(id)
}
