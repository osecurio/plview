use core::fmt;

use crate::mtk_loaders::gfh_headers::MtkGfhHeader;

/*
#define GFH_HEADER_MAGIC		0x4D4D4D
#define GFH_HEADER_VERSION_SHIFT	24

#define GFH_TYPE_FILE_INFO	0
#define GFH_TYPE_BL_INFO	1
#define GFH_TYPE_BROM_CFG	7
#define GFH_TYPE_BL_SEC_KEY	3
#define GFH_TYPE_ANTI_CLONE	2
#define GFH_TYPE_BROM_SEC_CFG	8
*/

macro_rules! GFH_HEADER_MAGIC {
    () => {
        [0x4D, 0x4D, 0x4D]
    };
}

#[derive(Debug, Clone, Copy)]
pub enum GfhHeaderType {
    GFH_TYPE_FILE_INFO = 0,
    GFH_TYPE_BL_INFO = 1,
    GFH_TYPE_ANTI_CLONE = 2,
    GFH_TYPE_BL_SEC_KEY = 3,
    GFH_TYPE_BROM_CFG = 7,
    GFH_TYPE_BROM_SEC_CFG = 8,
    Unknown = 0xffff,
}

impl fmt::Display for GfhHeaderType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::GFH_TYPE_FILE_INFO => "GFH_TYPE_FILE_INFO",
            Self::GFH_TYPE_BL_INFO => "GFH_TYPE_BL_INFO",
            Self::GFH_TYPE_ANTI_CLONE => "GFH_TYPE_ANTI_CLONE",
            Self::GFH_TYPE_BL_SEC_KEY => "GFH_TYPE_BL_SEC_KEY",
            Self::GFH_TYPE_BROM_CFG => "GFH_TYPE_BROM_CFG",
            Self::GFH_TYPE_BROM_SEC_CFG => "GFH_TYPE_BROM_SEC_CFG",
            Self::Unknown => "GFH_UNKNOWN",
        };
        write!(f, "{s}")
    }
}

impl From<u16> for GfhHeaderType {
    fn from(value: u16) -> Self {
        match value {
            0 => Self::GFH_TYPE_FILE_INFO,
            1 => Self::GFH_TYPE_BL_INFO,
            2 => Self::GFH_TYPE_ANTI_CLONE,
            3 => Self::GFH_TYPE_BL_SEC_KEY,
            7 => Self::GFH_TYPE_BROM_CFG,
            8 => Self::GFH_TYPE_BROM_SEC_CFG,
            _ => Self::Unknown,
        }
    }
}

impl Into<u16> for GfhHeaderType {
    fn into(self) -> u16 {
        match self {
            Self::GFH_TYPE_FILE_INFO => 0,
            Self::GFH_TYPE_BL_INFO => 1,
            Self::GFH_TYPE_ANTI_CLONE => 2,
            Self::GFH_TYPE_BL_SEC_KEY => 3,
            Self::GFH_TYPE_BROM_CFG => 7,
            Self::GFH_TYPE_BROM_SEC_CFG => 8,
            _ => 0xffff,
        }
    }
}

const GFH_COMMON_MAGIC_VERSION_OFFSET: usize = 0x0;
const GFH_COMMON_SIZE_OFFSET: usize = 0x4;
const GFH_COMMON_TYPE_OFFSET: usize = 0x6;

#[derive(Debug, Clone, Copy)]
pub struct GfhCommonHeader {
    magic_version: u32,
    size: u16,
    gfh_type: GfhHeaderType,
}

impl GfhCommonHeader {
    pub fn get_size(&self) -> u16 {
        8
    }

    pub fn get_type(&self) -> GfhHeaderType {
        self.gfh_type
    }
}

impl fmt::Display for GfhCommonHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "    magic_version: 0x{:X}\n    size: 0x{:X}\n    gfh_type: {}", self.magic_version, self.size, self.gfh_type)
    }
}

impl MtkGfhHeader for GfhCommonHeader {
    type Header = GfhCommonHeader;
    fn load(data: &[u8], mut offset: usize) -> Option<Self::Header> {

        let mv = *data[offset..offset+size_of::<u32>()].as_array().unwrap();
        if mv[0..3] != GFH_HEADER_MAGIC!() { return None}
        let magic_version = u32::from_le_bytes(mv);
        offset += 0x4;

        let sz = *data[offset..offset+size_of::<u16>()].as_array().unwrap();
        let size = u16::from_le_bytes(sz);
        offset += 0x2;

        let t = *data[offset..offset+size_of::<u16>()].as_array().unwrap();
        let gfh_type = u16::from_le_bytes(t).into();

        Some(Self {
            magic_version,
            size,
            gfh_type,
        })
    }

    fn header_size(&self) -> usize {
        self.size as usize
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::mtk_loaders::gfh_headers::{MtkGfhHeader, gfh_common::GfhCommonHeader};

    #[test]
    fn test_gfh_common_parse() {
        let data = fs::read("./testbins/test.bin").unwrap();
        let gfh_ch = GfhCommonHeader::load(&data, 0).unwrap();
        println!("{gfh_ch}");
    }
}