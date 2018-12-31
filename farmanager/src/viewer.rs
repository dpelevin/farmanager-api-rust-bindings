use log::*;

use crate::common::string::WideString;
use crate::far_api;
use crate::ffi;

// TODO review and update
pub fn open_viewer(path: String) {
    trace!(">open_viewer()");
    far_api(|far_api: &mut ffi::PluginStartupInfo| {
        let file_name = WideString::from(path);
        far_api.viewer(file_name.as_ptr(), 0 as *const ffi::wchar_t, 0, 0, -1, -1, ffi::VIEWER_FLAGS::VF_NONE, ffi::CP_DEFAULT);
    });
    trace!("<open_viewer()");
}
