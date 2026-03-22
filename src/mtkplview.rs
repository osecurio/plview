use crate::mtkpl_loader::{MTKPL_MAGIC, MTKPreloaderParser};
use base64::prelude::*;
use binaryninja::{
    architecture::CoreArchitecture,
    binary_view::{BinaryView, BinaryViewBase, BinaryViewExt},
    custom_binary_view::{
        BinaryViewType, BinaryViewTypeBase, CustomBinaryView, CustomBinaryViewType,
    },
    data_buffer::DataBuffer,
    platform::Platform,
    section::Section,
    segment::{Segment, SegmentFlags},
};
use std::ops::Range;
use tracing::{debug, info, warn};

type BinaryViewResult<R> = binaryninja::binary_view::Result<R>;

pub struct MTKPreloaderBinaryViewType {
    view_type: BinaryViewType,
}

impl MTKPreloaderBinaryViewType {
    pub fn new(view_type: BinaryViewType) -> Self {
        Self { view_type }
    }
}

impl AsRef<BinaryViewType> for MTKPreloaderBinaryViewType {
    fn as_ref(&self) -> &BinaryViewType {
        &self.view_type
    }
}

impl BinaryViewTypeBase for MTKPreloaderBinaryViewType {
    fn is_deprecated(&self) -> bool {
        false
    }
    fn is_force_loadable(&self) -> bool {
        false
    }
    fn is_valid_for(&self, data: &BinaryView) -> bool {
        let mut magic = Vec::<u8>::new();

        let magic_b64 = BASE64_STANDARD.encode(MTKPL_MAGIC);
        let data_buf = DataBuffer::from_base64(magic_b64.as_str());
        let offset = if let Some(offset) = data.find_next_data(0x0, 0x1000, &data_buf) {
            offset
        } else {
            return false;
        };

        data.read_into_vec(&mut magic, offset, MTKPL_MAGIC.len());
        if magic == MTKPL_MAGIC {
            info!("Raw Preloader is valid.");
            return true;
        }
        warn!("Valid for failure!");
        false
    }
}

impl CustomBinaryViewType for MTKPreloaderBinaryViewType {
    fn create_custom_view<'builder>(
        &self,
        data: &BinaryView,
        builder: binaryninja::custom_binary_view::CustomViewBuilder<'builder, Self>,
    ) -> binaryninja::binary_view::Result<binaryninja::custom_binary_view::CustomView<'builder>>
    {
        info!("Creating MTKPreloaderBinaryView from MTKPreloaderBinaryViewType");

        let bv = builder.create::<MTKPreloaderBinaryView>(data, ());
        bv
    }
}

unsafe impl CustomBinaryView for MTKPreloaderBinaryView {
    type Args = ();

    fn new(handle: &BinaryView, _args: &Self::Args) -> binaryninja::binary_view::Result<Self> {
        MTKPreloaderBinaryView::new(handle)
    }

    fn init(&mut self, _args: Self::Args) -> binaryninja::binary_view::Result<()> {
        MTKPreloaderBinaryView::init(self)
    }
}

impl BinaryViewBase for MTKPreloaderBinaryView {
    fn address_size(&self) -> usize {
        4
    }

    fn default_endianness(&self) -> binaryninja::Endianness {
        binaryninja::Endianness::LittleEndian
    }

    fn entry_point(&self) -> u64 {
        self.get_entry_point()
    }
}

pub struct SegmentMappingData {
    mapped_addr_range: Range<u64>,
    file_backing: Range<u64>,
    mapped_segment_flags: SegmentFlags,
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

pub struct MTKPreloaderBinaryView {
    inner: binaryninja::rc::Ref<BinaryView>,
    mtkpl_parser: MTKPreloaderParser,
}

impl MTKPreloaderBinaryView {
    fn new(view: &BinaryView) -> BinaryViewResult<Self> {
        let parent_view = view.parent_view().ok_or(())?;
        let read_buffer = parent_view
            .read_buffer(0, parent_view.len() as usize)
            .ok_or(())?;
        Ok(Self {
            inner: view.to_owned(),
            mtkpl_parser: MTKPreloaderParser::new(read_buffer),
        })
    }

    fn init(&self) -> BinaryViewResult<()> {
        debug!("INIT");

        let default_arch = CoreArchitecture::by_name("thumb2").ok_or(())?;
        let default_platform = Platform::by_name("thumb2").ok_or(())?;
        self.set_default_arch(&default_arch);
        self.set_default_platform(&default_platform);

        let file_offset_to_pl_header = self.mtkpl_parser.get_file_backed_start_offset();

        //let segment_data = Vec::<SegmentMappingData>::new();

        // Load Base (Header)
        let load_addr = self.mtkpl_parser.get_image_load_addr() as u64;
        info!("Load Address: 0x{:X}", load_addr);
        let seg_flags = SegmentFlags::new()
            .readable(true)
            .contains_code(false)
            .contains_data(false)
            .deny_write(true)
            .executable(true);
        let header_segment = SegmentMappingData::new(
            Range {
                start: load_addr,
                end: load_addr + 0x300,
            },
            Range {
                start: file_offset_to_pl_header as u64,
                end: file_offset_to_pl_header as u64 + 0x300,
            },
            seg_flags,
        );

        let header_segment = Segment::builder(header_segment.mapped_addr_range.clone())
            .parent_backing(header_segment.file_backing.clone())
            .is_auto(true)
            .flags(header_segment.mapped_segment_flags);

        self.add_segment(header_segment);

        let header_section = Section::builder(
            "plhdr".to_string(),
            Range {
                start: load_addr,
                end: load_addr + 0x300 as u64,
            },
        )
        .is_auto(true);
        self.add_section(header_section);

        // Load Code & Data
        let entry_offset = self.mtkpl_parser.get_entry_point_offset() as u64;
        let entry_addr = entry_offset + load_addr;
        let preloader_size = self.mtkpl_parser.get_preloader_size() as usize;
        info!("Code & Data Address: 0x{:X}", entry_addr);

        // Segment Flags
        let seg_flags = SegmentFlags::new()
            .readable(true)
            .contains_code(true)
            .contains_data(true)
            .deny_write(false)
            .executable(true)
            .writable(true);

        let code_data_map_start = entry_addr;
        let code_data_map_end = (preloader_size as u64 - entry_offset) + entry_addr;
        let code_data_fb_start = file_offset_to_pl_header as u64 + entry_offset;
        let code_data_fb_end = (file_offset_to_pl_header + preloader_size) as u64;

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
            seg_flags,
        );

        // Build Segment
        let code_data_segment = Segment::builder(code_data_segment.mapped_addr_range.clone())
            .parent_backing(code_data_segment.file_backing.clone())
            .is_auto(true)
            .flags(code_data_segment.mapped_segment_flags);

        // Add Segment
        self.add_segment(code_data_segment);

        // Build Section
        let code_data_section = Section::builder(
            "code.data".to_string(),
            Range {
                start: entry_addr,
                end: entry_addr + preloader_size as u64 - entry_offset,
            },
        )
        .is_auto(true)
        .semantics(binaryninja::section::Semantics::ReadOnlyCode);

        // Add Section
        self.add_section(code_data_section);

        let entry_forced_platform = Platform::by_name("armv7").ok_or(())?;
        self.add_user_function_with_platform(entry_addr, &entry_forced_platform);

        Ok(())
    }

    fn get_entry_point(&self) -> u64 {
        self.mtkpl_parser.get_entry_point()
    }
}

impl AsRef<BinaryView> for MTKPreloaderBinaryView {
    fn as_ref(&self) -> &BinaryView {
        &self.inner
    }
}
