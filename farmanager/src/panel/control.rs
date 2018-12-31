use std::ptr;

use failure::*;
use log::*;

use crate::far_api;
use crate::ffi;

use super::*;

pub fn check_panels_exist() -> bool {
    trace!(">check_panels_exist");
    let result = far_api(|far_api: &mut ffi::PluginStartupInfo| {
        let param1: libc::intptr_t = 0;
        let param2: *mut libc::c_void = ptr::null_mut();
        let result = far_api.panel_control(Panel::None.into(),
                                           ffi::FILE_CONTROL_COMMANDS::FCTL_CHECKPANELSEXIST,
                                           param1,
                                           param2);
        match result {
            0 => false,
            _ => true
        }
    });
    trace!("<check_panels_exist");
    return result;
}

pub fn is_active_panel(panel: Panel) -> bool {
    trace!(">is_active_panel");
    let result = far_api(|far_api: &mut ffi::PluginStartupInfo| {
        let h_panel: ffi::HANDLE = panel.into();
        let param1: libc::intptr_t = 0;
        let param2: *mut libc::c_void = ptr::null_mut();
        let result = far_api.panel_control(h_panel,
                                           ffi::FILE_CONTROL_COMMANDS::FCTL_ISACTIVEPANEL,
                                           param1,
                                           param2);
        match result {
            0 => false,
            _ => true
        }
    });
    trace!("<is_active_panel");
    return result;
}

pub fn close_panel(panel: Panel, path: Option<String>) {
    trace!(">close_panel");
    far_api(|far_api: &mut ffi::PluginStartupInfo| {
        let h_panel: ffi::HANDLE = panel.into();
        let path_ws = path.map(|p| WideString::from(p.as_str()));
        let param1: libc::intptr_t = 0;
        let param2: *const u16 = match path_ws {
            None => ptr::null(),
            Some(path) => path.as_ptr(),
        };
        let _ = far_api.panel_control(h_panel,
                                      ffi::FILE_CONTROL_COMMANDS::FCTL_CLOSEPANEL,
                                      param1,
                                      param2 as *mut libc::c_void);
    });
    trace!("<close_panel");
}

pub fn get_panel_info(panel: Panel) -> crate::Result<PanelInfo> {
    trace!(">get_panel_info");
    let result = far_api(|far_api: &mut ffi::PluginStartupInfo| {
        let param1: libc::intptr_t = 0;
        let param2;

        let mut panel_info: ffi::PanelInfo = ffi::PanelInfo {
            struct_size: mem::size_of::<ffi::PanelInfo>(),
            plugin_handle: ptr::null_mut(),
            owner_guid: ffi::DEFAULT_GUID,
            flags: PANELINFOFLAGS::PFLAGS_NONE,
            items_number: 0,
            selected_items_number: 0,
            panel_rect: ffi::RECT {
                left: 0,
                top: 0,
                right: 0,
                bottom: 0,
            },
            current_item: 0,
            top_panel_item: 0,
            view_mode: 0,
            panel_type: PANELINFOTYPE::PTYPE_FILEPANEL,
            sort_mode: OPENPANELINFO_SORTMODES::SM_DEFAULT,
        };
        param2 = &mut panel_info as *mut _ as *mut libc::c_void;

        let r = far_api.panel_control(panel.into(),
                                      ffi::FILE_CONTROL_COMMANDS::FCTL_GETPANELINFO,
                                      param1,
                                      param2);
        match r {
            0 => Err(format_err!("")),
            _ => Ok(PanelInfo {
                plugin_handle: panel_info.plugin_handle,
                owner_guid: panel_info.owner_guid,
                flags: panel_info.flags,
                items_number: panel_info.items_number,
                selected_items_number: panel_info.selected_items_number,
                panel_rect: panel_info.panel_rect,
                current_item: panel_info.current_item,
                top_panel_item: panel_info.top_panel_item,
                view_mode: panel_info.view_mode,
                panel_type: panel_info.panel_type,
                sort_mode: panel_info.sort_mode,
            })
        }
    });
    trace!("<get_panel_info");
    return result;
}

