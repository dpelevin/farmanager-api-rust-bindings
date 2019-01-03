use log::*;

use crate::common::string::WideString;
use crate::far_api;
use crate::ffi;

// TODO review and update
pub fn open_editor(path: WideString) {
    trace!(">open_editor()");
    far_api(|far_api: &mut ffi::PluginStartupInfo| {
        let file_name = path;
        far_api.editor(file_name.as_ptr(), 0 as *const ffi::wchar_t, 0, 0, -1, -1, ffi::EDITOR_FLAGS::EN_NONE, 1, 1, ffi::CP_DEFAULT);
    });
    trace!("<open_editor()");
}
