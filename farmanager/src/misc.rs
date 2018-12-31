use log::*;

use crate::basic;
use crate::far_api;
use crate::ffi;
pub use crate::ffi::COLORDIALOGFLAGS as COLORDIALOGFLAGS;
pub use crate::ffi::rgba as rgba;

#[allow(unused_variables)]
pub fn show_color_chooser_dialog(flags: COLORDIALOGFLAGS) -> Option<ffi::FarColor> {
    trace!(">show_color_chooser_dialog()");
    let guid = &basic::plugin_guid();

    let result = far_api(|far_api: &mut ffi::PluginStartupInfo| {
        let mut color: ffi::FarColor;

        color = ffi::FarColor::default();
        let result;
        let return_code = far_api.color_dialog(guid, flags, &mut color);
        result = if return_code != 0 {
            Some(color)
        } else {
            None
        };
        return result;
    });

    trace!("<show_color_chooser_dialog()");
    return result;
}
