//! # bgm_property_lib
//!
//! bgm_property_lib is a library for reading and writing `bgm_property.bin` files from Super Smash Bros. Ultimate.
use std::{
    fs,
    io::{Cursor, Read, Seek, Write},
    path::Path,
};

use binrw::{binrw, BinReaderExt, BinResult, BinWrite};
pub use hash40::Hash40;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// The container type for BGM stream properties.
#[binrw]
#[brw(magic = b"PMGB", little)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Debug)]
pub struct BgmPropertyFile {
    #[br(temp)]
    #[bw(calc = entries.len() as u32)]
    entry_count: u32,

    #[br(count = entry_count)]
    pub entries: Vec<BgmPropertyEntry>,
}

impl BgmPropertyFile {
    /// Reads the data from the given reader.
    pub fn read<R: Read + Seek>(reader: &mut R) -> BinResult<Self> {
        let bgm_property = reader.read_le::<Self>()?;

        Ok(bgm_property)
    }

    /// Reads the data from the given file path.
    pub fn from_file<P: AsRef<Path>>(path: P) -> BinResult<Self> {
        let mut file = Cursor::new(fs::read(path)?);
        let bgm_property = file.read_le::<Self>()?;

        Ok(bgm_property)
    }

    /// Writes the data to the given writer.
    pub fn write<W: Write + Seek>(&self, writer: &mut W) -> BinResult<()> {
        self.write_le(writer)
    }

    /// Writes the data to the given file path.
    pub fn write_to_file<P: AsRef<Path>>(&self, path: P) -> BinResult<()> {
        let mut cursor = Cursor::new(Vec::new());

        self.write_le(&mut cursor)?;
        fs::write(path, cursor.get_mut())?;

        Ok(())
    }
}

/// A group of timing properties for a BGM stream.
#[binrw]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Debug)]
pub struct BgmPropertyEntry {
    /// Hashed name of the BGM stream.
    pub stream_name: Hash40,

    /// Beginning of the BGM stream's loop measured in milliseconds.
    pub loop_start_ms: u32,

    /// Beginning of the BGM stream's loop measured in samples.
    pub loop_start_sample: u32,

    /// End of the BGM stream's loop measured in milliseconds.
    pub loop_end_ms: u32,

    /// End of the BGM stream's loop measured in samples.
    pub loop_end_sample: u32,

    /// Duration of the BGM stream measured in milliseconds.
    pub duration_ms: u32,

    /// Duration of the BGM stream measured in samples.
    #[brw(pad_after = 4)]
    pub duration_sample: u32,
}
