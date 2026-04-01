use crate::mtk_loaders::gfh_headers::{MtkGfhHeader, gfh_common::GfhCommonHeader};

#[derive(Debug, Clone, Copy)]
pub struct GfhBromCfg {
    gfh_common: GfhCommonHeader,
    cfg_bits: u32,
    usbdl_by_auto_detect_timeout_ms: u32,
    unused: [u8; 0x45],
    jump_bl_arm64: u8,
    unused2: [u8; 0x2],
    usbdl_by_kcol0_timeout_ms: u32,
    usbdl_by_flag_timeout_ms: u32,
    pad: u32,
}

impl MtkGfhHeader for GfhBromCfg {
    type Header = GfhBromCfg;
    fn load(data: &[u8], mut offset: usize) -> Option<Self::Header> {
        let gfh_common = GfhCommonHeader::load(data, offset)?;
        offset += gfh_common.get_size() as usize;

        let cfgb = *data[offset..offset + size_of::<u32>()].as_array().unwrap();
        let cfg_bits = u32::from_le_bytes(cfgb);
        offset += 4;

        let usbdl_adtms = *data[offset..offset + size_of::<u32>()].as_array().unwrap();
        let usbdl_by_auto_detect_timeout_ms = u32::from_le_bytes(usbdl_adtms);
        offset += 4;

        let unused = *data[offset..offset + 0x45].as_array().unwrap();
        offset += 0x45;

        let jba64 = *data[offset..offset + size_of::<u8>()].as_array().unwrap();
        let jump_bl_arm64 = u8::from_le_bytes(jba64);
        offset += 1;

        let unused2 = *data[offset..offset + 0x2].as_array().unwrap();
        offset += 0x2;

        let usdbdl_ktms = *data[offset..offset + size_of::<u32>()].as_array().unwrap();
        let usbdl_by_kcol0_timeout_ms = u32::from_le_bytes(usdbdl_ktms);
        offset += 4;

        let usbdl_ftms = *data[offset..offset + size_of::<u32>()].as_array().unwrap();
        let usbdl_by_flag_timeout_ms = u32::from_le_bytes(usbdl_ftms);
        offset += 4;

        let p = *data[offset..offset + size_of::<u32>()].as_array().unwrap();
        let pad = u32::from_le_bytes(p);

        Some(Self {
            gfh_common,
            cfg_bits,
            usbdl_by_auto_detect_timeout_ms,
            unused,
            jump_bl_arm64,
            unused2,
            usbdl_by_kcol0_timeout_ms,
            usbdl_by_flag_timeout_ms,
            pad,
        })
    }

    fn header_size(&self) -> usize {
        self.gfh_common.header_size()
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::mtk_loaders::gfh_headers::{MtkGfhHeader, gfh_brom_cfg::GfhBromCfg};

    #[test]
    fn test_gfh_brom_cfg_parse() {
        let data = fs::read("./testbins/test.bin").unwrap();
        let gbc = GfhBromCfg::load(&data, 0x44).unwrap();
        println!("{gbc:#X?}");
    }
}
