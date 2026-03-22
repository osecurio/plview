use binaryninja::data_buffer::DataBuffer;
use tracing::debug;
//use tracing::{info, warn};

// EMI (MediaTek External Memory Interface)
//pub const EMMC_BOOT_MTKPL_OFFSET: u64 = 0x800;
//pub const EMMC_BOOT_MAGIC: &'static [u8; 9] = b"EMMC_BOOT";

pub const MTKPL_MAGIC: &'static [u8; 8] = b"\x4d\x4d\x4d\x01\x38\x00\x00\x00";
pub const MTKPL_LOAD_ADDRESS_OFFSET: u64 = 0x1C;
pub const MTKPL_ENTRY_POINT_OFFSET: u64 = 0x28;
pub const MTKPL_BIN_SIZE_OFFSET: u64 = 0x20;
pub const MTKPL_ADDRESS_WIDTH: u64 = 0x4;

pub struct MTKPreloaderParser {
    image_data: Vec<u8>,
    drained_data: Vec<u8>,
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
        Self {
            image_data,
            drained_data,
        }
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
}
