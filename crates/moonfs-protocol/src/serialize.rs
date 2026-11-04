use serde::{Deserialize, Serialize};

pub enum SerializeErr {
    Corrupted,
    // TODO: add more error if needed
}

// Generic F(ile), D(irectory)
pub enum SerializedPayload<F, D> {
    File(F),
    Directory(D),
}

/*
 * Packed so compiler doesn't add paddings
 * */

#[derive(Serialize, Deserialize, Debug)]
#[repr(C, packed)]
pub struct Header<'a> {
    pub name: &'a str,
    pub size: u64,
    pub permissions: u16,
}

#[repr(C, packed)]
pub struct FilePayload {
    // TODO: Add to this field
    pub content: Vec<u8>,
}

#[repr(C, packed)]
pub struct DirectoryPayload {
    // TODO: Add to this field
}

pub fn serialize<T: Serialize>(data: &T) -> Result<Vec<u8>, SerializeErr> {
    bincode::serialize(data).map_err(|_| SerializeErr::Corrupted)
}
