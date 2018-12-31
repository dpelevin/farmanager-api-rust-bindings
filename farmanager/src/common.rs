use log::*;
use winapi::shared::winerror;
use winapi::um::combaseapi::CoCreateGuid;

use crate::ffi::GUID;

pub mod ffi;
pub mod string;

pub trait Enrichable<C, S> {
    fn enrich(&mut self, ctx: &mut C, source: S);
}

pub trait Cleanable<C> {
    fn cleanup(&mut self, _ctx: &mut C) {}
}

pub fn generate_guid() -> GUID {
    trace!(">generate_guid()");
    let mut result: GUID = GUID {
        Data1: 0x0,
        Data2: 0x0,
        Data3: 0x0,
        Data4: [0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x01]
    };

    if unsafe { CoCreateGuid(&mut result as *mut GUID) } != winerror::S_OK {
        panic!("Fail to generate a GUID")
    }
    trace!("<generate_guid()");
    return result;
}
