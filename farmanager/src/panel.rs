extern crate libc;

use std::cell::RefCell;
use std::cmp::Ordering;
use std::mem;
use std::panic;
use std::prelude::v1::Vec;
use std::ptr;
use std::slice;

use bitflags::bitflags;
use failure::*;
use log::*;

#[allow(unused_imports)] // TODO remove after IntelliJ Rust will stop to highlight is as an error
use crate::common::Enrichable;
use crate::common::ffi::Array;
use crate::common::string::WideString;
use crate::far_api;
#[allow(unused_imports)] // TODO remove after IntelliJ Rust will stop to highlight is as an error
use crate::FarPlugin;
use crate::ffi;
pub use crate::ffi::FILETIME as FILETIME;
pub use crate::ffi::INFOPANELLINE_FLAGS as INFOPANELLINE_FLAGS;
pub use crate::ffi::OPENPANELINFO_FLAGS as OPENPANELINFO_FLAGS;
pub use crate::ffi::OPENPANELINFO_SORTMODES as OPENPANELINFO_SORTMODES;
pub use crate::ffi::OPERATION_MODES as OPERATION_MODES;
pub use crate::ffi::PANELINFOFLAGS as PANELINFOFLAGS;
pub use crate::ffi::PANELINFOTYPE as PANELINFOTYPE;
pub use crate::ffi::PANELMODE_FLAGS as PANELMODE_FLAGS;
pub use crate::ffi::PLUGINPANELITEMFLAGS as PLUGINPANELITEMFLAGS;
use crate::HANDLE;
#[allow(unused_imports)] // TODO remove after IntelliJ Rust will stop to highlight is as an error
use crate::plugin;
use crate::Result;
pub use crate::ReturnCode as ReturnCode;

use self::wrapper as wrp;

mod ctx;
mod wrapper;
pub mod control;
pub mod filter;

thread_local! {
    static CONTEXT: RefCell<Option<ctx::Context>> = RefCell::new(None);
}

pub enum PutFilesReturnCode {
    Success = 1,
    SuccessNoFileSelect = 2,
    UserCancel = -1
}

#[allow(unused_variables)]
pub trait ExportFunctions {

    fn analyse(&mut self, info: AnalyseInfo) -> crate::HANDLE {
        let result: crate::HANDLE = ptr::null_mut();
        result
    }
    fn close_analyse(&mut self, info: CloseAnalyseInfo) { }
    fn get_open_panel_info(&mut self, handle: crate::HANDLE) -> &OpenPanelInfo;
    #[allow(unused_variables)]
    fn close_panel(&mut self, handle: crate::HANDLE) {}
    fn get_find_data(&mut self, info: GetFindDataInfo) -> Result<&PluginPanelItems>;
    fn compare(&mut self, info: CompareInfo) -> Option<Ordering> {
        None
    }
    fn delete_files(&mut self, info: DeleteFilesInfo) -> Result<()>;
    fn free_find_data(&mut self, handle: crate::HANDLE);
    fn set_directory(&mut self, handle: crate::HANDLE, path: &String) -> Result<()>;
    #[allow(unused_variables)]
    fn get_files(&mut self, info: &mut GetFilesInfo) -> Result<ReturnCode> {
        Ok(ReturnCode::UserCancel)
    }
    #[allow(unused_variables)]
    fn make_directory(&mut self, info: &mut MakeDirectoryInfo) -> Result<ReturnCode> {
        Ok(ReturnCode::UserCancel)
    }
    #[allow(unused_variables)]
    fn process_panel_event(&mut self, info: ProcessPanelEventInfo) -> bool {
        false
    }
    #[allow(unused_variables)]
    fn process_host_file(&mut self, info: ProcessHostFileInfo) -> bool {
        false
    }
    #[allow(unused_variables)]
    fn process_panel_input(&mut self, info: ProcessPanelInputInfo) -> bool {
        false
    }
    #[allow(unused_variables)]
    fn put_files(&mut self, info: PutFilesInfo) -> Result<PutFilesReturnCode> {
        Ok(PutFilesReturnCode::UserCancel)
    }
    #[allow(unused_variables)]
    fn set_find_list(&mut self, info: SetFindListInfo) -> bool {
        false
    }
}

pub struct AnalyseInfo<'a>
{
    pub file_name: String,
    pub buffer: &'a [u8],
    pub op_mode: OPERATION_MODES
}

pub struct CloseAnalyseInfo
{
    pub handle: crate::HANDLE
}

pub struct FarPanelDirectory {
    pub name: String,
    pub plugin_id: crate::GUID,
    pub file: String,
}

impl From<&ffi::FarPanelDirectory> for FarPanelDirectory {
    fn from(src: &ffi::FarPanelDirectory) -> Self {
        FarPanelDirectory {
            name: unsafe { WideString::from_ptr_str(src.name) }.to_string_lossy(),
            plugin_id: src.plugin_id,
            file: unsafe { WideString::from_ptr_str(src.file) }.to_string_lossy(),
        }
    }
}

pub struct GetFindDataInfo {
    pub handle: crate::HANDLE,
    pub op_mode: OPERATION_MODES,
}