pub fn get_column_types(panel: Panel) -> crate::Result<Vec<String>> {
    trace!(">get_column_types");
    let result = far_api(|far_api: &mut ffi::PluginStartupInfo| {
        let h_panel = panel.into();
        let mut param1: libc::intptr_t = 0;
        let mut param2: *mut libc::c_void = ptr::null_mut();

        let chars_count = far_api.panel_control(h_panel,
                                                ffi::FILE_CONTROL_COMMANDS::FCTL_GETCOLUMNTYPES,
                                                param1,
                                                param2) as usize;

        if chars_count == 0 {
            return Err(format_err!(""));
        }

        let mut buf: Vec<ffi::wchar_t> = vec![0; chars_count];
        param1 = chars_count as libc::intptr_t;
        param2 = buf.as_mut_ptr() as *mut libc::c_void;

        let r = far_api.panel_control(h_panel,
                                      ffi::FILE_CONTROL_COMMANDS::FCTL_GETCOLUMNTYPES,
                                      param1,
                                      param2);
        match r {
            0 => Err(format_err!("")),
            _ => {
                let result = WideString::from(buf.as_slice()).to_string_lossy()
                    .split(',').map(|w| w.to_string()).collect();
                Ok(result)
            }
        }
    });
    trace!("<get_column_types");
    return result;
}

pub fn get_column_widths(panel: Panel) -> crate::Result<Vec<usize>> {
    trace!(">get_column_widths");
    let result = far_api(|far_api: &mut ffi::PluginStartupInfo| {
        let h_panel = panel.into();
        let mut param1: libc::intptr_t = 0;
        let mut param2: *mut libc::c_void = ptr::null_mut();

        let chars_count = far_api.panel_control(h_panel,
                                                ffi::FILE_CONTROL_COMMANDS::FCTL_GETCOLUMNWIDTHS,
                                                param1,
                                                param2) as usize;

        if chars_count == 0 {
            return Err(format_err!(""));
        }

        let mut buf: Vec<ffi::wchar_t> = vec![0; chars_count];
        param1 = chars_count as libc::intptr_t;
        param2 = buf.as_mut_ptr() as *mut libc::c_void;

        let r = far_api.panel_control(h_panel,
                                      ffi::FILE_CONTROL_COMMANDS::FCTL_GETCOLUMNWIDTHS,
                                      param1,
                                      param2);
        match r {
            0 => Err(format_err!("")),
            _ => {
                let result = WideString::from(buf.as_slice()).to_string_lossy()
                    .split(',').map(|w| w.parse::<usize>().unwrap()).collect();
                Ok(result)
            }
        }
    });
    trace!("<get_column_widths");
    return result;
}

pub fn get_panel_directory(panel: Panel) -> crate::Result<FarPanelDirectory> {
    trace!(">get_panel_directory");
    let result = far_api(|far_api: &mut ffi::PluginStartupInfo| {
        let h_panel: ffi::HANDLE = panel.into();
        let mut param1: libc::intptr_t = 0;
        let mut param2: *mut libc::c_void = ptr::null_mut();

        let size: libc::size_t = far_api.panel_control(h_panel,
                                                       ffi::FILE_CONTROL_COMMANDS::FCTL_GETPANELDIRECTORY,
                                                       param1,
                                                       param2) as libc::size_t;

        if size == 0 {
            return Err(format_err!(""));
        }

        let mut buf: Vec<u8> = vec![0; size];
        let plugin_directory_item: *mut ffi::FarPanelDirectory = buf.as_mut_ptr() as *mut ffi::FarPanelDirectory;

        (unsafe { plugin_directory_item.as_mut().unwrap() }).struct_size = mem::size_of::<ffi::FarPanelDirectory>();
        param1 = buf.len() as libc::intptr_t;
        param2 = plugin_directory_item as *mut libc::c_void;

        let r = far_api.panel_control(h_panel,
                                      ffi::FILE_CONTROL_COMMANDS::FCTL_GETPANELDIRECTORY,
                                      param1,
                                      param2);
        match r {
            0 => Err(format_err!("")),
            _ => Ok(FarPanelDirectory::from(unsafe {&*plugin_directory_item}))
        }
    });
    trace!("<get_panel_directory");
    return result;
}

