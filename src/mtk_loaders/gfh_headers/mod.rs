use tracing::{info, warn};

use crate::mtk_loaders::gfh_headers::{
    gfh_anti_clone::GfhAntiClone,
    gfh_bl_info::GfhBlInfo,
    gfh_bl_sec_key::GfhBlSecKey,
    gfh_brom_cfg::GfhBromCfg,
    gfh_brom_sec_cfg::GfhBromSecCfg,
    gfh_common::{GfhCommonHeader, GfhHeaderType},
    gfh_file_info::GfhFileInfo,
};

pub(crate) mod gfh_anti_clone;
pub(crate) mod gfh_bl_info;
pub(crate) mod gfh_bl_sec_key;
pub(crate) mod gfh_brom_cfg;
pub(crate) mod gfh_brom_sec_cfg;
pub(crate) mod gfh_common;
pub(crate) mod gfh_file_info;
pub(crate) mod gfh_types;

pub trait MtkGfhHeader {
    type Header;
    fn load(data: &[u8], offset: usize) -> Option<Self::Header>;
    fn header_size(&self) -> usize;
}

#[derive(Default, Debug, Clone)]
pub struct GfhHeader {
    gfh_file_info: Option<gfh_file_info::GfhFileInfo>,
    gfh_bl_info: Option<gfh_bl_info::GfhBlInfo>,
    gfh_brom_cfg: Option<gfh_brom_cfg::GfhBromCfg>,
    gfh_bl_sec_key: Option<gfh_bl_sec_key::GfhBlSecKey>,
    gfh_anti_clone: Option<gfh_anti_clone::GfhAntiClone>,
    gfh_brom_sec_cfg: Option<gfh_brom_sec_cfg::GfhBromSecCfg>,
}

impl GfhHeader {
    pub fn get_gfh_file_info(&self) -> Option<gfh_file_info::GfhFileInfo> {
        self.gfh_file_info
    }

    pub fn get_gfh_bl_info(&self) -> Option<gfh_bl_info::GfhBlInfo> {
        self.gfh_bl_info
    }

    pub fn get_gfh_brom_cfg(&self) -> Option<gfh_brom_cfg::GfhBromCfg> {
        self.gfh_brom_cfg
    }

    pub fn get_gfh_bl_sec_key(&self) -> Option<gfh_bl_sec_key::GfhBlSecKey> {
        self.gfh_bl_sec_key
    }

    pub fn get_gfh_anti_clone(&self) -> Option<gfh_anti_clone::GfhAntiClone> {
        self.gfh_anti_clone
    }

    pub fn get_gfh_brom_sec_cfg(&self) -> Option<gfh_brom_sec_cfg::GfhBromSecCfg> {
        self.gfh_brom_sec_cfg
    }
}

impl MtkGfhHeader for GfhHeader {
    type Header = GfhHeader;
    fn load(data: &[u8], mut offset: usize) -> Option<Self::Header> {
        let mut gfh_header = Self::default();

        let gfh_file_info = GfhFileInfo::load(&data, offset)?;
        let gfh_hdr_size = gfh_file_info.get_gfh_header_total_size() as usize;

        println!("Being parse loop..");

        loop {
            let Some(header_type) = GfhCommonHeader::load(data, offset) else {
                // No more headers to parse
                println!("No more headers!");
                break;
            };

            match header_type.get_type() {
                GfhHeaderType::GFH_TYPE_FILE_INFO => {
                    println!("Trying {}", header_type.get_type());
                    if let Some(header) = GfhFileInfo::load(data, offset) {
                        offset += header.header_size();
                        if gfh_header.gfh_file_info.is_none() {
                            gfh_header.gfh_file_info = Some(header);
                        } else {
                            warn!(
                                "Already loaded a {} header... skipping..",
                                header_type.get_type()
                            );
                        }
                    }
                }
                GfhHeaderType::GFH_TYPE_BL_INFO => {
                    println!("Trying {}", header_type.get_type());
                    if let Some(header) = GfhBlInfo::load(data, offset) {
                        offset += header.header_size();
                        if gfh_header.gfh_bl_info.is_none() {
                            gfh_header.gfh_bl_info = Some(header);
                        } else {
                            warn!(
                                "Already loaded a {} header... skipping..",
                                header_type.get_type()
                            );
                        }
                    }
                }
                GfhHeaderType::GFH_TYPE_BROM_CFG => {
                    println!("Trying {}", header_type.get_type());
                    if let Some(header) = GfhBromCfg::load(data, offset) {
                        println!("BROM Success");
                        offset += header.header_size();
                        if gfh_header.gfh_brom_cfg.is_none() {
                            gfh_header.gfh_brom_cfg = Some(header);
                        } else {
                            warn!(
                                "Already loaded a {} header... skipping..",
                                header_type.get_type()
                            );
                        }
                    }
                }
                GfhHeaderType::GFH_TYPE_BL_SEC_KEY => {
                    println!("Trying {}", header_type.get_type());
                    if let Some(header) = GfhBlSecKey::load(data, offset) {
                        offset += header.header_size();
                        if gfh_header.gfh_bl_sec_key.is_none() {
                            gfh_header.gfh_bl_sec_key = Some(header);
                        } else {
                            warn!(
                                "Already loaded a {} header... skipping..",
                                header_type.get_type()
                            );
                        }
                    }
                }
                GfhHeaderType::GFH_TYPE_ANTI_CLONE => {
                    println!("Trying {}", header_type.get_type());
                    if let Some(header) = GfhAntiClone::load(data, offset) {
                        offset += header.header_size();
                        if gfh_header.gfh_anti_clone.is_none() {
                            gfh_header.gfh_anti_clone = Some(header);
                        } else {
                            warn!(
                                "Already loaded a {} header... skipping..",
                                header_type.get_type()
                            );
                        }
                    }
                }
                GfhHeaderType::GFH_TYPE_BROM_SEC_CFG => {
                    println!("Trying {}", header_type.get_type());
                    if let Some(header) = GfhBromSecCfg::load(data, offset) {
                        offset += header.header_size();
                        if gfh_header.gfh_brom_sec_cfg.is_none() {
                            gfh_header.gfh_brom_sec_cfg = Some(header);
                        } else {
                            warn!(
                                "Already loaded a {} header... skipping..",
                                header_type.get_type()
                            );
                        }
                    }
                }
                GfhHeaderType::Unknown => {
                    println!("Trying {}", header_type.get_type());
                    // Ruh Roh Raggy
                    println!("Got GfhHeaderType::Unknown.. something is probably wrong..");
                }
            }

            if gfh_hdr_size as usize == offset {
                println!("GFH header size offset reached");
                break;
            } else if offset > gfh_hdr_size {
                println!("offset > gfh_hdr_size... something went wrong..");
                break;
            }
        }

        Some(gfh_header)
    }
    fn header_size(&self) -> usize {
        self.gfh_file_info
            .as_ref()
            .unwrap()
            .get_gfh_header_total_size() as usize
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::mtk_loaders::gfh_headers::{GfhHeader, MtkGfhHeader};

    #[test]
    fn test_gfh_header_parse() {
        let data = fs::read("./testbins/test.bin").unwrap();
        let gfh = GfhHeader::load(&data, 0).unwrap();
        println!("{gfh:X?}");
    }
}