pub struct PluginPanelItem {
    pub creation_time: FILETIME,
    pub last_access_time: FILETIME,
    pub last_write_time: FILETIME,
    pub change_time: FILETIME,
    pub file_size: u64,
    pub allocation_size: u64,
    pub file_name: String,
    pub alternate_file_name: Option<String>,
    pub description: Option<String>,
    pub owner: Option<String>,
    pub flags: PLUGINPANELITEMFLAGS,
    pub file_attributes: FILE_ATTRIBUTES,
    pub number_of_links: usize,
    pub crc32: usize,
}

pub struct PluginPanelItems {
    panel_items: Array<wrp::PluginPanelItem, ffi::PluginPanelItem>,
}

impl From<Vec<PluginPanelItem>> for PluginPanelItems {
    fn from(src: Vec<PluginPanelItem>) -> Self {
        let items: Vec<wrp::PluginPanelItem> = src.into_iter().map(wrp::PluginPanelItem::from).collect();
        PluginPanelItems {
            panel_items: Array::from(items)
        }
    }
}

impl From<&ffi::PluginPanelItem> for PluginPanelItem {
    fn from(ppi: &ffi::PluginPanelItem) -> Self {
        PluginPanelItem {
            creation_time: ppi.creation_time,
            last_access_time: ppi.last_access_time,
            last_write_time: ppi.last_write_time,
            change_time: ppi.change_time,
            file_size: ppi.file_size,
            allocation_size: ppi.allocation_size,
            file_name: unsafe { WideString::from_ptr_str(ppi.file_name) }.to_string_lossy(),
            alternate_file_name: if ppi.alternate_file_name != ptr::null() {
                Some(unsafe { WideString::from_ptr_str(ppi.alternate_file_name) }.to_string_lossy())
            } else {
                None
            },
            description: if ppi.description != ptr::null() {
                Some(unsafe { WideString::from_ptr_str(ppi.description) }.to_string_lossy())
            } else {
                None
            },
            owner: if ppi.owner != ptr::null() {
                Some(unsafe { WideString::from_ptr_str(ppi.owner) }.to_string_lossy())
            } else {
                None
            },
            flags: ppi.flags,
            file_attributes: FILE_ATTRIBUTES::from_bits_truncate(ppi.file_attributes),
            number_of_links: ppi.number_of_links,
            crc32: ppi.crc32
        }
    }
}

bitflags! {
    #[allow(non_camel_case_types)]
    pub struct FILE_ATTRIBUTES: libc::uintptr_t {
        const READONLY               = 0x00000001;
        const HIDDEN                 = 0x00000002;
        const SYSTEM                 = 0x00000004;
        const DIRECTORY              = 0x00000010;
        const ARCHIVE                = 0x00000020;
        //const DEVICE                = 0x00000040;
        const NORMAL                 = 0x00000080;
        const TEMPORARY              = 0x00000100;
        const SPARSE_FILE            = 0x00000200;
        const REPARSE_POINT          = 0x00000400;
        const COMPRESSED             = 0x00000800;
        const OFFLINE                = 0x00001000;
        const NOT_CONTENT_INDEXED    = 0x00002000;
        const ENCRYPTED              = 0x00004000;
        //const INTEGRITY_STREAM      = 0x00008000;
        const VIRTUAL                = 0x00010000;
        //const NO_SCRUB_DATA         = 0x00020000;
        //const EA                    = 0x00040000;
        //const PINNED                = 0x00080000;
        //const UNPINNED              = 0x00100000;
        //const RECALL_ON_OPEN        = 0x00040000;
        //const RECALL_ON_DATA_ACCESS = 0x00400000;
    }
}

pub struct InfoPanelLine {
    pub text: String,
    pub data: String,
    pub flags: INFOPANELLINE_FLAGS
}

pub struct CompareInfo {
    pub panel: crate::HANDLE,
    pub item1: PluginPanelItem,
    pub item2: PluginPanelItem,
    pub op_mode: OPENPANELINFO_SORTMODES
}

pub struct DeleteFilesInfo {
    pub panel: crate::HANDLE,
    pub panel_items: Vec<PluginPanelItem>,
    pub op_mode: OPERATION_MODES
}

pub struct GetFilesInfo {
    pub panel: crate::HANDLE,
    pub panel_items: Vec<PluginPanelItem>,
    pub items_number: usize,
    pub move_file: bool,
    pub dest_path: String,
    pub op_mode: OPERATION_MODES,
}

#[allow(non_camel_case_types)]
pub enum OPENPANELINFO_SORTORDERS {
    ASC = 0,
    DESC = 1
}

pub struct PanelMode {
    pub column_types: String,
    pub column_widths: String,
    pub column_titles: Vec<String>,
    pub status_column_types: String,
    pub status_column_widths: String,
    pub flags: PANELMODE_FLAGS,
}

pub struct KeyBarLabel {
    pub key: crate::basic::FarKey,
    pub text: String,
    pub long_text: String
}