pub fn get_panel_format(panel: Panel) -> crate::Result<String> {
    trace!(">get_panel_format");
    let result = far_api(|far_api: &mut ffi::PluginStartupInfo| {
        let h_panel = panel.into();
        let mut param1: libc::intptr_t = 0;
        let mut param2: *mut libc::c_void = ptr::null_mut();

        let chars_count = far_api.panel_control(h_panel,
                                                ffi::FILE_CONTROL_COMMANDS::FCTL_GETPANELFORMAT,
                                                param1,
                                                param2) as usize;

        if chars_count == 0 {
            return Err(format_err!(""));
        }

        let mut buf: Vec<ffi::wchar_t> = vec![0; chars_count];
        param1 = chars_count as libc::intptr_t;
        param2 = buf.as_mut_ptr() as *mut libc::c_void;

        let r = far_api.panel_control(h_panel,
                                      ffi::FILE_CONTROL_COMMANDS::FCTL_GETPANELFORMAT,
                                      param1,
                                      param2);
        match r {
            0 => Err(format_err!("")),
            _ => {
                let result = WideString::from(buf.as_slice());
                Ok(result.to_string_lossy())
            }
        }
    });
    trace!("<get_panel_format");
    return result;
}

pub fn get_panel_host_file(panel: Panel) -> crate::Result<String> {
    trace!(">get_panel_host_file");
    let result = far_api(|far_api: &mut ffi::PluginStartupInfo| {
        let h_panel = panel.into();
        let mut param1: libc::intptr_t = 0;
        let mut param2: *mut libc::c_void = ptr::null_mut();

        let chars_count = far_api.panel_control(h_panel,
                                                ffi::FILE_CONTROL_COMMANDS::FCTL_GETPANELHOSTFILE,
                                                param1,
                                                param2) as usize;

        if chars_count == 0 {
            return Err(format_err!(""));
        }

        let mut buf: Vec<ffi::wchar_t> = vec![0; chars_count];
        param1 = chars_count as libc::intptr_t;
        param2 = buf.as_mut_ptr() as *mut libc::c_void;

        let r = far_api.panel_control(h_panel,
                                      ffi::FILE_CONTROL_COMMANDS::FCTL_GETPANELHOSTFILE,
                                      param1,
                                      param2);
        match r {
            0 => Err(format_err!("")),
            _ => {
                let result = WideString::from(buf.as_slice());
                Ok(result.to_string_lossy())
            }
        }
    });
    trace!("<get_panel_host_file");
    return result;
}

pub fn get_panel_item(panel: Panel, item_index: usize) -> crate::Result<PluginPanelItem> {
    trace!(">get_panel_item");
    let result = far_api(|far_api: &mut ffi::PluginStartupInfo| {
        let param1: libc::intptr_t = item_index as libc::intptr_t;
        let mut param2: *mut libc::c_void = ptr::null_mut();

        let size: libc::size_t = far_api.panel_control(panel.into(),
                                                       ffi::FILE_CONTROL_COMMANDS::FCTL_GETPANELITEM,
                                                       param1,
                                                       param2) as libc::size_t;

        if size == 0 {
            return Err(format_err!(""));
        }

        let mut buf: Vec<u8> = vec![0; size];
        let plugin_panel_item: *mut ffi::PluginPanelItem = buf.as_mut_ptr() as *mut ffi::PluginPanelItem;

        let mut far_get_plugin_panel_item = ffi::FarGetPluginPanelItem {
            struct_size: mem::size_of::<ffi::FarGetPluginPanelItem>(),
            size: buf.capacity(),
            item: plugin_panel_item,
        };
        param2 = &mut far_get_plugin_panel_item as *mut _ as *mut libc::c_void;

        let r = far_api.panel_control(ffi::PANEL_ACTIVE,
                                      ffi::FILE_CONTROL_COMMANDS::FCTL_GETPANELITEM,
                                      param1,
                                      param2);
        match r {
            0 => Err(format_err!("")),
            _ => Ok(PluginPanelItem::from(unsafe {&*plugin_panel_item}))
        }
    });
    trace!("<get_panel_item");
    return result;
}

