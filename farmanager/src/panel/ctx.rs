use std::collections::HashMap;
use std::mem;
use std::ptr;

use crate::common::Enrichable;
use crate::common::ffi::Array;
use crate::common::string::{WideString, WideStringArray};
use crate::ffi;
use crate::panel::*;

use super::wrapper as wrp;

#[allow(dead_code)]
pub struct Panel {
    pub(super) open_panel_info: wrp::OpenPanelInfo,
}

impl Default for Panel {
    fn default() -> Panel {
        Panel {
            open_panel_info: wrp::OpenPanelInfo {
                inner: ffi::OpenPanelInfo {
                    struct_size: 0,
                    h_panel: ptr::null_mut(),
                    flags: ffi::OPENPANELINFO_FLAGS::OPIF_NONE,
                    host_file: ptr::null(),
                    cur_dir: ptr::null(),
                    format: ptr::null(),
                    panel_title: ptr::null(),
                    info_lines: ptr::null(),
                    info_lines_number: 0,
                    descr_files: ptr::null(),
                    descr_files_number: 0,
                    panel_modes_array: ptr::null(),
                    panel_modes_number: 0,
                    start_panel_mode: 0,
                    start_sort_mode: ffi::OPENPANELINFO_SORTMODES::SM_DEFAULT,
                    start_sort_order: 0,
                    key_bar: ptr::null(),
                    shortcut_data: ptr::null(),
                    free_size: 0,
                    user_data: ffi::UserDataItem {
                        data: ptr::null_mut(),
                        free_data: None,
                    },
                    instance: ptr::null(),
                },
                host_file: None,
                cur_dir: WideString::new(),
                format: None,
                panel_title: WideString::new(),
                info_lines: Array::new(),
                descr_files: None,
            },
        }
    }
}

impl Panel {

    #[allow(dead_code)]
    pub(super) fn set_current_directory(&mut self, dir: &str) {
        self.open_panel_info.cur_dir = WideString::from(dir);
    }

}

pub(super) struct Context {
    active_panels: HashMap<libc::intptr_t, Panel>,
}

impl Context {

    pub(crate) fn panel(&mut self, handle: ffi::HANDLE) -> &mut Panel {
        if !self.active_panels.contains_key(&(handle as libc::intptr_t)) {
            self.active_panels.insert(handle as libc::intptr_t, Panel::default());
        }
        return self.active_panels.get_mut(&(handle as libc::intptr_t)).unwrap();
    }

    #[allow(dead_code)]
    pub(super) fn remove_panel(&mut self, handle: crate::HANDLE) {
        self.active_panels.remove(&(handle as libc::intptr_t));
    }

    #[allow(dead_code)]
    pub(crate) fn count(&self) -> usize {
        self.active_panels.len()
    }

}

impl Default for Context {

    fn default() -> Context {
        Context {
            active_panels: HashMap::new(),
        }
    }
}

impl Enrichable<Panel, &super::PluginPanelItems> for ffi::GetFindDataInfo {
    fn enrich(&mut self, _ctx: &mut Panel, src: &super::PluginPanelItems) {
        self.items_number = src.panel_items.len();
        self.panel_item = src.panel_items.as_ptr();
    }
}

impl Enrichable<Context, (ffi::HANDLE, &crate::panel::OpenPanelInfo)> for ffi::OpenPanelInfo {
    fn enrich(&mut self, ctx: &mut Context, (h_panel, src): (ffi::HANDLE, &crate::panel::OpenPanelInfo)) {
        let panel: &mut Panel = ctx.panel(h_panel);

        panel.open_panel_info.inner.flags = src.flags;
        panel.open_panel_info.host_file = src.host_file.as_ref().map(|s| WideString::from(s.as_str()));
        panel.open_panel_info.inner.host_file = match panel.open_panel_info.host_file {
            Some(ref file) => file.as_ptr(),
            None => ptr::null()
        };

        panel.open_panel_info.cur_dir = WideString::from(src.cur_dir.as_str());
        panel.open_panel_info.inner.cur_dir = panel.open_panel_info.cur_dir.as_ptr();

        panel.open_panel_info.format = src.format.as_ref().map(|s| WideString::from(s.as_str()));
        panel.open_panel_info.inner.format = match panel.open_panel_info.format {
            Some(ref file) => file.as_ptr(),
            None => ptr::null()
        };

        panel.open_panel_info.panel_title = WideString::from(src.panel_title.as_str());
        panel.open_panel_info.inner.panel_title = panel.open_panel_info.panel_title.as_ptr();

        let info_lines: Vec<wrp::InfoPanelLine> = src.info_lines.iter().map(wrp::InfoPanelLine::from).collect();
        panel.open_panel_info.info_lines = Array::from(info_lines);
        panel.open_panel_info.inner.info_lines = panel.open_panel_info.info_lines.as_ptr();
        panel.open_panel_info.inner.info_lines_number = panel.open_panel_info.info_lines.len();

        panel.open_panel_info.descr_files = src.descr_files.as_ref().map(|descr_files| WideStringArray::from(descr_files.as_slice()));
        panel.open_panel_info.inner.descr_files = match &panel.open_panel_info.descr_files {
            Some(descr_files) => descr_files.as_ptr(),
            None => ptr::null(),
        };
        panel.open_panel_info.inner.descr_files_number = match &panel.open_panel_info.descr_files {
            Some(descr_files) => descr_files.len(),
            None => 0,
        };

        let raw_data: ffi::OpenPanelInfo = panel.open_panel_info.inner;

        self.struct_size = mem::size_of::<ffi::OpenPanelInfo>();
        self.flags = raw_data.flags;
        self.host_file = raw_data.host_file;
        self.cur_dir = raw_data.cur_dir;
        self.format = raw_data.format;
        self.panel_title = raw_data.panel_title;
        self.info_lines = raw_data.info_lines;
        self.info_lines_number = raw_data.info_lines_number;
        self.descr_files = raw_data.descr_files;
        self.descr_files_number = raw_data.descr_files_number;
        self.panel_modes_array = raw_data.panel_modes_array;
        self.panel_modes_number = raw_data.panel_modes_number;
        self.start_panel_mode = raw_data.start_panel_mode;
        self.start_sort_mode = raw_data.start_sort_mode;
        self.start_sort_order = raw_data.start_sort_order;
        self.key_bar = raw_data.key_bar;
        self.shortcut_data = raw_data.shortcut_data;
        self.free_size = src.free_size;
    }
}
