use std::{collections::HashMap, fmt, ops::Range};

use binaryninja::{data_buffer::DataBuffer, segment::SegmentFlags};
use tracing::debug;


// EMI (MediaTek External Memory Interface)
//pub const EMMC_BOOT_MTKPL_OFFSET: u64 = 0x800;
//pub const EMMC_BOOT_MAGIC: &'static [u8; 9] = b"EMMC_BOOT";

pub const MTKPL_MAGIC: &'static [u8; 8] = b"\x4d\x4d\x4d\x01\x38\x00\x00\x00";
pub const MTKPL_LOAD_ADDRESS_OFFSET: u64 = 0x1C;
pub const MTKPL_ENTRY_POINT_OFFSET: u64 = 0x28;
pub const MTKPL_BIN_SIZE_OFFSET: u64 = 0x20;
pub const MTKPL_ADDRESS_WIDTH: u64 = 0x4;

#[derive(Clone)]
pub struct SegmentMappingData {
    pub mapped_addr_range: Range<u64>,
    pub mapped_segment_flags: SegmentFlags,
    pub file_backing: Range<u64>,
}

impl SegmentMappingData {
    fn new(
        mapped_addr_range: Range<u64>,
        file_backing: Range<u64>,
        mapped_segment_flags: SegmentFlags,
    ) -> Self {
        Self {
            mapped_addr_range,
            file_backing,
            mapped_segment_flags,
        }
    }
}

#[derive(Clone)]
pub struct SectionData {
    pub name: String,
    pub mapped_addr_range: Range<u64>
}

impl SectionData {
    fn new(name: &str, mapped_addr_range: Range<u64>) -> Self {
        Self { name: name.to_string(), mapped_addr_range }
    }
}

pub struct MTKPreloaderParser {
    image_data: Vec<u8>,
    drained_data: Vec<u8>,
    segment_data: HashMap<String, SegmentMappingData>,
    section_data: HashMap<String, SectionData>,
}

impl MTKPreloaderParser {
    pub fn new(data: DataBuffer) -> Self {
        let mut image_data = data.get_data().to_vec();
        let offset = if let Some(offset) =
            MTKPreloaderParser::find_byte_seq_offset(&image_data, MTKPL_MAGIC)
        {
            offset
        } else {
            0
        };
        let drained_data = image_data.drain(0..offset).as_slice().to_vec();
        debug!(
            "Drained {} bytes.. First 4 bytes are now: {:?}",
            drained_data.len(),
            &image_data.as_slice()[0..4]
        );

        let mut parser = Self {
            image_data,
            drained_data,
            segment_data: HashMap::new(),
            section_data: HashMap::new(),
        };

        let file_offset_to_pl_header = Self::get_file_backed_start_offset(&parser);
        let load_addr = Self::get_image_load_addr(&parser) as u64;
        let entry_offset = Self::get_entry_point_offset(&parser) as u64;
        let entry_addr = entry_offset + load_addr;
        let preloader_size = Self::get_preloader_size(&parser) as usize;
        let header_size = entry_addr - load_addr;
        debug!("header_size: {header_size:X} = {entry_addr:X} - {load_addr:X}");
        let code_data_map_start = entry_addr;
        let code_data_map_end = (preloader_size as u64 - entry_offset) + entry_addr;
        let code_data_fb_start = file_offset_to_pl_header as u64 + entry_offset;
        let code_data_fb_end = (file_offset_to_pl_header + preloader_size) as u64;

        let header_seg_flags = SegmentFlags::new()
            .readable(true)
            .contains_code(false)
            .contains_data(false)
            .deny_write(false)
            .executable(true);

        let header_segment = SegmentMappingData::new(
            Range {
                start: load_addr,
                end: load_addr + header_size,
            },
            Range {
                start: file_offset_to_pl_header as u64,
                end: file_offset_to_pl_header as u64 + header_size,
            },
            header_seg_flags,
        );

        let header_section = SectionData::new(".plhdr", Range { start: load_addr, end: load_addr + header_size as u64 });

        // Segment Flags
        let code_data_seg_flags = SegmentFlags::new()
            .readable(true)
            .contains_code(true)
            .contains_data(true)
            .deny_write(false)
            .executable(true)
            .writable(true);

        // Segment Mapping Data
        let code_data_segment = SegmentMappingData::new(
            Range {
                start: code_data_map_start,
                end: code_data_map_end,
            },
            Range {
                start: code_data_fb_start,
                end: code_data_fb_end,
            },
            code_data_seg_flags,
        );

        let code_data_section = SectionData::new(".code.data", Range { start: entry_addr, end: entry_addr + preloader_size as u64 - entry_offset });

        parser.segment_data.insert(".plhdr".to_string(),header_segment);
        parser.segment_data.insert(".code.data".to_string(), code_data_segment);
        parser.section_data.insert(".plhdr".to_string(), header_section);
        parser.section_data.insert(".code.data".to_string(), code_data_section);

        parser
    }