pub struct OpenPanelInfo {
    pub flags: OPENPANELINFO_FLAGS,
    pub host_file: Option<String>,
    pub cur_dir: String,
    pub format: Option<String>,
    pub panel_title: String,
    pub info_lines: Vec<InfoPanelLine>,
    pub descr_files: Option<Vec<String>>,
    pub panel_modes_array: Vec<PanelMode>,
    pub start_panel_mode: usize,
    pub start_sort_mode: OPENPANELINFO_SORTMODES,
    pub start_sort_order: OPENPANELINFO_SORTORDERS,
    pub key_bar: Option<Vec<KeyBarLabel>>,
    pub shortcut_data: Option<String>,
    pub free_size: u64
}

pub struct MakeDirectoryInfo {
    pub panel: crate::HANDLE,
    pub name: String,
    pub op_mode: OPERATION_MODES
}

impl From<ffi::MakeDirectoryInfo> for MakeDirectoryInfo {
    fn from(info: ffi::MakeDirectoryInfo) -> Self {
        MakeDirectoryInfo {
            panel: info.h_panel,
            name: unsafe { WideString::from_ptr_str(info.name) }.to_string_lossy(),
            op_mode: info.op_mode,
        }
    }
}

impl From<&ffi::MakeDirectoryInfo> for MakeDirectoryInfo {
    fn from(info: &ffi::MakeDirectoryInfo) -> Self {
        MakeDirectoryInfo {
            panel: info.h_panel,
            name: unsafe { WideString::from_ptr_str(info.name) }.to_string_lossy(),
            op_mode: info.op_mode,
        }
    }
}

pub enum PanelEvent {
    ChangeViewMode(String),
    Redraw,
    Idle,
    Close,
    Break(ffi::DWORD),
    Command(String),
    GotFocus,
    KillFocus,
    ChangeSortParams,
}

pub struct ProcessPanelEventInfo
{
    pub event: PanelEvent,
    pub h_panel: ffi::HANDLE
}

impl From<&ffi::ProcessPanelEventInfo> for ProcessPanelEventInfo {
    fn from(info: &ffi::ProcessPanelEventInfo) -> Self {
        ProcessPanelEventInfo {
            event: match info.event {
                ffi::FAR_EVENTS::FE_CHANGEVIEWMODE => PanelEvent::ChangeViewMode(unsafe { WideString::from_ptr_str(info.param as *const ffi::wchar_t) }.to_string_lossy()),
                ffi::FAR_EVENTS::FE_REDRAW => PanelEvent::Redraw,
                ffi::FAR_EVENTS::FE_IDLE => PanelEvent::Idle,
                ffi::FAR_EVENTS::FE_CLOSE => PanelEvent::Close,
                ffi::FAR_EVENTS::FE_BREAK => PanelEvent::Break(info.param as ffi::DWORD),
                ffi::FAR_EVENTS::FE_COMMAND => PanelEvent::Command(unsafe { WideString::from_ptr_str(info.param as *const ffi::wchar_t) }.to_string_lossy()),
                ffi::FAR_EVENTS::FE_GOTFOCUS => PanelEvent::GotFocus,
                ffi::FAR_EVENTS::FE_KILLFOCUS => PanelEvent::KillFocus,
                ffi::FAR_EVENTS::FE_CHANGESORTPARAMS => PanelEvent::ChangeSortParams,
            },
            h_panel: info.h_panel
        }
    }
}

pub struct ProcessHostFileInfo
{
    pub h_panel: ffi::HANDLE,
    pub panel_item: Vec<PluginPanelItem>,
    pub op_mode: OPERATION_MODES
}

impl From<&ffi::ProcessHostFileInfo> for ProcessHostFileInfo {
    fn from(info: &ffi::ProcessHostFileInfo) -> Self {
        let mut items = Vec::with_capacity(info.items_number);
        unsafe {
            for raw_item in slice::from_raw_parts(info.panel_item, info.items_number) {
                items.push(PluginPanelItem::from(raw_item));
            }
        }

        ProcessHostFileInfo {
            h_panel: info.h_panel,
            panel_item: items,
            op_mode: info.op_mode,
        }
    }
}

pub struct ProcessPanelInputInfo {
    pub h_panel: ffi::HANDLE,
    pub record: ffi::INPUT_RECORD
}

impl From<&ffi::ProcessPanelInputInfo> for ProcessPanelInputInfo {
    fn from(info: &ffi::ProcessPanelInputInfo) -> Self {
        ProcessPanelInputInfo {
            h_panel: info.h_panel,
            record: info.rec,
        }
    }
}

pub struct PutFilesInfo {
    pub panel: crate::HANDLE,
    pub panel_item: Vec<PluginPanelItem>,
    pub move_file: bool,
    pub src_path: String,
    pub op_mode: OPERATION_MODES
}

impl From<&ffi::PutFilesInfo> for PutFilesInfo {
    fn from(info: &ffi::PutFilesInfo) -> Self {
        let mut items = Vec::with_capacity(info.items_number);
        unsafe {
            for raw_item in slice::from_raw_parts(info.panel_item, info.items_number) {
                items.push(PluginPanelItem::from(raw_item));
            }
        }

        PutFilesInfo {
            panel: info.h_panel,
            panel_item: items,
            move_file: {
                match info.move_file {
                    0 => false,
                    _ => true
                }
            },
            src_path: unsafe { WideString::from_ptr_str(info.src_path) }.to_string_lossy(),
            op_mode: info.op_mode,
        }
    }
}