pub fn get_panel_prefix(panel: Panel) -> crate::Result<String> {
    trace!(">get_panel_prefix");
    let result = far_api(|far_api: &mut ffi::PluginStartupInfo| {
        let h_panel = panel.into();
        let mut param1: libc::intptr_t = 0;
        let mut param2: *mut libc::c_void = ptr::null_mut();

        let chars_count = far_api.panel_control(h_panel,
                                                ffi::FILE_CONTROL_COMMANDS::FCTL_GETPANELPREFIX,
                                                param1,
                                                param2) as usize;

        if chars_count == 0 {
            return Err(format_err!(""));
        }

        let mut buf: Vec<ffi::wchar_t> = vec![0; chars_count];
        param1 = chars_count as libc::intptr_t;
        param2 = buf.as_mut_ptr() as *mut libc::c_void;

        let r = far_api.panel_control(h_panel,
                                      ffi::FILE_CONTROL_COMMANDS::FCTL_GETPANELPREFIX,
                                      param1,
                                      param2);
        match r {
            0 => Err(format_err!("")),
            _ => {
                let result = WideString::from(buf.as_slice());
                Ok(result.to_string_lossy())
            }
        }
    });
    trace!("<get_panel_prefix");
    return result;
}

pub fn get_selected_panel_item(panel: Panel, sel_item_index: usize) -> crate::Result<PluginPanelItem> {
    trace!(">get_selected_panel_item");
    let result = far_api(|far_api: &mut ffi::PluginStartupInfo| {
        let param1: libc::intptr_t = sel_item_index as libc::intptr_t;
        let mut param2: *mut libc::c_void = ptr::null_mut();

        let size: libc::size_t = far_api.panel_control(panel.into(),
                                                       ffi::FILE_CONTROL_COMMANDS::FCTL_GETSELECTEDPANELITEM,
                                                       param1,
                                                       param2) as libc::size_t;

        if size == 0 {
            return Err(format_err!(""));
        }

        let mut buf: Vec<u8> = vec![0; size];
        let plugin_panel_item: *mut ffi::PluginPanelItem = buf.as_mut_ptr() as *mut ffi::PluginPanelItem;

        let mut far_get_plugin_panel_item = ffi::FarGetPluginPanelItem {
            struct_size: mem::size_of::<ffi::FarGetPluginPanelItem>(),
            size: buf.capacity(),
            item: plugin_panel_item,
        };
        param2 = &mut far_get_plugin_panel_item as *mut _ as *mut libc::c_void;

        let r = far_api.panel_control(ffi::PANEL_ACTIVE,
                                      ffi::FILE_CONTROL_COMMANDS::FCTL_GETSELECTEDPANELITEM,
                                      param1,
                                      param2);
        match r {
            0 => Err(format_err!("")),
            _ => Ok(PluginPanelItem::from(unsafe {&*plugin_panel_item}))
        }
    });
    trace!("<get_selected_panel_item");
    return result;
}

pub fn get_current_panel_item(panel: Panel) -> crate::Result<PluginPanelItem> {
    trace!(">get_current_panel_item");
    let result = far_api(|far_api: &mut ffi::PluginStartupInfo| {
        let param1: libc::intptr_t = 0;
        let mut param2: *mut libc::c_void = ptr::null_mut();

        let size: libc::size_t = far_api.panel_control(panel.into(),
                                                       ffi::FILE_CONTROL_COMMANDS::FCTL_GETCURRENTPANELITEM,
                                                       param1,
                                                       param2) as libc::size_t;

        if size == 0 {
            return Err(format_err!(""));
        }

        let mut buf: Vec<u8> = vec![0; size];
        let plugin_panel_item: *mut ffi::PluginPanelItem = buf.as_mut_ptr() as *mut ffi::PluginPanelItem;

        let mut far_get_plugin_panel_item = ffi::FarGetPluginPanelItem {
            struct_size: mem::size_of::<ffi::FarGetPluginPanelItem>(),
            size: buf.capacity(),
            item: plugin_panel_item,
        };
        param2 = &mut far_get_plugin_panel_item as *mut _ as *mut libc::c_void;

        let r = far_api.panel_control(ffi::PANEL_ACTIVE,
                                      ffi::FILE_CONTROL_COMMANDS::FCTL_GETCURRENTPANELITEM,
                                      param1,
                                      param2);
        match r {
            0 => Err(format_err!("")),
            _ => Ok(PluginPanelItem::from(unsafe {&*plugin_panel_item}))
        }
    });
    trace!("<get_current_panel_item");
    return result;
}

