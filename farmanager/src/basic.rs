#![allow(non_camel_case_types)]

use std::cell::RefCell;
use std::mem;
use std::panic;
use std::ptr;
use std::slice;

use log::*;

use crate::common;
use crate::common::Enrichable;
use crate::common::string::{WideString, WideStringArray};
use crate::far_api;
use crate::FAR_API;
use crate::FAR_STANDARD_FUNCTIONS;
use crate::FarPlugin;
use crate::ffi;
pub use crate::ffi::FARHELPFLAGS as FARHELPFLAGS;
pub use crate::ffi::FarKey as FarKey;
pub use crate::ffi::FARMENUFLAGS as FARMENUFLAGS;
pub use crate::ffi::FARMESSAGEFLAGS as FARMESSAGEFLAGS;
pub use crate::ffi::INPUTBOXFLAGS as INPUTBOXFLAGS;
pub use crate::ffi::MENUITEMFLAGS as MENUITEMFLAGS;
pub use crate::ffi::OPENPANELINFO_FLAGS as OPENPANELINFO_FLAGS;
pub use crate::ffi::PLUGIN_FLAGS as PLUGIN_FLAGS;
pub use crate::ffi::VersionInfo as VersionInfo;
pub use crate::ffi::VersionStage as VersionStage;
use crate::init;
use crate::plugin;

pub mod ctx;

thread_local! {
    static CONTEXT: RefCell<Option<ctx::Context>> = RefCell::new(None);
}

pub enum OpenFrom {
    LeftDiskMenu,
    PluginsMenu,
    FindList,
    Shortcut(OpenShortcutInfo),
    CommandLine(OpenCommandLineInfo),
    Editor,
    Viewer,
    FilePanel,
    Dialog(OpenDlgPluginData),
    Analyse(OpenAnalyseInfo),
    RightDiskMenu,
    FromMacro,
    LuaMacro
}

pub trait ExportFunctions {

    fn get_global_info(&mut self) -> GlobalInfo;
    #[allow(unused_variables)]
    fn set_startup_info(&mut self, plugin_startup_info: PluginStartupInfo) { }
    fn get_plugin_info(&mut self) -> PluginInfo;
    #[allow(unused_variables)]
    fn open(&mut self, open_from: OpenFrom) -> ffi::HANDLE {
        ptr::null_mut()
    }
    #[allow(unused_variables)]
    fn exit_far(&mut self, exit_info: &ExitInfo) { }
}

pub trait Langpack {

    fn to_message_id(&self) -> isize;
}

pub struct GlobalInfo {
    pub min_far_version: ffi::VersionInfo,
    pub version: ffi::VersionInfo,
    pub guid: ffi::GUID,
    pub title: WideString,
    pub description: WideString,
    pub author: WideString
}

impl Default for GlobalInfo {

    fn default() -> GlobalInfo {
        GlobalInfo {
            min_far_version: ffi::VersionInfo::default(),
            version: ffi::VersionInfo::default(),
            guid: ffi::DEFAULT_GUID,
            title: WideString::new(),
            description: WideString::new(),
            author: WideString::new(),
        }
    }
}

#[derive(Default)]
pub struct PluginInfo {
    pub flags: ffi::PLUGIN_FLAGS,
    pub disk_menu: Vec<MenuItem>,
    pub plugin_menu: Vec<MenuItem>,
    pub plugin_config: Vec<MenuItem>,
    pub command_prefix: Option<WideString>
}

pub struct PluginStartupInfo {
    pub module_name: WideString
}

pub struct MenuItem {
    pub guid: ffi::GUID,
    pub label: WideString
}

pub struct FarMenuItem {
    pub flags: ffi::MENUITEMFLAGS,
    pub text: WideString,
    pub accel_key: FarKey
}

pub struct OpenCommandLineInfo {
    pub command_line: WideString
}

pub struct OpenShortcutInfo {
    pub host_file: Option<WideString>,
    pub shortcut_data: Option<WideString>,
    pub flags: ffi::FAROPENSHORTCUTFLAGS,
}

