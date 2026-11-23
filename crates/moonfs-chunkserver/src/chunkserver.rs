use std::{fs, path::PathBuf};

#[derive(Default)]
struct Chunkserver {
    base_dir: PathBuf,
}

impl Chunkserver {
    pub fn read_chunk(&self, id: usize) -> std::io::Result<Vec<u8>> {
        let path = self.base_dir.join(id.to_string());
        let bytes = fs::read(&path)?;
        Ok(bytes)
    }

    pub fn write_chunk(&self, content: &[u8], id: usize) -> std::io::Result<()> {
        let path = self.base_dir.join(id.to_string());
        fs::write(&path, content)?;
        Ok(())
    }
}
