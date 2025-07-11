
use std::fs::File;
use std::io::Read;
use std::path::Path;

use serde::{Serialize, Deserialize};
use ron::{ser::PrettyConfig, de::from_bytes as ron_reader, Options as ron_writer, Result as RonResult};

use super::{Nodal, DirEdge, DiGraph};

pub trait FileIO: Default + Serialize + for<'a> Deserialize<'a>
{
    fn config() -> PrettyConfig {
        PrettyConfig::new()
            .depth_limit(4)
            .indentor("\t")
    }
    fn load_or_default<P: AsRef<Path>>(path: P) -> Self {
        Self::load_from_file(path).unwrap_or(Self::default())
    }
    fn load_from_file<P: AsRef<Path>>(path: P) -> Option<Self> {
        if let Ok(mut file) = File::open(path) {
            let mut buf = vec![];
            if file.read_to_end(&mut buf).is_ok() {
                if let Ok(loaded_item) = ron_reader(&buf[..]) {
                    return Some(loaded_item);
                }
            }
        }
        // No file, or failure to load
        None
    }
    fn save_to_file<P: AsRef<Path>>(&self, path: P) -> RonResult<()> {
        let f = File::create(path)?;
        ron_writer::default()
            .to_io_writer_pretty(f, self, Self::config())?;
        Ok(())
    }
}

impl<N, E> FileIO for DiGraph<N, E>
where
    N: Nodal,
    E: DirEdge
{
    // Default implementations for
    //  - config()
    //  - load_or_default()
    //  - load_from_file()
    //  - save_to_file()
}
