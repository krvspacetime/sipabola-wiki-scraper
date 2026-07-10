use std::{
    fs::{self, File},
    io::BufWriter,
    path::Path,
};

use crate::MatchRecord;

pub struct RecordWriter;

impl RecordWriter {
    pub fn write_json(records: &[MatchRecord], path: &str) -> anyhow::Result<()> {
        let path = Path::new(path);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let file = File::create(path)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, records)?;

        Ok(())
    }
}
