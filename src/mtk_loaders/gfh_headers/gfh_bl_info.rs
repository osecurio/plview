use core::fmt;

use crate::mtk_loaders::gfh_headers::{MtkGfhHeader, gfh_common::GfhCommonHeader};

#[derive(Debug, Clone, Copy)]
pub struct GfhBlInfo {
    gfh_type_offset: u32,
    gfh_common: GfhCommonHeader,
    attr: u32,
}

impl GfhBlInfo {
    pub fn get_header_offset(&self) -> u32 {
        self.gfh_type_offset
    }
}

impl MtkGfhHeader for GfhBlInfo {
    type Header = GfhBlInfo;
    fn load(data: &[u8], mut offset: usize) -> Option<Self::Header> {
        let gfh_type_offset = offset as u32;

        let gfh_common = GfhCommonHeader::load(&data, offset)?;
        offset += gfh_common.get_size() as usize;

        let at = *data[offset..offset + size_of::<u32>()].as_array().unwrap();
        let attr: u32 = u32::from_le_bytes(at);

        Some(Self { gfh_type_offset, gfh_common, attr })
    }

    fn header_size(&self) -> usize {
        self.gfh_common.header_size()
    }
}

impl fmt::Display for GfhBlInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n    attr: {}", self.gfh_common, self.attr)
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::mtk_loaders::gfh_headers::{MtkGfhHeader, gfh_bl_info::GfhBlInfo};

    #[test]
    fn test_gfh_bl_info_parse() {
        let data = fs::read("./testbins/test.bin").unwrap();
        let gfh_bl_info = GfhBlInfo::load(&data, 0x38).unwrap();
        println!("{gfh_bl_info}");
    }
}
