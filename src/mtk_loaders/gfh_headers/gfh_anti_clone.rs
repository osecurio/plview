use crate::mtk_loaders::gfh_headers::{MtkGfhHeader, gfh_common::GfhCommonHeader};

#[derive(Debug, Clone, Copy)]
pub struct GfhAntiClone {
    gfh_common: GfhCommonHeader,
    ac_b2k: u8,
    ac_b2c: u8,
    pad: u16,
    ac_offset: u32,
    ac_len: u32,
}

impl MtkGfhHeader for GfhAntiClone {
    type Header = GfhAntiClone;
    fn load(data: &[u8], mut offset: usize) -> Option<Self::Header> {
        let gfh_common = GfhCommonHeader::load(data, offset)?;
        offset += gfh_common.get_size() as usize;

        let acbk = *data[offset..offset + size_of::<u8>()]
            .as_array()
            .unwrap();
        let ac_b2k = u8::from_le_bytes(acbk);
        offset += 1;

        let acbc = *data[offset..offset + size_of::<u8>()]
            .as_array()
            .unwrap();
        let ac_b2c = u8::from_le_bytes(acbc);
        offset += 1;

        let p = *data[offset..offset + size_of::<u16>()]
            .as_array()
            .unwrap();
        let pad = u16::from_le_bytes(p);
        offset += 2;

        let aco = *data[offset..offset + size_of::<u32>()]
            .as_array()
            .unwrap();
        let ac_offset = u32::from_le_bytes(aco);
        offset += 4;

        let acl = *data[offset..offset + size_of::<u32>()]
            .as_array()
            .unwrap();
        let ac_len = u32::from_le_bytes(acl);

        Some(Self {
            gfh_common,
            ac_b2k,
            ac_b2c,
            pad,
            ac_offset,
            ac_len,
        })
    }

    fn header_size(&self) -> usize {
        self.gfh_common.header_size()
    }
}