pub struct OpenDlgPluginData {
    pub h_dlg: ffi::HANDLE
}

pub struct OpenAnalyseInfo {
    pub info: AnalyseInfo,
    pub handle: ffi::HANDLE
}

pub struct AnalyseInfo {
    pub file_name: WideString,
    pub buffer: Vec<u8>,
    pub op_mode: ffi::OPERATION_MODES
}

pub struct ExitInfo;

pub const DIALOG_SEPARATOR: &'static str = "\x01";

pub(crate) fn init_context() {
    CONTEXT.with(|ref_cell: &RefCell<Option<ctx::Context>>| {
        ref_cell.replace(Some(ctx::Context::default()));
    });
}

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
                panic!("Fail to acquire Basic API context")
            }
        };
    })
}

pub(crate) fn cleanup_context() {
    CONTEXT.with(|ref_cell: &RefCell<Option<ctx::Context>>| {
        ref_cell.replace(None);
    });
}

pub(crate) fn plugin_guid() -> crate::GUID {
    context(|ctx: &mut ctx::Context| ctx.plugin_guid())
}

pub fn get_msg(key: &dyn Langpack) -> WideString {
    trace!(">get_msg()");
    let result: WideString;
    result = far_api(|far_api: &mut ffi::PluginStartupInfo| {
        let guid = &context(|ctx: &mut ctx::Context| ctx.plugin_guid());

        let raw_msg: *const ffi::wchar_t = far_api.get_msg(guid, key.to_message_id());
        return unsafe { WideString::from_ptr_str(raw_msg) }
    });

    trace!("<get_msg()");
    return result;
}

pub fn input_box(title: Option<WideString>,
                 sub_title: Option<WideString>,
                 history_name: Option<WideString>,
                 src_text: Option<WideString>,
                 input_length: usize, help_topic: Option<WideString>,
                 flags: ffi::INPUTBOXFLAGS) -> Option<WideString> {
    trace!(">input_box()");
    let guid = &context(|ctx: &mut ctx::Context| ctx.plugin_guid());
    let event_guid: ffi::GUID = common::generate_guid();

    let dest_text_buf: Vec<ffi::wchar_t> = vec![0; input_length];

    let return_code;
    return_code = far_api(|far_api: &mut ffi::PluginStartupInfo| {
        far_api.input_box(guid, &event_guid,
                          title.as_ref().map_or(ptr::null(),|s| s.as_ptr()),
                          sub_title.as_ref().map_or(ptr::null(),|s| s.as_ptr()),
                          history_name.as_ref().map_or(ptr::null(),|s| s.as_ptr()),
                          src_text.as_ref().map_or(ptr::null(),|s| s.as_ptr()),
                          dest_text_buf.as_ptr(),
                          input_length,
                          help_topic.as_ref().map_or(ptr::null(),|s| s.as_ptr()),
                          flags)
    });

    let result = if return_code != 0 {
        Some(WideString::from(dest_text_buf.as_slice()))
    } else {
        None
    };
    trace!("<input_box()");
    return result;
}