    pub fn get_image_load_addr(&self) -> u32 {
        let load_addr = self.image_data[MTKPL_LOAD_ADDRESS_OFFSET as usize
            ..(MTKPL_LOAD_ADDRESS_OFFSET + MTKPL_ADDRESS_WIDTH) as usize]
            .to_owned();
        let la = load_addr
            .try_into()
            .unwrap_or_else(|_| [0; MTKPL_ADDRESS_WIDTH as usize]);
        u32::from_le_bytes(la)
    }

    pub fn get_entry_point_offset(&self) -> u32 {
        let entry_point = self.image_data[MTKPL_ENTRY_POINT_OFFSET as usize
            ..(MTKPL_ENTRY_POINT_OFFSET + MTKPL_ADDRESS_WIDTH) as usize]
            .to_owned();
        let ep = entry_point
            .try_into()
            .unwrap_or_else(|_| [0; MTKPL_ADDRESS_WIDTH as usize]);
        u32::from_le_bytes(ep)
    }

    pub fn get_entry_point(&self) -> u64 {
        return (self.get_image_load_addr() + self.get_entry_point_offset()) as u64;
    }

    pub fn get_preloader_size(&self) -> u32 {
        let size = self.image_data[MTKPL_BIN_SIZE_OFFSET as usize
            ..(MTKPL_BIN_SIZE_OFFSET + MTKPL_ADDRESS_WIDTH) as usize]
            .to_owned();
        let sz = size
            .try_into()
            .unwrap_or_else(|_| [0; MTKPL_ADDRESS_WIDTH as usize]);
        u32::from_le_bytes(sz)
    }

    pub fn get_file_backed_start_offset(&self) -> usize {
        self.drained_data.len()
    }

    fn find_byte_seq_offset(hs: &[u8], needle: &[u8]) -> Option<usize> {
        // Should I add a length delimiter parameter???
        hs.windows(needle.len()).position(|w| w == needle)
    }

    pub fn get_segments(&self) -> HashMap<String, SegmentMappingData> {
        self.segment_data.clone()
    }

    pub fn get_sections(&self) -> HashMap<String, SectionData> {
        self.section_data.clone()
    }
}

impl fmt::Display for MTKPreloaderParser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = format!("Loaded Preloader Data:\n");
        if let Some(hs) = self.segment_data.get(".plhdr") {
            s = format!("{s}m .plhdr -> 0x{:X} - 0x{:X}\n",hs.mapped_addr_range.start, hs.mapped_addr_range.end);
            s = format!("{s}f .plhdr -> 0x{:X} - 0x{:X}\n",hs.file_backing.start, hs.file_backing.end);
        };

        if let Some(cds) = self.segment_data.get(".code.data") {
            s = format!("{s}m .code.data -> 0x{:X} - 0x{:X}\n",cds.mapped_addr_range.start, cds.mapped_addr_range.end);
            s = format!("{s}f .code.data -> 0x{:X} - 0x{:X}\n",cds.file_backing.start, cds.file_backing.end);
        }
        write!(f, "{s}")
    }
}