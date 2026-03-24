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
    segment::Segment, symbol::{Symbol, SymbolType},
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
            debug!("Raw Preloader is valid.");
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
        debug!("Creating MTKPreloaderBinaryView from MTKPreloaderBinaryViewType");

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

        info!("{}", self.mtkpl_parser);

        for (_name, segment) in self.mtkpl_parser.get_segments() {

            let new_segment = Segment::builder(segment.mapped_addr_range.clone())
                .parent_backing(segment.file_backing.clone())
                .is_auto(true)
                .flags(segment.mapped_segment_flags);

            self.add_segment(new_segment);
        }

        for (name, section) in self.mtkpl_parser.get_sections() {
            let mut new_section = Section::builder(
                section.name.clone(),
                Range {
                    start: section.mapped_addr_range.start,
                    end: section.mapped_addr_range.end,
                },
            )
            .is_auto(true);

            if name == ".code.data" {
                new_section = new_section.semantics(binaryninja::section::Semantics::ReadOnlyCode);
            }

            self.add_section(new_section);
        }

        let entry_forced_platform = Platform::by_name("armv7").ok_or(())?;
        let entry_point = self.get_entry_point();
        let start_symbol = Symbol::builder(SymbolType::Function, "_start", entry_point).full_name("_start").short_name("_start").create();
        self.add_entry_point_with_platform(entry_point, &entry_forced_platform);
        self.define_user_symbol(&start_symbol);

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