pub fn menu(x: Option<isize>, y: Option<isize>, max_height: Option<isize>, flags: ffi::FARMENUFLAGS,
            title: Option<&str>, bottom: Option<&str>, help_topic: Option<&str>,
            break_keys: Option<Vec<FarKey>>,
            items: Vec<FarMenuItem>) -> (Option<usize>, Option<usize>) {
    trace!(">menu()");
    let guid = &context(|ctx: &mut ctx::Context| ctx.plugin_guid());
    let event_guid: ffi::GUID = common::generate_guid();

    let result = far_api(|far_api: &mut ffi::PluginStartupInfo| {
        let title_ws = title.map(WideString::from);
        let bottom_ws = bottom.map(WideString::from);
        let help_topic_ws = help_topic.map(WideString::from);
        let mut break_keys_raw = break_keys.unwrap_or(Vec::new()).clone();
        break_keys_raw.push(FarKey {
            virtual_key_code: 0,
            control_key_state: 0,
        });

        let mut menu_item_labels: Vec<WideString> = Vec::new();
        let mut items_raw = Vec::new();
        for item in items {
            menu_item_labels.push(WideString::from(item.text));
            items_raw.push(ffi::FarMenuItem {
                flags: item.flags,
                text: menu_item_labels[menu_item_labels.len() - 1].as_ptr(),
                accel_key: item.accel_key,
                user_data: 0,
                reserved: [0; 2],
            });
        }
        let break_code: libc::intptr_t = -1;
        let result;
        result = far_api.menu(guid, &event_guid, x.unwrap_or(-1), y.unwrap_or(-1),
                                max_height.unwrap_or(0),
                                flags,
                                title_ws.as_ref().map_or(ptr::null(), |s| s.as_ptr()),
                                bottom_ws.as_ref().map_or(ptr::null(), |s| s.as_ptr()),
                                help_topic_ws.as_ref().map_or(ptr::null(), |s| s.as_ptr()),
                                break_keys_raw.as_ptr(),
                                &break_code,
                                items_raw.as_ptr(),
                                items_raw.len());
        let result = (if result == -1 { None } else { Some(result as usize) },
                      if break_code == -1 { None } else { Some(break_code as usize) });
        return result;
    });
    trace!("<menu()");
    return result;
}

pub fn show_help(module_name: &str, topic: Option<&str>, flags: ffi::FARHELPFLAGS) -> bool {
    trace!(">show_help()");
    let module_name_ws = WideString::from(module_name);
    let topic_ws = topic.map(WideString::from);
    let result = far_api(|far_api: &mut ffi::PluginStartupInfo| {
        far_api.show_help(module_name_ws.as_ptr(),
                          topic_ws.as_ref().map_or(ptr::null(), |s| s.as_ptr()),
                          flags) == ffi::TRUE
    });
    trace!("<show_help()");
    return result;
}

pub enum MessageItems {
    Lines(Vec<WideString>),
    AllInOne(WideString)
}

pub fn message(flags: ffi::FARMESSAGEFLAGS, help_topic: Option<&WideString>,
               items: MessageItems, buttons_number: usize) -> Option<usize> {
    trace!(">message()");
    let result = far_api(|far_api: &mut ffi::PluginStartupInfo| {
        let guid = &context(|ctx: &mut ctx::Context| ctx.plugin_guid());
        let event_guid: ffi::GUID = common::generate_guid();

        let result: isize;
        match items {
            MessageItems::Lines(lines) => {
                let wlines = WideStringArray::from(lines);
                result = far_api.message(guid, &event_guid,
                                         flags - ffi::FARMESSAGEFLAGS::FMSG_ALLINONE,
                                         help_topic.as_ref().map_or(ptr::null(), |s| s.as_ptr()),
                                         wlines.as_ptr(),
                                         wlines.len(),
                                         buttons_number as isize);
            },
            MessageItems::AllInOne(line) => {
                result = far_api.message(guid, &event_guid,
                                         flags | ffi::FARMESSAGEFLAGS::FMSG_ALLINONE,
                                         help_topic.as_ref().map_or(ptr::null(), |s| s.as_ptr()),
                                         line.as_ptr() as *const *const ffi::wchar_t,
                                         0,
                                         buttons_number as isize);
            },
        };
        return result;
    });
    trace!("<message()");
    return if result == -1 { None } else { Some(result as usize) };
}

pub fn get_global_info(plugin_instance: Box<dyn FarPlugin>, info: *mut ffi::GlobalInfo) {
    trace!(">get_global_info()");
    let result = panic::catch_unwind(|| {
        init(plugin_instance);

        let global_info= plugin(|plugin: &mut dyn FarPlugin| {
            plugin.basic_exports().get_global_info()
        });
        let info_ref: &mut ffi::GlobalInfo =  unsafe { &mut *info };

        context(|ctx: &mut ctx::Context| {
            info_ref.enrich(ctx, global_info);
        });
    });
    if result.is_err() {
        error!("Oups! Something went wrong during the get_global_info() method call.");
    }
    trace!("<get_global_info()");
}