pub struct SetFindListInfo
{
    pub h_panel: ffi::HANDLE,
    pub panel_item: Vec<PluginPanelItem>
}

impl From<&ffi::SetFindListInfo> for SetFindListInfo {
    fn from(info: &ffi::SetFindListInfo) -> Self {
        let mut items = Vec::with_capacity(info.items_number);
        unsafe {
            for raw_item in slice::from_raw_parts(info.panel_item, info.items_number) {
                items.push(PluginPanelItem::from(raw_item));
            }
        }

        SetFindListInfo {
            h_panel: info.h_panel,
            panel_item: items
        }
    }
}

pub enum Panel {
    Active,
    Passive,
    Handle(HANDLE),
    None
}

impl Into<ffi::HANDLE> for Panel {
    fn into(self) -> ffi::HANDLE {
        match self {
            Panel::Active => ffi::PANEL_ACTIVE,
            Panel::Passive => ffi::PANEL_PASSIVE,
            Panel::Handle(handle) => handle,
            Panel::None => ffi::PANEL_NONE,
        }
    }
}

pub struct PanelInfo {
    pub plugin_handle: ffi::HANDLE,
    pub owner_guid: ffi::GUID,
    pub flags: PANELINFOFLAGS,
    pub items_number: libc::size_t,
    pub selected_items_number: libc::size_t,
    pub panel_rect: ffi::RECT,
    pub current_item: libc::size_t,
    pub top_panel_item: libc::size_t,
    pub view_mode: libc::intptr_t,
    pub panel_type: PANELINFOTYPE,
    pub sort_mode: OPENPANELINFO_SORTMODES
}

pub struct PanelRedrawInfo {
    pub current_item: usize,
    pub top_panel_item: usize,
}

pub(crate) fn init_context() {
    CONTEXT.with(|ref_cell: &RefCell<Option<ctx::Context>>| {
        ref_cell.replace(Some(ctx::Context::default()));
    });
}

#[allow(dead_code)]
fn context<F,R>(func: F) -> R where F: FnOnce(&mut ctx::Context) -> R {
    CONTEXT.with(|ref_cell: &RefCell<Option<ctx::Context>>| {
        return match ref_cell.try_borrow_mut() {
            Ok(mut r) => {
                match *r {
                    Some(ref mut ctx) => {
                        func(ctx)
                    },
                    None => {
                        panic!("Plugin is not initialized")
                    }
                }
            },
            Err(_) => {
                panic!("Fail to acquire Panel API context")
            }
        };
    })
}

pub(crate) fn cleanup_context() {
    CONTEXT.with(|ref_cell: &RefCell<Option<ctx::Context>>| {
        ref_cell.replace(None);
    });
}

#[cfg(feature = "panel_analyse")]
#[no_mangle]
#[export_name="AnalyseW"]
pub extern "system" fn analyse(info: *const ffi::AnalyseInfo) -> crate::HANDLE {
    trace!(">analyse()");
    let call_result = panic::catch_unwind(|| {
        plugin(|plugin: &mut dyn FarPlugin| {
            let info_ref: &ffi::AnalyseInfo = unsafe { &*info };
            assert_eq!(info_ref.struct_size, mem::size_of::<ffi::AnalyseInfo>());

            let buffer = unsafe { slice::from_raw_parts(info_ref.buffer as *const u8, info_ref.buffer_size) };
            match plugin.panel_exports() {
                Some(exports) => exports.analyse(AnalyseInfo {
                    file_name: unsafe { WideString::from_ptr_str(info_ref.file_name) }.to_string_lossy(),
                    buffer,
                    op_mode: info_ref.op_mode,
                }),
                None => unimplemented!()
            }
        })
    });
    let r_val: crate::HANDLE = match call_result {
        Ok(v) => v,
        Err(_) => unimplemented!()
    };
    trace!("<analyse()");
    return r_val;
}

#[cfg(feature = "panel_analyse")]
#[no_mangle]
#[export_name="CloseAnalyseW"]
pub extern "system" fn close_analyse(info: *const ffi::CloseAnalyseInfo) {
    trace!(">analyse()");
    let call_result = panic::catch_unwind(|| {
        plugin(|plugin: &mut dyn FarPlugin| {
            let info_ref: &ffi::CloseAnalyseInfo = unsafe { &*info };
            assert_eq!(info_ref.struct_size, mem::size_of::<ffi::AnalyseInfo>());

            match plugin.panel_exports() {
                Some(exports) => exports.close_analyse(CloseAnalyseInfo {
                    handle: info_ref.handle,
                }),
                None => unimplemented!()
            }
        })
    });
    match call_result {
        Ok(_) => {},
        Err(_) => unimplemented!(),
    };
    trace!("<analyse()");
}