pub fn redraw_panel(panel: Panel, redraw_info: Option<PanelRedrawInfo>) {
    trace!(">redraw_panel");
    far_api(|far_api: &mut ffi::PluginStartupInfo| {
        let h_panel = panel.into();
        let param1: libc::intptr_t = 0;
        match redraw_info {
            Some(redraw_info) => {
                let mut panel_redraw_info: ffi::PanelRedrawInfo = ffi::PanelRedrawInfo {
                    struct_size: mem::size_of::<ffi::FarPanelDirectory>(),
                    current_item: redraw_info.current_item,
                    top_panel_item: redraw_info.top_panel_item,
                };

                let param2 = &mut panel_redraw_info as *mut _ as *mut libc::c_void;
                let _ = far_api.panel_control(h_panel,
                                              ffi::FILE_CONTROL_COMMANDS::FCTL_REDRAWPANEL,
                                              param1,
                                              param2);
            },
            None => {
                let param2 = ptr::null_mut();
                let _ = far_api.panel_control(h_panel,
                                              ffi::FILE_CONTROL_COMMANDS::FCTL_REDRAWPANEL,
                                              param1,
                                              param2);

            },
        };
    });
    trace!("<redraw_panel");
}

pub fn set_active_panel(panel: Panel) -> bool {
    trace!(">set_active_panel");
    let result = far_api(|far_api: &mut ffi::PluginStartupInfo| {
        let handle = panel.into();
        let param1: libc::intptr_t = 0;
        let param2: *mut libc::c_void = ptr::null_mut();
        let result = far_api.panel_control(handle,
                                           ffi::FILE_CONTROL_COMMANDS::FCTL_SETACTIVEPANEL,
                                           param1,
                                           param2);
        match result {
            0 => false,
            _ => true
        }
    });
    trace!("<set_active_panel");
    return result;
}

pub fn set_panel_directory(panel: Panel, guid: ffi::GUID, path: String, host_file: String) -> crate::Result<()> {
    trace!(">set_panel_directory");
    let result = far_api(|far_api: &mut ffi::PluginStartupInfo| {
        let h_panel = panel.into();
        let path_ws = WideString::from(path);
        let param_ws = WideString::from("");
        let host_file_ws = WideString::from(host_file);
        let mut far_panel_directory: ffi::FarPanelDirectory = ffi::FarPanelDirectory {
            struct_size: mem::size_of::<ffi::FarPanelDirectory>(),
            name: path_ws.as_ptr(),
            param: param_ws.as_ptr(),
            plugin_id: guid,
            file: host_file_ws.as_ptr(),
        };

        let param1: libc::intptr_t = 0;
        let param2 = &mut far_panel_directory as *mut _ as *mut libc::c_void;

        let r = far_api.panel_control(h_panel,
                                      ffi::FILE_CONTROL_COMMANDS::FCTL_SETPANELDIRECTORY,
                                      param1,
                                      param2);
        match r {
            0 => Err(format_err!("")),
            _ => {
                Ok(())
            }
        }
    });
    trace!("<set_panel_directory");
    return result;
}

pub fn begin_selection(panel: Panel) {
    trace!(">begin_selection");
    far_api(|far_api: &mut ffi::PluginStartupInfo| {
        let param1: libc::intptr_t = 0;
        let param2: *mut libc::c_void = ptr::null_mut();
        let _ = far_api.panel_control(panel.into(),
                                      ffi::FILE_CONTROL_COMMANDS::FCTL_BEGINSELECTION,
                                      param1,
                                      param2);
    });
    trace!("<begin_selection");
}

