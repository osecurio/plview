use crate::mtk_loaders::preloader::gfh_headers::{MtkGfhHeader, gfh_common::GfhCommonHeader};

#[derive(Debug, Clone, Copy)]
pub struct GfhBromSecCfg {
    gfh_type_offset: u32,
    gfh_common: GfhCommonHeader,
    cfg_bits: u32,
    customer_name: [u8; 0x20],
    pad: u32,
}

impl GfhBromSecCfg {
    pub fn get_header_offset(&self) -> u32 {
        self.gfh_type_offset
    }
}

impl MtkGfhHeader for GfhBromSecCfg {
    type Header = GfhBromSecCfg;
    fn load(data: &[u8], mut offset: usize) -> Option<Self::Header> {
        let gfh_type_offset = offset as u32;
        let gfh_common = GfhCommonHeader::load(data, offset)?;
        offset += gfh_common.get_size() as usize;

        let cfgb = *data[offset..offset + size_of::<u32>()].as_array().unwrap();
        let cfg_bits = u32::from_le_bytes(cfgb);
        offset += 4;

        let customer_name = *data[offset..offset + 0x20].as_array().unwrap();
        offset += 0x20;

        let p = *data[offset..offset + size_of::<u32>()].as_array().unwrap();
        let pad = u32::from_le_bytes(p);

        Some(Self {
            gfh_type_offset,
            gfh_common,
            cfg_bits,
            customer_name,
            pad,
        })
    }

    fn header_size(&self) -> usize {
        self.gfh_common.header_size()
    }
}