#[no_mangle]
#[export_name="SetStartupInfoW"]
pub extern "system" fn set_startup_info(plugin_startup_info: *const ffi::PluginStartupInfo) {
    trace!(">register_plugin()");
    let module_name_ws: WideString;
    module_name_ws = unsafe { WideString::from_ptr_str((*plugin_startup_info).module_name) };

    FAR_API.with(|ref_cell: &RefCell<Option<*mut ffi::PluginStartupInfo>>| {
        let far_api = Box::new(unsafe { *plugin_startup_info });
        ref_cell.replace(Some(Box::into_raw(far_api)));
    });

    FAR_STANDARD_FUNCTIONS.with(|ref_cell: &RefCell<Option<*mut ffi::FarStandardFunctions>>| {
        let far_standard_functions = Box::new(unsafe { *((*plugin_startup_info).far_standard_functions) });
        ref_cell.replace(Some(Box::into_raw(far_standard_functions)));
    });

    FAR_API.with(|ref_cell: &RefCell<Option<*mut ffi::PluginStartupInfo>>| {
        match *ref_cell.borrow_mut() {
            Some(far_api) => {
                FAR_STANDARD_FUNCTIONS.with(|ref_cell: &RefCell<Option<*mut ffi::FarStandardFunctions>>| {
                    match *ref_cell.borrow_mut() {
                        Some(far_standard_functions) => {
                            (unsafe { &mut *far_api }).far_standard_functions = far_standard_functions;
                        },
                        None => panic!("Plugin is not initialized")
                    }
                });
            },
            None => panic!("Plugin is not initialized")
        };
    });
    plugin(|plugin: &mut dyn FarPlugin| {
        plugin.basic_exports().set_startup_info(PluginStartupInfo {
            module_name: module_name_ws
        });
    });
    trace!("<register_plugin()");
}

#[no_mangle]
#[export_name="GetPluginInfoW"]
pub extern "system" fn get_plugin_info(info: *mut ffi::PluginInfo) {
    trace!(">get_plugin_info()");

    let plugin_info = plugin(|plugin: &mut dyn FarPlugin| {
        plugin.basic_exports().get_plugin_info()
    });

    context(|ctx: &mut ctx::Context| {
        unsafe { &mut (*info) }.enrich(ctx, plugin_info);
    });
    trace!("<get_plugin_info()");
}

