use core::fmt;

use crate::mtk_loaders::gfh_headers::{MtkGfhHeader, gfh_common::GfhCommonHeader};

macro_rules! gfh_file_info_fmt {
    () => {
        r#"
GfhFileInfo {{
{}
    name: {:?}
    unused: 0x{:X}
    file_type: 0x{:X}
    flash_type: 0x{:X}
    sig_type: 0x{:X}
    load_addr: 0x{:X}
    total_size: 0x{:X}
    max_size: 0x{:X}
    hdr_size: 0x{:X}
    sig_size: 0x{:X}
    jump_offset: 0x{:X}
    processed: 0x{:X}
}}
"#
    };
}

#[derive(Debug, Clone, Copy)]
pub struct GfhFileInfo {
    gfh_common: GfhCommonHeader,
    name: [u8; 12],
    unused: u32,
    file_type: u16,
    flash_type: u8,
    sig_type: u8,
    load_addr: u32,
    total_size: u32,
    max_size: u32,
    hdr_size: u32,
    sig_size: u32,
    jump_offset: u32,
    processed: u32,
}

impl GfhFileInfo {
    pub fn get_gfh_header_total_size(&self) -> u32 {
        self.hdr_size
    }

    pub fn get_load_addr(&self) -> u32 {
        self.load_addr
    }

    pub fn get_jump_offset(&self) -> u32 {
        self.jump_offset
    }

    pub fn get_total_size(&self) -> u32 {
        self.total_size
    }

    pub fn get_hdr_size(&self) -> u32 {
        self.hdr_size
    }

    pub fn get_signature_size(&self) -> u32 {
        self.sig_size
    }

    pub fn get_hdr_full_size(&self) -> u32 {
        self.hdr_size
    }
}

impl MtkGfhHeader for GfhFileInfo {
    type Header = GfhFileInfo;
    fn load(data: &[u8], mut offset: usize) -> Option<Self::Header> {
        let gfh_common = GfhCommonHeader::load(&data, offset)?;
        offset = gfh_common.get_size() as usize;

        let name = *data[offset..offset + 12].as_array().unwrap();
        offset += 12;

        let u = *data[offset..offset + size_of::<u32>()]
            .as_array()
            .unwrap();
        let unused = u32::from_le_bytes(u);
        offset += 4;

        let file_t = *data[offset..offset + size_of::<u16>()]
            .as_array()
            .unwrap();
        let file_type = u16::from_le_bytes(file_t);
        offset += 2;

        let flash_t = *data[offset..offset + size_of::<u8>()]
            .as_array()
            .unwrap();
        let flash_type = u8::from_le_bytes(flash_t);
        offset += 1;

        let sig_t = *data[offset..offset + size_of::<u8>()]
            .as_array()
            .unwrap();
        let sig_type = u8::from_le_bytes(sig_t);
        offset += 1;

        let load_a = *data[offset..offset + size_of::<u32>()]
            .as_array()
            .unwrap();
        let load_addr = u32::from_le_bytes(load_a);
        offset += 4;

        let total_sz = *data[offset..offset + size_of::<u32>()]
            .as_array()
            .unwrap();
        let total_size = u32::from_le_bytes(total_sz);
        offset += 4;

        let max_sz = *data[offset..offset + size_of::<u32>()]
            .as_array()
            .unwrap();
        let max_size = u32::from_le_bytes(max_sz);
        offset += 4;

        let hdr_sz = *data[offset..offset + size_of::<u32>()]
            .as_array()
            .unwrap();
        let hdr_size = u32::from_le_bytes(hdr_sz);
        offset += 4;

        let sig_sz = *data[offset..offset + size_of::<u32>()]
            .as_array()
            .unwrap();
        let sig_size = u32::from_le_bytes(sig_sz);
        offset += 4;

        let jo = *data[offset..offset + size_of::<u32>()]
            .as_array()
            .unwrap();
        let jump_offset = u32::from_le_bytes(jo);
        offset += 4;

        let procd = *data[offset..offset + size_of::<u32>()]
            .as_array()
            .unwrap();
        let processed = u32::from_le_bytes(procd);

        Some(Self {
            gfh_common,
            name,
            unused,
            file_type,
            flash_type,
            sig_type,
            load_addr,
            total_size,
            max_size,
            hdr_size,
            sig_size,
            jump_offset,
            processed,
        })
    }

    fn header_size(&self) -> usize {
        self.gfh_common.header_size() as usize
    }
}

impl fmt::Display for GfhFileInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = unsafe { String::from_utf8_unchecked(self.name.to_vec()) };
        write!(
            f,
            gfh_file_info_fmt!(),
            self.gfh_common,
            name,
            self.unused,
            self.file_type,
            self.flash_type,
            self.sig_type,
            self.load_addr,
            self.total_size,
            self.max_size,
            self.hdr_size,
            self.sig_size,
            self.jump_offset,
            self.processed
        )
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::mtk_loaders::gfh_headers::{MtkGfhHeader, gfh_file_info::GfhFileInfo};

    #[test]
    fn test_gfh_file_info_parse() {
        let data = fs::read("./testbins/test.bin").unwrap();
        let gfi = GfhFileInfo::load(&data, 0).unwrap();
        println!("{gfi}");
    }
}
