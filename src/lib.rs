use binaryninja::custom_binary_view::register_view_type;
use tracing::debug;

mod mtkpl_loader;
mod mtkplview;

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

    debug!("Preloader View Initialized.");

    true
}
