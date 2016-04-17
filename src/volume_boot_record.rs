
use byteorder::{LittleEndian,ReadBytesExt};
use std::io::Read;
use std::io;
use std::io::{Seek,SeekFrom};

#[derive(Debug)]
pub enum MediaDescriptor {
    Unknowen,
    HardDisk,
    FloppyDisk(u8),
}

#[derive(Debug)]
pub struct VolumeBootRecord {
    pub identifier: String,
    pub bytes_per_sector: u16,
    pub sectord_per_cluster: u8,
    pub reserved_sector_count: u16,
    pub fat_count: u8,
    pub max_root_dir_entries: u16,
    pub total_logical_sectors: u16,
    pub media_descriptor: MediaDescriptor,
    pub sectors_per_fat: u16,
    pub sectors_per_track: u16,
    pub head_count: u16,
    pub hidden_sectors: u32,
    pub total_sectors: u32,
}



impl VolumeBootRecord {
    fn empty() -> VolumeBootRecord {
        VolumeBootRecord {
            identifier: String::new(),
            bytes_per_sector: 0,
            sectord_per_cluster: 0,
            reserved_sector_count: 0,
            fat_count: 0,
            max_root_dir_entries: 0,
            total_logical_sectors: 0,
            media_descriptor: MediaDescriptor::Unknowen,
            sectors_per_fat: 0,
            sectors_per_track: 0,
            head_count: 0,
            hidden_sectors: 0,
            total_sectors: 0,
        }
    }

    pub fn new<T: Read + Seek>(descriptor: &mut T) -> io::Result<VolumeBootRecord,> {
        descriptor.seek(SeekFrom::Start(0));

        let mut ret = VolumeBootRecord::empty();
        let mut _buffer: [u8; 0x3] = [0; 0x3];
        descriptor.read(&mut _buffer)?;
        assert!(_buffer == [0xeb, 0x3c, 0x90], "no valid FAT entrypoint!");

        let mut identifier: [u8; 0x8] = [0; 0x8];
        descriptor.read(&mut identifier)?;
        ret.identifier = String::from_utf8_lossy(&identifier[0..8]).into_owned();

        ret.bytes_per_sector = descriptor.read_u16::<LittleEndian>()?;
        ret.sectord_per_cluster = descriptor.read_u8()?;
        ret.reserved_sector_count = descriptor.read_u16::<LittleEndian>()?;
        ret.fat_count = descriptor.read_u8()?;
        ret.max_root_dir_entries = descriptor.read_u16::<LittleEndian>()?;
        ret.total_logical_sectors = descriptor.read_u16::<LittleEndian>()?;

        let mut _buffer: u8 = 0;
        _buffer = descriptor.read_u8()?;
        ret.media_descriptor = match _buffer {
            0xF8 => MediaDescriptor::HardDisk,
            v @ 0xF9 ... 0xFF => MediaDescriptor::FloppyDisk(v),
            v @ 0xF0 => MediaDescriptor::FloppyDisk(v),
            _ => MediaDescriptor::Unknowen
        };

        ret.sectors_per_fat = descriptor.read_u16::<LittleEndian>()?;
        ret.sectors_per_track = descriptor.read_u16::<LittleEndian>()?;
        ret.head_count = descriptor.read_u16::<LittleEndian>()?;
        ret.hidden_sectors = descriptor.read_u32::<LittleEndian>()?;
        ret.total_sectors = descriptor.read_u32::<LittleEndian>()?;
        Ok(ret)
    }
}