use std::{collections::HashMap, net::IpAddr, path::Path};

enum NamespaceNode {
    Directory(HashMap<String, NamespaceNode>),
    File(Vec<[usize; 3]>),
}

pub struct Master {
    namespace_node: NamespaceNode,
    chunkservers_addr: Vec<IpAddr>,
}

pub enum MasterOperationError {
    PathNotFound,
    InvalidChunk,
}

impl Master {
    pub fn run(&mut self) -> ! {
        loop {}
    }

    pub fn lookup(&self, path: &Path, chunk_idx: usize) -> Result<[usize; 3], MasterOperationError> {
        let path_iter = path.iter()
            .filter_map(|os_str| os_str.to_str())
            .map(String::from);

        let mut node = &self.namespace_node;
        for segment in path_iter {
            match node {
                NamespaceNode::Directory(m) => {
                    if let Some(data) = m.get(&segment) {
                        node = data;
                    } else {
                        return Err(MasterOperationError::PathNotFound);
                    }
                }, 
                _ => {}
            }
        }

        let NamespaceNode::File(chunkservers) = node else {
            return Err(MasterOperationError::PathNotFound)
        };

        if let None = chunkservers.get(chunk_idx) {
            Err(MasterOperationError::InvalidChunk)
        } else {
            Ok(chunkservers[chunk_idx].clone())
        }
    }
}
