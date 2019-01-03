#![warn(bare_trait_objects)]

use std::cell::RefCell;
use std::panic;
use std::result;

pub use failure::Error as Error;
use log::*;

pub use crate::ffi::FARMANAGERVERSION_BUILD as FARMANAGERVERSION_BUILD;
pub use crate::ffi::FARMANAGERVERSION_MAJOR as FARMANAGERVERSION_MAJOR;
pub use crate::ffi::FARMANAGERVERSION_MINOR as FARMANAGERVERSION_MINOR;
pub use crate::ffi::FARMANAGERVERSION_REVISION as FARMANAGERVERSION_REVISION;
pub use crate::ffi::FARMANAGERVERSION_STAGE as FARMANAGERVERSION_STAGE;
pub use crate::ffi::GUID as GUID;
pub use crate::ffi::HANDLE as HANDLE;

pub mod ffi;
mod common;
pub mod basic;
pub mod panel;
pub mod dialog;
pub mod editor;
pub mod macros;
pub mod misc;
pub mod plugin_manager;
pub mod settings;
pub mod viewer;

pub type WideString = common::string::WideString;
pub type Result<T> = result::Result<T, crate::Error>;

thread_local! {
    static FAR_PLUGIN: RefCell<Option<*mut dyn FarPlugin>> = RefCell::new(None);
    static FAR_API: RefCell<Option<*mut ffi::PluginStartupInfo>> = RefCell::new(None);
    static FAR_STANDARD_FUNCTIONS: RefCell<Option<*mut ffi::FarStandardFunctions>> = RefCell::new(None);
}

#[macro_export]
macro_rules! plugin {
    ( $t:ident ) => {
        #[no_mangle]
        #[export_name="GetGlobalInfoW"]
        pub extern "system" fn get_global_info(global_info: *mut farmanager::ffi::GlobalInfo) {
            farmanager::basic::get_global_info(Box::new($t::new()), global_info);
        }
    };
}

pub enum ReturnCode {
    Success = 1,
    UserCancel = -1
}

pub trait FarPlugin: panic::UnwindSafe {

    fn basic_exports(&mut self) -> &mut dyn basic::ExportFunctions;
    fn panel_exports(&mut self) -> Option<&mut dyn panel::ExportFunctions> {
        None
    }
    fn settings_exports(&mut self) -> Option<&mut dyn settings::ExportFunctions> {
        None
    }
}

fn init(plugin: Box<dyn FarPlugin>) {
    FAR_PLUGIN.with(|ref_cell: &RefCell<Option<*mut dyn FarPlugin>>| {
        ref_cell.replace(Some(Box::into_raw(plugin)));
    });
    basic::init_context();
    panel::init_context();
    panic::set_hook(Box::new(|info| {
        handle_panic(info.payload());
    }));
}

fn destroy() {
    panel::cleanup_context();
    basic::cleanup_context();
    FAR_STANDARD_FUNCTIONS.with(|ref_cell: &RefCell<Option<*mut ffi::FarStandardFunctions>>| {
        match ref_cell.replace(None) {
            Some(ptr) => {
                let _ = unsafe { Box::from_raw(ptr) };
            },
            None => ()
        }
    });
    FAR_API.with(|ref_cell: &RefCell<Option<*mut ffi::PluginStartupInfo>>| {
        match ref_cell.replace(None) {
            Some(ptr) => {
                let _ = unsafe { Box::from_raw(ptr) };
            },
            None => ()
        }
    });
    FAR_PLUGIN.with(|ref_cell: &RefCell<Option<*mut dyn FarPlugin>>| {
        match ref_cell.replace(None) {
            Some(ptr) => {
                let _ = unsafe { Box::from_raw(ptr) };
            },
            None => ()
        }
    });
}

fn plugin<F,R>(func: F) -> R where F: FnOnce(&mut dyn FarPlugin) -> R {
    FAR_PLUGIN.with(|ref_cell: &RefCell<Option<*mut dyn FarPlugin>>| {
        return match *ref_cell.borrow() {
            Some(plugin) => {
                func(unsafe { &mut *plugin })
            },
            None => panic!("Plugin is not initialized")
        };
    })
}

fn far_api<F,R>(func: F) -> R where F: FnOnce(&mut ffi::PluginStartupInfo) -> R {
    FAR_API.with(|ref_cell: &RefCell<Option<*mut ffi::PluginStartupInfo>>| {
        return match *ref_cell.borrow() {
            Some(far_api) => {
                func(unsafe { &mut *far_api })
            },
            None => panic!("Plugin is not initialized")
        };
    })
}

fn handle_panic(payload: &(dyn std::any::Any + Send)) {
    trace!(">handle_panic()");
    let line: String;

    match payload.downcast_ref::<&str>() {
        Some(text) => {
            line = text.to_string();
        },
        _ => {
            match payload.downcast_ref::<String>() {
                Some(text) => {
                    line = text.to_string();
                },
                None => {
                    line = "Oups! Something went wrong.".to_string();
                }
            }
        }
    }
    error!("{}", line);
    trace!("<handle_panic()");
}
