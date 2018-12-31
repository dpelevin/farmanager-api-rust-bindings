#[allow(unused_imports)] // till IntelliJ Rust stop to highlight is as an error
use std::panic;

use libc;
use log::*;

#[allow(unused_imports)] // TODO remove after IntelliJ Rust will stop to highlight is as an error
use crate::FarPlugin;
use crate::ffi;
#[allow(unused_imports)] // TODO remove after IntelliJ Rust will stop to highlight is as an error
use crate::plugin;

pub trait ExportFunctions {

    #[allow(unused_variables)]
    fn configure(&self, info: &ConfigureInfo) -> libc::intptr_t {
        trace!(">configure()");
        trace!("<configure()");
        return 0;
    }

}

pub struct ConfigureInfo {
    pub guid: ffi::GUID
}

#[allow(unused_variables)]
#[cfg(feature = "settings")]
#[no_mangle]
#[export_name="ConfigureW"]
pub extern "system" fn configure(info: *const ffi::ConfigureInfo) -> libc::intptr_t {
    trace!(">configure()");
    let call_result = panic::catch_unwind(|| {
        plugin(|plugin: &mut dyn FarPlugin| {
            match plugin.settings_exports() {
                Some(exports) => exports.configure(&ConfigureInfo { guid: unsafe { *(*info).guid }}),
                None => unimplemented!()
            }
        })
    });

    let r_val: libc::intptr_t = match call_result {
        Ok(v) => v,
        Err(_) => 0 as libc::intptr_t
    };
    trace!("<configure()");
    return r_val;
}

