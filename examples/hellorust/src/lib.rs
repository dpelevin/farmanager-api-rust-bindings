#![warn(bare_trait_objects)]

mod lng;

use log::*;
use farmanager::FarPlugin;
use farmanager::*;
use crate::lng::Lng;
use std::io;
use std::ptr;
use std::str;
use simplelog::WriteLogger;
use simplelog::LevelFilter;
use simplelog::Config;
use widestring::WideCString;

plugin!(Plugin);

struct Plugin {
    guid: GUID
}

impl Plugin {
    fn new() -> Plugin {
        init_logger();
        Plugin {
            guid: GUID {
                Data1: 0xdb9dd0f1,
                Data2: 0x8687,
                Data3: 0x4aab,
                Data4: [0x80, 0x4c, 0x31, 0x60, 0x98, 0x53, 0x0e, 0x61]
            }
        }
    }
}

impl FarPlugin for Plugin {

    fn basic_exports(&mut self) -> &mut dyn basic::ExportFunctions {
        self
    }
}

impl basic::ExportFunctions for Plugin {

    fn get_global_info(&mut self) -> basic::GlobalInfo {
        basic::GlobalInfo {
            min_far_version: basic::VersionInfo {
                    major: FARMANAGERVERSION_MAJOR,
                    minor: FARMANAGERVERSION_MINOR,
                    revision: FARMANAGERVERSION_REVISION,
                    build: FARMANAGERVERSION_BUILD,
                    stage: FARMANAGERVERSION_STAGE
                },
            version: basic::VersionInfo {
                    major: 0,
                    minor: 0,
                    revision: 1,
                    build: 1,
                    stage: basic::VersionStage::VS_ALPHA
                },
            guid: self.guid,
            title: String::from("HelloRust"),
            description: String::from("Hello world plugin written in Rust"),
            author: String::from("Dmitry Pelevin <dpelevin@gmail.com>"),
        }
    }

    fn get_plugin_info(&mut self) -> basic::PluginInfo {
        basic::PluginInfo {
            flags: basic::PLUGIN_FLAGS::PF_NONE,
            command_prefix: None,
            plugin_menu: vec!(basic::MenuItem {
                guid: GUID {
                    Data1: 0x788f13f7,
                    Data2: 0x9133,
                    Data3: 0x4106,
                    Data4: [0x86, 0x82, 0xb1, 0xbf, 0x45, 0xa6, 0xd3, 0xa6]
                },
                label: basic::get_msg(&Lng::MenuItemTitle)
            }),
            disk_menu: Vec::new(),
            plugin_config: Vec::new()
        }
    }

    fn open(&mut self, open_from: basic::OpenFrom) -> HANDLE {
        trace!(">open_from_plugins_menu()");
        match open_from {
            basic::OpenFrom::PluginsMenu => {
                basic::message(basic::FARMESSAGEFLAGS::FMSG_LEFTALIGN, None,
                               basic::MessageItems::Lines(vec!(
                                   basic::get_msg(&Lng::MessageTitle),
                                   basic::get_msg(&Lng::MessageLine0),
                                   basic::get_msg(&Lng::MessageLine1),
                                   basic::get_msg(&Lng::MessageLine2),
                                   basic::get_msg(&Lng::MessageLine3),
                                   basic::DIALOG_SEPARATOR.to_string(),
                                   basic::get_msg(&Lng::MessageButton)
                               )), 1);
            },
            _ => {}
        };
        trace!("<open_from_plugins_menu()");
        return ptr::null_mut();
    }
}

fn init_logger() {
    WriteLogger::init(LevelFilter::Trace, Config::default(), io::LineWriter::new(Logger)).unwrap();
}

struct Logger;

impl io::Write for Logger {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        unsafe {
            let message = str::from_utf8(&buf).unwrap();
            kernel32::OutputDebugStringW(WideCString::from_str(message).unwrap().as_ptr());
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