#[allow(unused_variables)]
#[cfg(feature = "panel_find_data")]
#[no_mangle]
#[export_name="GetFindDataW"]
pub extern "system" fn get_find_data(info: *mut ffi::GetFindDataInfo) -> libc::intptr_t {
    trace!(">get_find_data()");
    let call_result = panic::catch_unwind(|| {
        let info_ref = unsafe { &mut *info };
        assert_eq!(info_ref.struct_size, mem::size_of::<ffi::GetFindDataInfo>());
        let handle = info_ref.h_panel;

        let mut result: libc::intptr_t = 0;
        plugin(|plugin: &mut dyn FarPlugin| {
            match plugin.panel_exports() {
                Some(exports) => {
                    let panel_items: Result<&PluginPanelItems> = exports.get_find_data(GetFindDataInfo {
                        handle,
                        op_mode: info_ref.op_mode,
                    });
                    match panel_items {
                        Ok(items_ref) => {
                            context(|ctx: &mut ctx::Context| {
                                let panel = ctx.panel(handle);
                                info_ref.enrich(panel, items_ref);
                                result = 1
                            });
                        },
                        Err(_) => {}
                    }
                },
                None => unimplemented!()
            }
        });

        return result;
    });

    let r_val: libc::intptr_t = match call_result {
        Ok(v) => v,
        Err(_) => 0 as libc::intptr_t
    };
    trace!("<get_find_data()");
    return r_val;
}

#[cfg(feature = "panel_compare")]
#[no_mangle]
#[export_name="CompareW"]
pub extern "system" fn compare(info: *const ffi::CompareInfo) -> libc::intptr_t {
    trace!(">compare()");
    let call_result = panic::catch_unwind(|| {
        let compare_result = plugin(|plugin: &mut dyn FarPlugin| {
            let info_ref = unsafe { &*info };
            assert_eq!(info_ref.struct_size, mem::size_of::<ffi::CompareInfo>());

            match plugin.panel_exports() {
                Some(exports) => exports.compare(CompareInfo {
                    panel: info_ref.h_panel,
                    item1: PluginPanelItem::from(unsafe { &*info_ref.item1 }),
                    item2: PluginPanelItem::from(unsafe { &*info_ref.item2 }),
                    op_mode: info_ref.mode,
                }),
                None => unimplemented!()
            }
        });

        match compare_result {
            Some(Ordering::Less) => -1,
            Some(Ordering::Equal) => 0,
            Some(Ordering::Greater) => 1,
            None => -2
        }
    });
    let r_val: libc::intptr_t = match call_result {
        Ok(v) => v,
        Err(_) => unimplemented!()
    };
    trace!("<compare()");
    return r_val;
}

#[cfg(feature = "panel_delete_files")]
#[no_mangle]
#[export_name="DeleteFilesW"]
pub extern "system" fn delete_files(info: *const ffi::DeleteFilesInfo) -> libc::intptr_t {
    trace!(">delete_files()");
    let call_result = panic::catch_unwind(|| {
        let result: libc::intptr_t;
        let info_ref = unsafe { &*info };
        assert_eq!(info_ref.struct_size, mem::size_of::<ffi::DeleteFilesInfo>());
        let mut items = Vec::with_capacity(info_ref.items_number);
        unsafe {
            for raw_item in slice::from_raw_parts(info_ref.panel_item, info_ref.items_number) {
                items.push(PluginPanelItem::from(raw_item));
            }
        }

        let delete_result = plugin(|plugin: &mut dyn FarPlugin| {
            match plugin.panel_exports() {
                Some(exports) => exports.delete_files(DeleteFilesInfo {
                    panel: info_ref.h_panel,
                    panel_items: items,
                    op_mode: info_ref.op_mode
                }),
                None => unimplemented!()
            }
        });
        match delete_result {
            Ok(_) => result = 1,
            Err(_) => result = 0
        }
        return result;
    });
    let r_val: libc::intptr_t = match call_result {
        Ok(v) => v,
        Err(_) => 0 as libc::intptr_t
    };
    trace!("<delete_files()");
    return r_val;
}

#[cfg(feature = "panel_make_directory")]
#[no_mangle]
#[export_name="MakeDirectoryW"]
pub extern "system" fn make_directory(info: *mut ffi::MakeDirectoryInfo) -> libc::intptr_t {
    trace!(">make_directory()");
    let call_result = panic::catch_unwind(|| {
        context(|ctx: &mut ctx::Context| {
            let result: libc::intptr_t;
            let info_ref = unsafe { &mut *info };
            assert_eq!(info_ref.struct_size, mem::size_of::<ffi::MakeDirectoryInfo>());
            let mut dir_info = MakeDirectoryInfo::from(&*info_ref);

            let panel = ctx.panel(info_ref.h_panel);

            let silent_mode = info_ref.op_mode.contains(OPERATION_MODES::OPM_SILENT);

            let mut update_name = |name: &str| {
                panel.make_directory_name = Some(WideString::from(name));
                info_ref.name = panel.make_directory_name.as_ref().unwrap().as_ptr();
                ReturnCode::Success as libc::intptr_t
            };

            let make_directory_result = plugin(|plugin: &mut dyn FarPlugin| {
                match plugin.panel_exports() {
                    Some(exports) => exports.make_directory(&mut dir_info),
                    None => unimplemented!()
                }
            });
            match make_directory_result {
                Ok(code) => {
                    if !silent_mode {
                        result = update_name(&dir_info.name)
                    } else {
                        result = code as libc::intptr_t;
                    }
                },
                Err(_) => {
                    if !silent_mode {
                        let _ = update_name(&dir_info.name);
                    }
                    result = 0
                }
            }
            return result;
        })
    });

    let r_val: libc::intptr_t = match call_result {
        Ok(v) => v,
        Err(_) => 0 as libc::intptr_t
    };
    trace!("<make_directory()");
    return r_val;
}