pub fn end_selection(panel: Panel) {
    trace!(">end_selection");
    far_api(|far_api: &mut ffi::PluginStartupInfo| {
        let param1: libc::intptr_t = 0;
        let param2: *mut libc::c_void = ptr::null_mut();
        let _ = far_api.panel_control(panel.into(),
                                      ffi::FILE_CONTROL_COMMANDS::FCTL_ENDSELECTION,
                                      param1,
                                      param2);
    });
    trace!("<end_selection");
}

pub fn set_selection(panel: Panel, item_index: usize, select: bool) {
    trace!(">set_selection");
    far_api(|far_api: &mut ffi::PluginStartupInfo| {
        let param1: libc::intptr_t = item_index as libc::intptr_t;
        let param2: *mut libc::c_void = match select {
            false => ptr::null_mut(),
            true => 1 as *mut libc::c_void
        };
        let _ = far_api.panel_control(panel.into(),
                                      ffi::FILE_CONTROL_COMMANDS::FCTL_SETSELECTION,
                                      param1,
                                      param2);
    });
    trace!("<set_selection");
}

pub fn clear_selection(panel: Panel, item_index: usize) {
    trace!(">clear_selection");
    far_api(|far_api: &mut ffi::PluginStartupInfo| {
        let param1: libc::intptr_t = item_index as libc::intptr_t;
        let param2: *mut libc::c_void = ptr::null_mut();
        let _ = far_api.panel_control(panel.into(),
                                      ffi::FILE_CONTROL_COMMANDS::FCTL_CLEARSELECTION,
                                      param1,
                                      param2);
    });
    trace!("<clear_selection");
}

pub fn set_sort_mode(panel: Panel, sort_mode: OPENPANELINFO_SORTMODES) {
    trace!(">set_sort_mode");
    far_api(|far_api: &mut ffi::PluginStartupInfo| {
        let param1: libc::intptr_t = sort_mode as libc::intptr_t;
        let param2: *mut libc::c_void = ptr::null_mut();
        let _ = far_api.panel_control(panel.into(),
                                      ffi::FILE_CONTROL_COMMANDS::FCTL_SETSORTMODE,
                                      param1,
                                      param2);
    });
    trace!("<set_sort_mode");
}

pub fn set_sort_order(panel: Panel, sort_order: OPENPANELINFO_SORTORDERS) {
    trace!(">set_sort_order");
    far_api(|far_api: &mut ffi::PluginStartupInfo| {
        let param1: libc::intptr_t = sort_order as libc::intptr_t;
        let param2: *mut libc::c_void = ptr::null_mut();
        let _ = far_api.panel_control(panel.into(),
                                      ffi::FILE_CONTROL_COMMANDS::FCTL_SETSORTORDER,
                                      param1,
                                      param2);
    });
    trace!("<set_sort_order");
}

pub fn set_view_mode(panel: Panel, view_mode: isize) {
    trace!(">set_view_mode");
//    if view_mode < 0 || view_mode > 9 {
//        return;
//    }
    far_api(|far_api: &mut ffi::PluginStartupInfo| {
        let param1: libc::intptr_t = view_mode;
        let param2: *mut libc::c_void = ptr::null_mut();
        let _ = far_api.panel_control(panel.into(),
                                      ffi::FILE_CONTROL_COMMANDS::FCTL_SETVIEWMODE,
                                      param1,
                                      param2);
    });
    trace!("<set_view_mode");
}

pub fn update_panel(panel: Panel, preserve_selection: bool) {
    trace!(">update_panel");
    far_api(|far_api: &mut ffi::PluginStartupInfo| {
        let param1: libc::intptr_t = match preserve_selection {
            false => 0,
            true => 1
        };
        let param2: *mut libc::c_void = ptr::null_mut();
        let _ = far_api.panel_control(panel.into(),
                                      ffi::FILE_CONTROL_COMMANDS::FCTL_UPDATEPANEL,
                                      param1,
                                      param2);
    });
    trace!("<update_panel");
}

