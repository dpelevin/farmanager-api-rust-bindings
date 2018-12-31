use std::mem;
use std::ptr;

use crate::basic::*;
use crate::common::Enrichable;
use crate::common::string::{WideString, WideStringArray};
use crate::ffi;

pub(super) struct GlobalInfoContext {
    guid: ffi::GUID,
    title: WideString,
    description: WideString,
    author: WideString
}

#[allow(dead_code)]
pub(super) struct PluginMenuItem {
    inner: ffi::PluginMenuItem,
    guids: Box<[ffi::GUID]>,
    strings: WideStringArray,
}

impl From<&Vec<MenuItem>> for PluginMenuItem {
    fn from(menu_items: &Vec<MenuItem>) -> Self {
        let mut guids: Vec<ffi::GUID> = Vec::with_capacity(menu_items.len());
        let mut strings: Vec<WideString> = Vec::with_capacity(menu_items.len());

        for item in menu_items.iter() {
            guids.push(item.guid);
            strings.push(WideString::from(item.label.as_str()));
        }

        let guids_slice: Box<[ffi::GUID]> = guids.into_boxed_slice();
        let strings_array: WideStringArray = WideStringArray::from(strings);

        PluginMenuItem {
            inner: ffi::PluginMenuItem {
                guids: guids_slice.as_ptr(),
                strings: strings_array.as_ptr(),
                count: strings_array.len(),
            },
            guids: guids_slice,
            strings: strings_array,
        }
    }
}

impl PluginMenuItem {
    pub(super) fn as_raw(&self) -> &ffi::PluginMenuItem {
        &self.inner
    }
}

#[allow(dead_code)]
pub(super) struct PluginInfoContext {
    flags: ffi::PLUGIN_FLAGS,
    disk_menu: PluginMenuItem,
    plugin_menu: PluginMenuItem,
    plugin_config: PluginMenuItem,
    command_prefix: Option<WideString>
}

impl From<PluginInfo> for PluginInfoContext {
    fn from(plugin_info: PluginInfo) -> Self {
        PluginInfoContext {
            flags: plugin_info.flags,
            disk_menu: PluginMenuItem::from(&plugin_info.disk_menu),
            plugin_menu: PluginMenuItem::from(&plugin_info.plugin_menu),
            plugin_config: PluginMenuItem::from(&plugin_info.plugin_config),
            command_prefix: match &plugin_info.command_prefix {
                Some(prefix) => Some(WideString::from(prefix.as_str())),
                None => None
            }
        }
    }
}

pub(super) struct Context {
    plugin_info: Option<PluginInfoContext>,
    global_info: Option<GlobalInfoContext>
}

impl Context {

    pub(super) fn plugin_guid(&self) -> ffi::GUID {
        self.global_info.as_ref().unwrap_or_else(panic_global_info_uninitialized).guid
    }
}

impl Default for Context {

    fn default() -> Context {
        Context {
            plugin_info: None,
            global_info: None,
        }
    }
}

impl Enrichable<Context, GlobalInfo> for ffi::GlobalInfo {
    fn enrich(&mut self, ctx: &mut Context, src: GlobalInfo) {
        ctx.global_info = Some(GlobalInfoContext {
            guid: src.guid,
            title: WideString::from(src.title.as_str()),
            description: WideString::from(src.description.as_str()),
            author: WideString::from(src.author.as_str()),
        });

        let info_ref = ctx.global_info.as_ref().unwrap_or_else(panic_global_info_uninitialized);

        self.struct_size = mem::size_of::<ffi::GlobalInfo>();
        self.min_far_version = src.min_far_version;
        self.version = src.version;
        self.guid = info_ref.guid;
        self.title = info_ref.title.as_ptr();
        self.description = info_ref.description.as_ptr();
        self.author = info_ref.author.as_ptr();
    }
}

impl Enrichable<Context, PluginInfo> for ffi::PluginInfo {
    fn enrich(&mut self, ctx: &mut Context, src: PluginInfo) {
        ctx.plugin_info = Some(PluginInfoContext::from(src));

        let info_ref = &ctx.plugin_info.as_ref().unwrap_or_else(panic_plugin_info_uninitialized);

        self.flags = info_ref.flags;
        self.disk_menu = *info_ref.disk_menu.as_raw();
        self.plugin_menu = *info_ref.plugin_menu.as_raw();
        self.plugin_config = *info_ref.plugin_config.as_raw();
        self.command_prefix = match info_ref.command_prefix.as_ref() {
            Some(prefix) => prefix.as_ptr(),
            None => ptr::null()
        };
   }
}

fn panic_global_info_uninitialized<'a>() -> &'a GlobalInfoContext {
    panic!("Global info struct is not initialized")
}

fn panic_plugin_info_uninitialized<'a>() -> &'a PluginInfoContext {
    panic!("Plugin info struct is not initialized")
}