#[allow(unused_variables)]
#[cfg(feature = "panel_find_data")]
#[no_mangle]
#[export_name="FreeFindDataW"]
pub extern "system" fn free_find_data(info: *mut ffi::FreeFindDataInfo) {
    trace!(">free_find_data()");
    let result = panic::catch_unwind(|| {
        let info_ref = unsafe { &mut *info };
        assert_eq!(info_ref.struct_size, mem::size_of::<ffi::FreeFindDataInfo>());
        plugin(|plugin: &mut dyn FarPlugin| {
            match plugin.panel_exports() {
                Some(exports) => exports.free_find_data(info_ref.h_panel),
                None => unimplemented!()
            };
        });
    });
    trace!("<free_find_data()");
}

#[allow(unused_variables)]
#[cfg(feature = "panel_get_files")]
#[no_mangle]
#[export_name="GetFilesW"]
pub extern fn get_files(info: *mut ffi::GetFilesInfo) -> libc::intptr_t {
    trace!(">get_files()");
    let call_result = panic::catch_unwind(|| {
        let info_ref: &mut ffi::GetFilesInfo =  unsafe { &mut *info };
        assert_eq!(info_ref.struct_size, mem::size_of::<ffi::GetFilesInfo>());
        let raw_items = unsafe { slice::from_raw_parts_mut(info_ref.panel_item, info_ref.items_number) };
        let mut items: Vec<PluginPanelItem> = Vec::with_capacity(raw_items.len());

        for i in 0..raw_items.len() {
            let item: &mut ffi::PluginPanelItem = raw_items.get_mut(i).unwrap();
            items.push(PluginPanelItem {
                creation_time: item.creation_time,
                last_access_time: item.last_access_time,
                last_write_time: item.last_write_time,
                change_time: item.change_time,
                file_size: item.file_size,
                allocation_size: item.allocation_size,
                file_name: unsafe { WideString::from_ptr_str(item.file_name) }.to_string_lossy(),
                alternate_file_name: if item.alternate_file_name != ptr::null() {
                    Some(unsafe { WideString::from_ptr_str(item.alternate_file_name) }.to_string_lossy())
                } else {
                    None
                },
                description: if item.description != ptr::null() {
                    Some(unsafe { WideString::from_ptr_str(item.description) }.to_string_lossy())
                } else {
                    None
                },
                owner: if item.owner != ptr::null() {
                    Some(unsafe { WideString::from_ptr_str(item.owner) }.to_string_lossy())
                } else {
                    None
                },
                flags: item.flags,
                file_attributes: match FILE_ATTRIBUTES::from_bits(item.file_attributes) {
                    Some(attributes) => attributes,
                    None => {
                        error!("Unknown file attirbute");
                        FILE_ATTRIBUTES::from_bits_truncate(item.file_attributes)
                    }
                },
                number_of_links: item.number_of_links,
                crc32: item.crc32,
            });
        }
        let mut get_files_info = GetFilesInfo {
            panel: info_ref.h_panel,
            panel_items: items,
            items_number: info_ref.items_number,
            move_file: info_ref.move_file != 0,
            dest_path: unsafe { WideString::from_ptr_str(info_ref.dest_path) }.to_string_lossy(),
            op_mode: info_ref.op_mode,
        };

        let get_files_result = plugin(|plugin: &mut dyn FarPlugin| {
            match plugin.panel_exports() {
                Some(exports) => exports.get_files(&mut get_files_info),
                None => unimplemented!()
            }
        });
        let result = match get_files_result {
            Ok(code) => code as libc::intptr_t,
            Err(_) => return 0
        };

        for i in 0..raw_items.len() {
            let raw_item: &mut ffi::PluginPanelItem = raw_items.get_mut(i).unwrap();
            if let Some(item) = get_files_info.panel_items.get(i) {
                raw_item.flags = item.flags;
            }

        }
        return result;
    });

    let r_val: libc::intptr_t = match call_result {
        Ok(v) => v,
        Err(_) => 0 as libc::intptr_t
    };
    trace!("<get_files()");
    return r_val;
}

#[allow(unused_variables)]
#[cfg(feature = "panel_open")]
#[no_mangle]
#[export_name="GetOpenPanelInfoW"]
pub extern "system" fn get_open_panel_info(info: *mut ffi::OpenPanelInfo) {
    trace!(">get_open_panel_info()");
    let call_result = panic::catch_unwind(|| {
        let info_ref = unsafe { &mut *info };
        assert_eq!(info_ref.struct_size, mem::size_of::<ffi::OpenPanelInfo>());

        let h_panel= info_ref.h_panel;
        plugin(|plugin: &mut dyn FarPlugin| {
            let open_panel_info: &OpenPanelInfo = match plugin.panel_exports() {
                Some(exports) => exports.get_open_panel_info(h_panel),
                None => unimplemented!()
            };
            context(|ctx: &mut ctx::Context| {
                info_ref.enrich(ctx, (h_panel, open_panel_info));
            });
        });
    });
    trace!("<get_open_panel_info()");
}

