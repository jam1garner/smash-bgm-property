//! A Rust library for working with `bgm_property.bin` files from Smash Ultimate. This allows for
//! modifying various properties associated with  background music.
//! 
/// ```rust
/// # fn main() -> binread::BinResult<()> {
/// use bgm_property::BgmPropertyFile;
/// 
/// let mut file = BgmPropertyFile::open("bgm_property.bin")?;
/// 
/// for entry in file.entries() {
///     println!("name_id: {:#X}", entry.name_id);
/// }
/// 
/// for entry in file.entries_mut() {
///     entry.loop_start_sample = 0;
/// }
/// 
/// file.save("bgm_property_out.bin")?;
/// # Ok(())
/// # }
/// ```

use binread::{BinRead, BinReaderExt, derive_binread, BinResult};
use binwrite::{BinWrite, WriterOption};

use std::fs::File;
use std::path::Path;
use std::io::{self, Write, BufReader, BufWriter};

/// Type alias for Hash40
pub type Hash40 = u64;

/// ```rust
/// # fn main() -> binread::BinResult<()> {
/// use bgm_property::BgmPropertyFile;
/// 
/// let mut file = BgmPropertyFile::open("bgm_property.bin")?;
/// 
/// for entry in file.entries() {
///     println!("name_id: {:#X}", entry.name_id);
/// }
/// 
/// for entry in file.entries_mut() {
///     entry.loop_start_sample = 0;
/// }
/// 
/// file.save("bgm_property_out.bin")?;
/// # Ok(())
/// # }
/// ```
#[derive_binread]
#[derive(Debug)]
#[br(magic = b"PMGB")]
pub struct BgmPropertyFile (
    #[br(temp)]
    u32,

    #[br(count = self_0)]
    Vec<Entry>,
);

impl BinWrite for BgmPropertyFile {
    fn write_options<W: Write>(&self, writer: &mut W, options: &WriterOption) -> io::Result<()> {
        (
            "PMGB",
            self.0.len() as u32,
            &self.0
        ).write_options(writer, options)
    }
}

/// An entry representing a single nus3audio background music file
#[derive(BinRead, BinWrite, Debug)]
pub struct Entry {
    pub name_id: Hash40,
    pub unk: u32,
    pub loop_start_sample: u32,
    pub unk_sample: u32,
    pub loop_end_sample: u32,
    pub unk2: u32,
    
    #[br(pad_after = 4)]
    #[binwrite(pad_after(0x4))]
    pub total_samples: u32,
}

impl BgmPropertyFile {
    pub fn open<P: AsRef<Path>>(path: P) -> BinResult<Self> {
        BufReader::new(File::open(path)?).read_le()
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        self.write(&mut BufWriter::new(File::create(path)?))
    }

    pub fn write<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        self.write_options(writer, &binwrite::writer_option_new!(endian: binwrite::Endian::Little))
    }

    pub fn new(entries: Vec<Entry>) -> Self {
        BgmPropertyFile(entries)
    }

    pub fn entries(&self) -> &Vec<Entry> {
        &self.0
    }

    pub fn entries_mut(&mut self) -> &mut Vec<Entry> {
        &mut self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round_trip() {
        let original = std::fs::read("bgm_property.bin").unwrap();
        let bgm_property = BgmPropertyFile::open("bgm_property.bin").unwrap();

        println!("{:#X?}", bgm_property);

        let mut round_trip = Vec::new();
        bgm_property.write(&mut round_trip).unwrap();

        assert_eq!(original, round_trip);
        //bgm_property.save("bgm_property_out.bin").unwrap();
    }
}