pub fn set_directories_first(panel: Panel, directories_first: bool) {
    trace!(">set_directories_first");
    far_api(|far_api: &mut ffi::PluginStartupInfo| {
        let param1: libc::intptr_t = match directories_first {
            false => 0,
            true => 1
        };
        let param2: *mut libc::c_void = ptr::null_mut();
        let _ = far_api.panel_control(panel.into(),
                                      ffi::FILE_CONTROL_COMMANDS::FCTL_SETDIRECTORIESFIRST,
                                      param1,
                                      param2);
    });
    trace!("<set_directories_first");
}

pub fn get_cmd_line(panel: Panel) -> crate::Result<String> {
    trace!(">get_cmd_line");
    let result = far_api(|far_api: &mut ffi::PluginStartupInfo| {
        let h_panel = panel.into();
        let mut param1: libc::intptr_t = 0;
        let mut param2: *mut libc::c_void = ptr::null_mut();

        let chars_count = far_api.panel_control(h_panel,
                                                ffi::FILE_CONTROL_COMMANDS::FCTL_GETCMDLINE,
                                                param1,
                                                param2) as usize;
        let mut buf: Vec<ffi::wchar_t> = vec![0; chars_count];
        param1 = chars_count as libc::intptr_t;
        param2 = buf.as_mut_ptr() as *mut libc::c_void;

        let r = far_api.panel_control(h_panel,
                                      ffi::FILE_CONTROL_COMMANDS::FCTL_GETCMDLINE,
                                      param1,
                                      param2);
        match r {
            0 => Err(format_err!("")),
            _ => {
                let result = WideString::from(buf.as_slice());
                Ok(result.to_string_lossy())
            }
        }
    });
    trace!("<get_cmd_line");
    return result;
}

pub fn get_cmd_line_pos(panel: Panel) -> crate::Result<usize> {
    trace!(">get_cmd_line_pos");
    let result = far_api(|far_api: &mut ffi::PluginStartupInfo| {
        let mut pos: usize = 0;

        let h_panel = panel.into();
        let param1: libc::intptr_t = 0;
        let param2 = &mut pos as *mut _ as *mut libc::c_void;

        let r = far_api.panel_control(h_panel,
                                      ffi::FILE_CONTROL_COMMANDS::FCTL_GETCMDLINEPOS,
                                      param1,
                                      param2);
        match r {
            0 => Err(format_err!("")),
            _ => {
                Ok(pos)
            }
        }
    });
    trace!("<get_cmd_line_pos");
    return result;
}

pub fn get_cmd_line_selection(panel: Panel) -> crate::Result<(isize, isize)> {
    trace!(">get_cmd_line_selection");
    let result = far_api(|far_api: &mut ffi::PluginStartupInfo| {
        let mut sel: ffi::CmdLineSelect = ffi::CmdLineSelect {
            struct_size: mem::size_of::<ffi::CmdLineSelect>(),
            sel_start: 0,
            sel_end: 0,
        };

        let h_panel = panel.into();
        let param1: libc::intptr_t = 0;
        let param2 = &mut sel as *mut _ as *mut libc::c_void;

        let r = far_api.panel_control(h_panel,
                                      ffi::FILE_CONTROL_COMMANDS::FCTL_GETCMDLINESELECTION,
                                      param1,
                                      param2);
        match r {
            0 => Err(format_err!("")),
            _ => {
                Ok((sel.sel_start, sel.sel_end))
            }
        }
    });
    trace!("<get_cmd_line_selection");
    return result;
}

pub fn insert_cmd_line(panel: Panel, text: String) -> crate::Result<()> {
    trace!(">insert_cmd_line");
    let result = far_api(|far_api: &mut ffi::PluginStartupInfo| {
        let text_ws = WideString::from(text);
        let h_panel = panel.into();
        let param1: libc::intptr_t = 0;
        let param2 = text_ws.as_ptr() as *mut libc::c_void;

        let r = far_api.panel_control(h_panel,
                                      ffi::FILE_CONTROL_COMMANDS::FCTL_INSERTCMDLINE,
                                      param1,
                                      param2);
        match r {
            0 => Err(format_err!("")),
            _ => {
                Ok(())
            }
        }
    });
    trace!("<insert_cmd_line");
    return result;
}