#[allow(unused_variables)]
#[cfg(feature = "panel_close")]
#[no_mangle]
#[export_name="ClosePanelW"]
pub extern "system" fn close_panel(info: *const ffi::ClosePanelInfo) {
    trace!(">close_panel()");
    let call_result = panic::catch_unwind(|| {
        let info_ref = unsafe { &*info };
        assert_eq!(info_ref.struct_size, mem::size_of::<ffi::ClosePanelInfo>());
        context(|ctx: &mut ctx::Context| ctx.remove_panel(info_ref.h_panel));
        plugin(|plugin: &mut dyn FarPlugin| {
            match plugin.panel_exports() {
                Some(exports) => exports.close_panel(info_ref.h_panel),
                None => unimplemented!()
            }
        });
        trace!("|close_panel() ACTIVE PANELS: {}", context(|ctx: &mut ctx::Context| ctx.count()));
    });
    trace!("<close_panel()");
}

#[allow(unused_variables)]
#[cfg(feature = "panel_process_panel_event")]
#[no_mangle]
#[export_name="ProcessPanelEventW"]
pub extern "system" fn process_panel_event(info: *const ffi::ProcessPanelEventInfo) -> libc::intptr_t {
    trace!(">process_panel_event()");
    let call_result = panic::catch_unwind(|| {
        let info_ref = unsafe { &*info };
        assert_eq!(info_ref.struct_size, mem::size_of::<ffi::ProcessPanelEventInfo>());
        let process_panel_event_info = ProcessPanelEventInfo::from(info_ref);

        let process_panel_event_result = plugin(|plugin: &mut dyn FarPlugin| {
            match plugin.panel_exports() {
                Some(exports) => exports.process_panel_event(process_panel_event_info),
                None => unimplemented!()
            }
        });
        return process_panel_event_result;
    });
    let r_val: libc::intptr_t = match call_result {
        Ok(true) => 1,
        Ok(false) => 0,
        Err(_) => 0
    };
    trace!("<process_panel_event()");
    return r_val;
}

#[allow(unused_variables)]
#[cfg(feature = "panel_process_host_file")]
#[no_mangle]
#[export_name="ProcessHostFileW"]
pub extern "system" fn process_host_file(info: *const ffi::ProcessHostFileInfo) -> libc::intptr_t {
    trace!(">process_host_file()");
    let call_result = panic::catch_unwind(|| {
        let info_ref = unsafe { &*info };
        assert_eq!(info_ref.struct_size, mem::size_of::<ffi::ProcessHostFileInfo>());
        let process_host_file_info = ProcessHostFileInfo::from(info_ref);

        let process_host_file_result = plugin(|plugin: &mut dyn FarPlugin| {
            match plugin.panel_exports() {
                Some(exports) => exports.process_host_file(process_host_file_info),
                None => unimplemented!()
            }
        });
        return process_host_file_result;
    });
    let r_val: libc::intptr_t = match call_result {
        Ok(true) => 1,
        Ok(false) => 0,
        Err(_) => 0
    };
    trace!("<process_host_file()");
    return r_val;
}

#[allow(unused_variables)]
#[cfg(feature = "panel_put_files")]
#[no_mangle]
#[export_name="PutFilesW"]
pub extern "system" fn put_files(info: *const ffi::PutFilesInfo) -> libc::intptr_t {
    trace!(">put_files_info()");
    let call_result = panic::catch_unwind(|| {
        let result: libc::intptr_t;
        let info_ref = unsafe { &*info };
        assert_eq!(info_ref.struct_size, mem::size_of::<ffi::PutFilesInfo>());
        let put_files_info = PutFilesInfo::from(info_ref);

        let put_files_result = plugin(|plugin: &mut dyn FarPlugin| {
            match plugin.panel_exports() {
                Some(exports) => exports.put_files(put_files_info),
                None => unimplemented!()
            }
        });
        result = match put_files_result {
            Ok(code) => code as libc::intptr_t,
            Err(_) => 0
        };
        return result;
    });
    let r_val: libc::intptr_t = match call_result {
        Ok(v) => v,
        Err(_) => 0 as libc::intptr_t
    };
    trace!("<put_files_info()");
    return r_val;
}

