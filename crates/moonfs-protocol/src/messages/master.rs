use std::{net::IpAddr, path::PathBuf};

pub enum MasterRequest {
    GetChunkLocations(PathBuf, usize),
}

pub enum MasterError {
    MissingChunkserver, 
    PathNotFound,
    InvalidChunkIndex,
}

pub enum MasterResponse {
    ChunkLocations(Result<Vec<[IpAddr; 3]>, MasterError>)
}
