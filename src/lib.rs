use binaryninja::{
    binary_view::{BinaryViewBase, BinaryViewExt},
    command::{Command, register_command},
    custom_binary_view::register_view_type,
};
use tracing::{debug, info};

mod mtkpl_loader;
mod mtkplview;

struct LoadCommand;

impl Command for LoadCommand {
    fn action(&self, view: &binaryninja::binary_view::BinaryView) {
        let Some(pv) = view.parent_view() else {
            info!("Failed to get parent view..");
            return;
        };
        let Some(buf) = pv.read_buffer(0, pv.len() as usize) else {
            info!("Failed to get read buffer..");
            return;
        };
        let pl = mtkpl_loader::MTKPreloaderParser::new(buf);
        info!("{pl}");
    }
    fn valid(&self, _view: &binaryninja::binary_view::BinaryView) -> bool {
        true
    }
}

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "C" fn CorePluginInit() -> bool {
    binaryninja::tracing_init!("mtkpl");
    debug!("Preloader View initializing..");

    register_view_type(
        "mtkpl",
        "MTK Preloader",
        mtkplview::MTKPreloaderBinaryViewType::new,
    );

    register_command(
        "plview\\Print Load Information",
        "Prints load information for the current file.",
        LoadCommand,
    );

    debug!("Preloader View Initialized.");

    true
}