pub fn set_cmd_line(panel: Panel, text: String) -> crate::Result<()> {
    trace!(">set_cmd_line");
    let result = far_api(|far_api: &mut ffi::PluginStartupInfo| {
        let text_ws = WideString::from(text);
        let h_panel = panel.into();
        let param1: libc::intptr_t = 0;
        let param2 = text_ws.as_ptr() as *mut libc::c_void;

        let r = far_api.panel_control(h_panel,
                                      ffi::FILE_CONTROL_COMMANDS::FCTL_SETCMDLINE,
                                      param1,
                                      param2);
        match r {
            0 => Err(format_err!("")),
            _ => {
                Ok(())
            }
        }
    });
    trace!("<set_cmd_line");
    return result;
}

pub fn set_cmd_line_pos(panel: Panel, pos: usize) -> crate::Result<()> {
    trace!(">set_cmd_line_pos");
    let result = far_api(|far_api: &mut ffi::PluginStartupInfo| {
        let h_panel = panel.into();
        let param1: libc::intptr_t = pos as libc::intptr_t;
        let param2 = ptr::null_mut();

        let r = far_api.panel_control(h_panel,
                                      ffi::FILE_CONTROL_COMMANDS::FCTL_SETCMDLINEPOS,
                                      param1,
                                      param2);
        match r {
            0 => Err(format_err!("")),
            _ => {
                Ok(())
            }
        }
    });
    trace!("<set_cmd_line_pos");
    return result;
}

pub fn set_cmd_line_selection(panel: Panel, selection: Option<(usize, usize)>) -> crate::Result<()> {
    trace!(">set_cmd_line_selection");
    let result = far_api(|far_api: &mut ffi::PluginStartupInfo| {
        let mut sel = match selection {
            Some((sel_start, sel_end)) => ffi::CmdLineSelect {
                struct_size: mem::size_of::<ffi::CmdLineSelect>(),
                sel_start: sel_start as libc::intptr_t,
                sel_end: sel_end as libc::intptr_t,
            },
            None => ffi::CmdLineSelect {
                struct_size: mem::size_of::<ffi::CmdLineSelect>(),
                sel_start: -1,
                sel_end: -1,
            },
        };

        let h_panel = panel.into();
        let param1: libc::intptr_t = 0;
        let param2 = &mut sel as *mut _ as *mut libc::c_void;

        let r = far_api.panel_control(h_panel,
                                      ffi::FILE_CONTROL_COMMANDS::FCTL_SETCMDLINESELECTION,
                                      param1,
                                      param2);
        match r {
            0 => Err(format_err!("")),
            _ => Ok(())
        }
    });
    trace!("<set_cmd_line_selection");
    return result;
}

pub fn set_user_screen(panel: Panel, no_new_line: bool) -> crate::Result<()> {
    trace!(">set_user_screen");
    let result = far_api(|far_api: &mut ffi::PluginStartupInfo| {
        let h_panel = panel.into();
        let param1: libc::intptr_t = match no_new_line {
            true => 1,
            false => 0
        };
        let param2 = ptr::null_mut();

        let r = far_api.panel_control(h_panel,
                                      ffi::FILE_CONTROL_COMMANDS::FCTL_SETUSERSCREEN,
                                      param1,
                                      param2);
        match r {
            0 => Err(format_err!("")),
            _ => Ok(())
        }
    });
    trace!("<set_user_screen");
    return result;
}

pub fn get_user_screen(panel: Panel, no_new_line: bool) -> crate::Result<()> {
    trace!(">get_user_screen");
    let result = far_api(|far_api: &mut ffi::PluginStartupInfo| {
        let h_panel = panel.into();
        let param1: libc::intptr_t = match no_new_line {
            true => 1,
            false => 0
        };
        let param2 = ptr::null_mut();

        let r = far_api.panel_control(h_panel,
                                      ffi::FILE_CONTROL_COMMANDS::FCTL_GETUSERSCREEN,
                                      param1,
                                      param2);
        match r {
            0 => Err(format_err!("")),
            _ => Ok(())
        }
    });
    trace!("<get_user_screen");
    return result;
}
