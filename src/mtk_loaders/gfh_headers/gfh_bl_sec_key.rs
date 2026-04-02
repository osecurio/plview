use crate::mtk_loaders::gfh_headers::{MtkGfhHeader, gfh_common::GfhCommonHeader};

#[derive(Debug, Clone, Copy)]
pub struct GfhBlSecKey {
    gfh_type_offset: u32,
    gfh_common: GfhCommonHeader,
    pad: [u8; 0x20c],
}

impl GfhBlSecKey {
    pub fn get_header_offset(&self) -> u32 {
        self.gfh_type_offset
    }
}

impl MtkGfhHeader for GfhBlSecKey {
    type Header = GfhBlSecKey;
    fn load(data: &[u8], mut offset: usize) -> Option<Self::Header> {
        let gfh_type_offset = offset as u32;
        let gfh_common = GfhCommonHeader::load(data, offset)?;
        offset += gfh_common.get_size() as usize;

        let pad = *data[offset..offset + 0x20c].as_array().unwrap();

        Some(Self { gfh_type_offset, gfh_common, pad })
    }

    fn header_size(&self) -> usize {
        self.gfh_common.header_size()
    }
}