#[allow(unused_variables)]
#[no_mangle]
#[export_name="OpenW"]
pub extern "system" fn open(info: *const ffi::OpenInfo) -> ffi::HANDLE {
    trace!(">open()");
    let result = panic::catch_unwind(|| {
        let open_info = unsafe { &(*info) };
        assert_eq!(open_info.struct_size, mem::size_of::<ffi::OpenInfo>());
        match open_info.open_from {
            ffi::OPENFROM::OPEN_LEFTDISKMENU => {
                plugin(|plugin: &mut dyn FarPlugin| {
                    plugin.basic_exports().open(OpenFrom::LeftDiskMenu)
                })
            },
            ffi::OPENFROM::OPEN_PLUGINSMENU => {
                plugin(|plugin: &mut dyn FarPlugin| {
                    plugin.basic_exports().open(OpenFrom::PluginsMenu)
                })
            },
            ffi::OPENFROM::OPEN_FINDLIST => {
                plugin(|plugin: &mut dyn FarPlugin| {
                    plugin.basic_exports().open(OpenFrom::FindList)
                })
            },
            ffi::OPENFROM::OPEN_SHORTCUT => {
                let data: &ffi::OpenShortcutInfo = unsafe {
                    let raw_data = (*info).data as *const ffi::OpenShortcutInfo;
                    &*raw_data
                };
                plugin(|plugin: &mut dyn FarPlugin| {
                    let host_file = if data.host_file != ptr::null() {
                        Some(unsafe { WideString::from_ptr_str(data.host_file) })
                    } else {
                        None
                    };
                    let shortcut_data = if data.host_file != ptr::null() {
                        Some(unsafe { WideString::from_ptr_str(data.shortcut_data) })
                    } else {
                        None
                    };
                    plugin.basic_exports().open(OpenFrom::Shortcut(OpenShortcutInfo {
                        host_file,
                        shortcut_data,
                        flags: (*data).flags
                    }))
                })
            },
            ffi::OPENFROM::OPEN_COMMANDLINE => {
                let data: &ffi::OpenCommandLineInfo = unsafe {
                    let raw_data = (*info).data as *const ffi::OpenCommandLineInfo;
                    &*raw_data
                };
                plugin(|plugin: &mut dyn FarPlugin| {
                    plugin.basic_exports().open(OpenFrom::CommandLine(OpenCommandLineInfo {
                        command_line: unsafe { WideString::from_ptr_str(data.command_line) }
                    }))
                })
            },
            ffi::OPENFROM::OPEN_EDITOR => {
                plugin(|plugin: &mut dyn FarPlugin| {
                    plugin.basic_exports().open(OpenFrom::Editor)
                })
            },
            ffi::OPENFROM::OPEN_VIEWER => {
                plugin(|plugin: &mut dyn FarPlugin| {
                    plugin.basic_exports().open(OpenFrom::Viewer)
                })
            },
            ffi::OPENFROM::OPEN_FILEPANEL => {
                plugin(|plugin: &mut dyn FarPlugin| {
                    plugin.basic_exports().open(OpenFrom::FilePanel)
                })
            },
            ffi::OPENFROM::OPEN_DIALOG => {
                let data: &ffi::OpenDlgPluginData = unsafe {
                    let raw_data = (*info).data as *const ffi::OpenDlgPluginData;
                    &*raw_data
                };
                plugin(|plugin: &mut dyn FarPlugin| {
                    plugin.basic_exports().open(OpenFrom::Dialog(OpenDlgPluginData {
                        h_dlg: data.h_dlg
                    }))
                })
            },
            ffi::OPENFROM::OPEN_ANALYSE => {
                plugin(|plugin: &mut dyn FarPlugin| {
                    let data: &ffi::OpenAnalyseInfo = unsafe {
                        let raw_data = (*info).data as *const ffi::OpenAnalyseInfo;
                        &*raw_data
                    };
                    let analyse_info: &ffi::AnalyseInfo = unsafe {
                        let analyse_info = data.info as *const ffi::AnalyseInfo;
                        &*analyse_info
                    };

                    let size: usize = analyse_info.buffer_size;
                    let raw_bytes: &[u8] = unsafe { slice::from_raw_parts(analyse_info.buffer as *const u8, size) };

                    let mut bytes: Vec<u8> = vec![0; size];
                    {
                        (&mut bytes).clone_from_slice(raw_bytes);
                    }
                    plugin.basic_exports().open(OpenFrom::Analyse(OpenAnalyseInfo {
                        info: AnalyseInfo {
                            file_name: unsafe { WideString::from_ptr_str(analyse_info.file_name) },
                            buffer: bytes,
                            op_mode: analyse_info.op_mode
                        },
                        handle: data.handle
                    }))
                })
            },
            ffi::OPENFROM::OPEN_RIGHTDISKMENU => {
                plugin(|plugin: &mut dyn FarPlugin| {
                    plugin.basic_exports().open(OpenFrom::RightDiskMenu)
                })
            },
            ffi::OPENFROM::OPEN_FROMMACRO => {
                unimplemented!();
            },
            ffi::OPENFROM::OPEN_LUAMACRO => {
                plugin(|plugin: &mut dyn FarPlugin| {
                    plugin.basic_exports().open(OpenFrom::LuaMacro)
                })
            }
        }
    });

    let r_val: ffi::HANDLE = match result {
        Ok(v) => v,
        Err(_) => 0 as ffi::HANDLE
    };
    trace!("<open()");
    return r_val;
}

#[allow(unused_variables)]
#[no_mangle]
#[export_name="ExitFARW"]
pub extern "system" fn exit_far(info: *const ffi::ExitInfo) {
    trace!(">exit_far()");
    plugin(|plugin: &mut dyn FarPlugin| plugin.basic_exports().exit_far(&ExitInfo));
    crate::destroy();
    trace!("<exit_far()");
}