#[allow(unused_variables)]
#[cfg(feature = "panel_process_panel_input")]
#[no_mangle]
#[export_name="ProcessPanelInputW"]
pub extern "system" fn process_panel_input(info: *const ffi::ProcessPanelInputInfo) -> libc::intptr_t {
    trace!(">process_panel_input()");
    let call_result = panic::catch_unwind(|| {
        let info_ref = unsafe { &*info };
        assert_eq!(info_ref.struct_size, mem::size_of::<ffi::ProcessPanelInputInfo>());
        let process_panel_event_info = ProcessPanelInputInfo::from(info_ref);

        let process_panel_input_result = plugin(|plugin: &mut dyn FarPlugin| {
            match plugin.panel_exports() {
                Some(exports) => exports.process_panel_input(process_panel_event_info),
                None => unimplemented!()
            }
        });
        return process_panel_input_result;
    });
    let r_val: libc::intptr_t = match call_result {
        Ok(true) => 1,
        Ok(false) => 0,
        Err(_) => 0
    };
    trace!("<process_panel_input()");
    return r_val;
}

#[allow(unused_variables)]
#[cfg(feature = "panel_set_directory")]
#[no_mangle]
#[export_name="SetDirectoryW"]
pub extern "system" fn set_directory(info: *const ffi::SetDirectoryInfo) -> libc::intptr_t {
    trace!(">set_directory()");
    let call_result = panic::catch_unwind(|| {
        let info_ref = unsafe { &*info };
        assert_eq!(info_ref.struct_size, mem::size_of::<ffi::SetDirectoryInfo>());
        let mut result: libc::intptr_t = 0;

        let handle = info_ref.h_panel;
        let path: String = unsafe { WideString::from_ptr_str(info_ref.dir) }.to_string_lossy();
        let set_directory_result = plugin(|plugin: &mut dyn FarPlugin| {
            match plugin.panel_exports() {
                Some(exports) => exports.set_directory(handle, &path),
                None => unimplemented!()
            }
        });
        match set_directory_result {
            Ok(_) => {
                context(|ctx: &mut ctx::Context| ctx.panel(handle).set_current_directory(&path));
                result = 1
            },
            Err(_) => {}
        }

        return result;
    });
    let r_val: libc::intptr_t = match call_result {
        Ok(v) => v,
        Err(_) => 0 as libc::intptr_t
    };
    trace!("<set_directory()");
    return r_val;
}

#[allow(unused_variables)]
#[cfg(feature = "panel_set_find_list")]
#[no_mangle]
#[export_name="SetFindListW"]
pub extern "system" fn set_find_list(info: *const ffi::SetFindListInfo) -> libc::intptr_t {
    trace!(">set_find_list()");
    let call_result = panic::catch_unwind(|| {
        let info_ref = unsafe { &*info };
        assert_eq!(info_ref.struct_size, mem::size_of::<ffi::SetFindListInfo>());
        let set_find_list_info = SetFindListInfo::from(info_ref);

        let set_find_list_result = plugin(|plugin: &mut dyn FarPlugin| {
            match plugin.panel_exports() {
                Some(exports) => exports.set_find_list(set_find_list_info),
                None => unimplemented!()
            }
        });
        return set_find_list_result;
    });
    let r_val: libc::intptr_t = match call_result {
        Ok(true) => 1,
        Ok(false) => 0,
        Err(_) => 0
    };
    trace!("<set_find_list()");
    return r_val;
}

pub fn get_dir_list(dir: &str) -> crate::Result<Vec<PluginPanelItem>> {
    trace!(">get_dir_list()");
    let result: crate::Result<Vec<PluginPanelItem>> = far_api(|far_api: &mut ffi::PluginStartupInfo| {
        let mut panel_items: *mut ffi::PluginPanelItem = ptr::null_mut();
        let mut items_number: libc::size_t = 0;

        let dir_ws: WideString = WideString::from(dir);
        let return_code = far_api.get_dir_list(dir_ws.as_ptr(), &mut panel_items, &mut items_number);

        if return_code == 0 {
            return Err(format_err!(""));
        }

        let mut items = Vec::with_capacity(items_number);
        unsafe {
            for raw_item in slice::from_raw_parts(panel_items, items_number) {
                items.push(PluginPanelItem::from(raw_item));
            }
        }

        far_api.free_dir_list(panel_items, items_number);
        return Ok(items);
    });

    trace!("<get_dir_list()");
    return result;
}

pub fn get_plugin_dir_list(plugin_id: ffi::GUID, h_panel: ffi::HANDLE, dir: &str) -> Result<Vec<PluginPanelItem>> {
    trace!(">get_plugin_dir_list()");
    let result: crate::Result<Vec<PluginPanelItem>> = far_api(|far_api: &mut ffi::PluginStartupInfo| {
        let mut panel_items: *mut ffi::PluginPanelItem = ptr::null_mut();
        let mut items_number: libc::size_t = 0;

        let dir_ws: WideString = WideString::from(dir);
        let return_code = far_api.get_plugin_dir_list(&plugin_id, h_panel,dir_ws.as_ptr(), &mut panel_items, &mut items_number);

        if return_code == 0 {
            return Err(format_err!(""));
        }

        let mut items = Vec::with_capacity(items_number);
        unsafe {
            for raw_item in slice::from_raw_parts(panel_items, items_number) {
                items.push(PluginPanelItem::from(raw_item));
            }
        }

        far_api.free_plugin_dir_list(h_panel,panel_items, items_number);
        return Ok(items);
    });

    trace!("<get_plugin_dir_list()");
    return result;
}
