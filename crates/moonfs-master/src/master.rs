use std::{collections::HashMap, net::IpAddr, path::Path};

enum NamespaceNode {
    Directory(HashMap<String, NamespaceNode>),
    File(Vec<[usize; 3]>),
}

impl Default for NamespaceNode {
    fn default() -> Self {
        NamespaceNode::Directory(HashMap::default())
    }
}

#[derive(Default)]
pub struct Master {
    namespace_node: NamespaceNode,
    chunkservers_addr: Vec<IpAddr>,
    current_chunk: usize,
}

pub enum MasterOperationError {
    PathNotFound,
    InvalidChunk,
}

impl Master {
    pub fn new(chunkservers_addr: Vec<IpAddr>) -> Self {
        Self {
            namespace_node: NamespaceNode::default(),
            chunkservers_addr,
            current_chunk: 0,
        } 
    }

    pub fn add_chunkserver(&mut self, chunkserver_addr: IpAddr) -> usize {
        self.chunkservers_addr.push(chunkserver_addr);
        self.chunkservers_addr.len() - 1
    }

    fn round_robin_next(&mut self) -> usize {
        let idx = self.current_chunk;
        self.current_chunk = (idx + 1) % self.chunkservers_addr.len();
        idx
    }

    pub fn run(&mut self) -> ! {
        loop {}
    }

    
    pub fn create_file(&mut self, path: &Path, chunk_count: i32) {
        let n = path.components().count();
        let path_iter = path.iter()
            .filter_map(|os_str| os_str.to_str());

        let mut node = &mut self.namespace_node;
        for (i, segment) in path_iter.enumerate() {
            node = match node {
                NamespaceNode::Directory(m) => {
                    if !m.contains_key(segment) {
                        let new_node = if i == n - 1 {
                            todo!()
                            //NamespaceNode::File([usize; 3])
                        } else {
                            NamespaceNode::Directory(HashMap::default())                            
                        };

                        m.insert(segment.to_string(), new_node);
                    }
                    m.get_mut(segment).unwrap()
                },
                NamespaceNode::File(a) => { node }
            } 
        }
    }

    pub fn lookup(&self, path: &Path, chunk_idx: usize) -> Result<[usize; 3], MasterOperationError> {
        let path_iter = path.iter()
            .filter_map(|os_str| os_str.to_str());

        let mut node = &self.namespace_node;
        for segment in path_iter {
            match node {
                NamespaceNode::Directory(m) => {
                    if let Some(data) = m.get(segment) {
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
