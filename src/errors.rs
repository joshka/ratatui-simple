//! This module contains the error handling and panic hooks for the application.
//!
//! The error handling and panic hooks are set up in the `init` function. This function should be
//! called at the start of the application to set up the error handling and panic hooks.
use std::panic;

use color_eyre::{
    config::{EyreHook, HookBuilder, PanicHook},
    eyre, Result,
};

use crate::tui;

/// This replaces the standard color_eyre panic and error hooks with hooks that restore the terminal
/// before printing the panic or error.
pub fn init() -> Result<()> {
    // add any extra configuration you need to the hook builder
    let hook_builder = HookBuilder::default();
    let (panic_hook, eyre_hook) = hook_builder.into_hooks();
    install_panic_hook(panic_hook);
    install_eyre_hook(eyre_hook)?;
    Ok(())
}

fn install_panic_hook(panic_hook: PanicHook) {
    // convert from a `color_eyre::config::PanicHook`` to a `Box<dyn Fn(&PanicInfo<'_>`
    let panic_hook = panic_hook.into_panic_hook();
    panic::set_hook(Box::new(move |panic_info| {
        tui::restore().unwrap();
        panic_hook(panic_info);
    }));
}

fn install_eyre_hook(eyre_hook: EyreHook) -> color_eyre::Result<()> {
    let eyre_hook = eyre_hook.into_eyre_hook();
    eyre::set_hook(Box::new(move |error| {
        tui::restore().unwrap();
        eyre_hook(error)
    }))?;
    Ok(())
}
