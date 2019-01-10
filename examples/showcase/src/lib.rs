#![warn(bare_trait_objects)]

use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;
use std::io;
use std::os::windows::fs::MetadataExt;
use std::path::PathBuf;
use std::ptr;
use std::str;

use log::*;
use simplelog::Config;
use simplelog::LevelFilter;
use simplelog::WriteLogger;
use widestring::WideCString;
use winapi::um::winuser;

use farmanager::*;
use farmanager::FarPlugin;

use crate::lng::Lng;
use std::rc::Rc;

mod lng;

struct PanelState {
    root: PathBuf,
    path: PathBuf,
    open_panel_info: panel::OpenPanelInfo,
    panel_items: Option<panel::PluginPanelItems>,
    make_directory_name: Option<Rc<WideString>>,
}

impl PanelState {

    fn current_path(&self) -> PathBuf {
        let result = self.root.clone();
        let resolved_path = result.join(&self.path);
        if let Ok(path) = resolved_path.canonicalize() {
            path
        } else {
            resolved_path
        }
    }

    fn apply_path_segment(&mut self, segment: PathBuf) {
        self.path.push(segment);
    }
}

struct TestDialog {

}

impl dialog::FarDialog for TestDialog {
    fn dlg_proc(&mut self, h_dlg: farmanager::HANDLE, msg: dialog::FarMessage) -> isize {
        trace!(">dlg_proc()");
        let result: isize = match msg {
            dialog::FarMessage::DnBtnClick { id, state } => {
                let text = WideString::from(format!("{}\nButton: {}\nState: {}\n{}\n{}",
                                                  "DnBtnClick".to_string(),
                                                  id,
                                                  state,
                                                  basic::DIALOG_SEPARATOR.to_string(),
                                                  basic::get_msg(&Lng::MessageButton)));
                basic::message(basic::FARMESSAGEFLAGS::FMSG_LEFTALIGN | basic::FARMESSAGEFLAGS::FMSG_ALLINONE,
                               None, basic::MessageItems::AllInOne(text), 1);
                1
            },
            _ => {
                // TODO uncomment as support for all FarMessages is implemented
                // dialog::def_dlg_proc(h_dlg, msg)
                0
            },
        };
        trace!("<dlg_proc()");
        return result;
    }
}

plugin!(Plugin);

struct Plugin {
    guid: GUID,
    module_name: WideString,
    panels: HashMap<HANDLE, PanelState>,
    #[allow(dead_code)]
    objects: Option<Vec<String>>
}

impl Plugin {

    fn new() -> Plugin {
        init_logger();
        Plugin {
            guid: GUID {
                Data1: 0x9c4a84dc,
                Data2: 0xa2e0,
                Data3: 0x43ec,
                Data4: [0xb7, 0x87, 0x17, 0xf2, 0x9b, 0x3, 0x89, 0xaf]
            },
            module_name: WideString::new(),
            panels: HashMap::new(),
            objects: None
        }
    }

    fn showcase(&mut self) {
        let (selected_item, _) = basic::menu(None, None, None,
            basic::FARMENUFLAGS::FMENU_AUTOHIGHLIGHT,
            Some("Showcase"), None, None, None,
            vec!(basic::FarMenuItem {
                            flags: basic::MENUITEMFLAGS::MIF_SELECTED,
                            text: WideString::from("Basic API"),
                            accel_key: basic::FarKey {
                                virtual_key_code: 0,
                                control_key_state: 0,
                            }
                        },
                basic::FarMenuItem {
                            flags: basic::MENUITEMFLAGS::MIF_NONE,
                            text: WideString::from("Panel API"),
                            accel_key: basic::FarKey {
                                virtual_key_code: winuser::VK_F4 as u16,
                                control_key_state: 0,
                            }
                        },
                basic::FarMenuItem {
                            flags: basic::MENUITEMFLAGS::MIF_NONE,
                            text: WideString::from("Editor API"),
                            accel_key: basic::FarKey {
                                virtual_key_code: 0,
                                control_key_state: 0,
                            }
                        },
                basic::FarMenuItem {
                            flags: basic::MENUITEMFLAGS::MIF_NONE,
                            text: WideString::from("Viewer API"),
                            accel_key: basic::FarKey {
                                virtual_key_code: 0,
                                control_key_state: 0,
                            }
                        },
                basic::FarMenuItem {
                            flags: basic::MENUITEMFLAGS::MIF_NONE,
                            text: WideString::from("Dialog API"),
                            accel_key: basic::FarKey {
                                virtual_key_code: 0,
                                control_key_state: 0,
                            }
                        },
                basic::FarMenuItem {
                            flags: basic::MENUITEMFLAGS::MIF_NONE,
                            text: WideString::from("Settings API"),
                            accel_key: basic::FarKey {
                                virtual_key_code: 0,
                                control_key_state: 0,
                            }
                        },
                basic::FarMenuItem {
                            flags: basic::MENUITEMFLAGS::MIF_NONE,
                            text: WideString::from("Plugin Manager API"),
                            accel_key: basic::FarKey {
                                virtual_key_code: 0,
                                control_key_state: 0,
                            }
                        },
                basic::FarMenuItem {
                            flags: basic::MENUITEMFLAGS::MIF_NONE,
                            text: WideString::from("Miscellaneous API"),
                            accel_key: basic::FarKey {
                                virtual_key_code: 0,
                                control_key_state: 0,
                            }
                        },
                basic::FarMenuItem {
                            flags: basic::MENUITEMFLAGS::MIF_NONE,
                            text: WideString::from("Macro API"),
                            accel_key: basic::FarKey {
                                virtual_key_code: 0,
                                control_key_state: 0,
                            }
                        }
            )
        );

        if let Some(selected_item_val) = selected_item {
            match selected_item_val {
                0 => { self.basic_api(); self.showcase(); },
                1 => { self.panel_api(); self.showcase(); },
                2 => { self.editor_api(); self.showcase(); },
                3 => { self.viewer_api(); self.showcase(); },
                4 => { self.dialog_api(); self.showcase(); },
                7 => { self.misc_api(); self.showcase(); },
                _ => crate::unimplemented_api()
            }
        }
    }

    pub fn basic_api(&mut self) {
        let (selected_item, _) = basic::menu(None, None, None,
            basic::FARMENUFLAGS::FMENU_AUTOHIGHLIGHT,
            Some("Basic API"), None, None, None,
            vec!(basic::FarMenuItem {
                            flags: basic::MENUITEMFLAGS::MIF_SELECTED,
                            text: WideString::from("GetMsg()"),
                            accel_key: basic::FarKey {
                                virtual_key_code: 0,
                                control_key_state: 0,
                            }
                        },
                basic::FarMenuItem {
                            flags: basic::MENUITEMFLAGS::MIF_NONE,
                            text: WideString::from("InputBox()"),
                            accel_key: basic::FarKey {
                                virtual_key_code: winuser::VK_F4 as u16,
                                control_key_state: 0,
                            }
                        },
                basic::FarMenuItem {
                            flags: basic::MENUITEMFLAGS::MIF_NONE,
                            text: WideString::from("Menu()"),
                            accel_key: basic::FarKey {
                                virtual_key_code: 0,
                                control_key_state: 0,
                            }
                        },
                basic::FarMenuItem {
                            flags: basic::MENUITEMFLAGS::MIF_NONE,
                            text: WideString::from("Message()"),
                            accel_key: basic::FarKey {
                                virtual_key_code: 0,
                                control_key_state: 0,
                            }
                        },
                basic::FarMenuItem {
                            flags: basic::MENUITEMFLAGS::MIF_NONE,
                            text: WideString::from("ShowHelp()"),
                            accel_key: basic::FarKey {
                                virtual_key_code: 0,
                                control_key_state: 0,
                            }
                        }
            )
        );

        if let Some(selected_item_val) = selected_item {
            match selected_item_val {
                0 => { self.get_msg(); self.basic_api(); },
                1 => { self.input_box(); self.basic_api(); },
                2 => { self.menu(); self.basic_api(); },
                3 => { self.message(); self.basic_api(); },
                4 => { self.show_help(); self.basic_api(); },
                _ => crate::unimplemented_api()
            }
        }

    }

    fn get_msg(&mut self) {
        basic::message(basic::FARMESSAGEFLAGS::FMSG_MB_OK | basic::FARMESSAGEFLAGS::FMSG_LEFTALIGN,
                       None,
                       basic::MessageItems::Lines(vec!(WideString::new(), basic::get_msg(&Lng::MenuItemTitle))),
                       0);

    }

    fn input_box(&mut self) {
        let input = basic::input_box(Some(WideString::from("Запрос данных")),
                                    Some(WideString::from("Введите строку")),
                                    Some(WideString::from("test_input")),
                                    None/*Some("<placeholder>")*/,
                                    10,
                                    None/*Some("Topic1")*/,
                                    basic::INPUTBOXFLAGS::FIB_NONE);

        let mut lines: Vec<WideString> = vec!(WideString::from(""));
        if input.is_some() {
            lines.push(WideString::from(format!("Input: '{}'", input.unwrap())));
            lines.push(WideString::from("Action: 'Ok'"));
        } else {
            lines.push(WideString::from("Action: 'Cancel'"));
        }
        basic::message(basic::FARMESSAGEFLAGS::FMSG_LEFTALIGN | basic::FARMESSAGEFLAGS::FMSG_MB_OK, None,
                       basic::MessageItems::Lines(lines), 0);
    }

    fn menu(&mut self) {
        let (selected_item, close_key) = basic::menu(Some(1), Some(2), None,
            basic::FARMENUFLAGS::FMENU_NONE,
            Some("title"), Some("bottom"), Some("help_topic"),
            Some(vec!(basic::FarKey {
                        virtual_key_code: winuser::VK_F7 as u16,
                        control_key_state: 0,
                    },basic::FarKey {
                        virtual_key_code: winuser::VK_F8 as u16,
                        control_key_state: 0,
                    }
            )),
            vec!(basic::FarMenuItem {
                            flags: basic::MENUITEMFLAGS::MIF_NONE,
                            text: WideString::from("Item0"),
                            accel_key: basic::FarKey {
                                virtual_key_code: winuser::VK_F3 as u16,
                                control_key_state: 0,
                            }
                        },
                basic::FarMenuItem {
                            flags: basic::MENUITEMFLAGS::MIF_NONE,
                            text: WideString::from("Item1"),
                            accel_key: basic::FarKey {
                                virtual_key_code: winuser::VK_F4 as u16,
                                control_key_state: 0,
                            }
                        },
                basic::FarMenuItem {
                            flags: basic::MENUITEMFLAGS::MIF_NONE,
                            text: WideString::from("Item2"),
                            accel_key: basic::FarKey {
                                virtual_key_code: 0,
                                control_key_state: 0,
                            }
                        }
            )
        );

        let mut lines: Vec<WideString> = vec!(WideString::from("Menu result"));
        if let Some(selected_item_val) = selected_item {
            lines.push(WideString::from(format!("Selected menu item: {}", selected_item_val)));
        }
        if let Some(close_key_val) = close_key {
            lines.push(WideString::from(format!("Close key number: {}", close_key_val)));
        }
        if selected_item.is_none() && close_key.is_none() {
            lines.push(WideString::from("Menu is closed with standard key"));
        }
        basic::message(basic::FARMESSAGEFLAGS::FMSG_MB_OK | basic::FARMESSAGEFLAGS::FMSG_LEFTALIGN,
                       None,
                       basic::MessageItems::Lines(lines),
                       0);
    }

    fn message(&mut self) {
        basic::message(basic::FARMESSAGEFLAGS::FMSG_LEFTALIGN, None, basic::MessageItems::Lines(vec!(
            basic::get_msg(&Lng::MessageTitle),
            basic::get_msg(&Lng::MessageLine0),
            basic::get_msg(&Lng::MessageLine1),
            basic::get_msg(&Lng::MessageLine2),
            basic::get_msg(&Lng::MessageLine3),
            WideString::from(basic::DIALOG_SEPARATOR),
            basic::get_msg(&Lng::MessageButton)
            )), 1);
        basic::message(basic::FARMESSAGEFLAGS::FMSG_LEFTALIGN | basic::FARMESSAGEFLAGS::FMSG_ALLINONE,
            None, basic::MessageItems::AllInOne(
            WideString::from(format!("{}\n{}\n{}\n{}\n{}\n{}\n{}",
                basic::get_msg(&Lng::MessageTitleAllInOne),
                basic::get_msg(&Lng::MessageLine0),
                basic::get_msg(&Lng::MessageLine1),
                basic::get_msg(&Lng::MessageLine2),
                basic::get_msg(&Lng::MessageLine3),
                WideString::from(basic::DIALOG_SEPARATOR),
                basic::get_msg(&Lng::MessageButton)
            ))), 1);
    }

    fn show_help(&mut self) {
        basic::show_help(&self.module_name.to_string_lossy(), Some("Topic1"), basic::FARHELPFLAGS::FHELP_NONE);
    }

    pub fn panel_api(&mut self) {
        let (selected_item, _) = basic::menu(None, None, None,
            basic::FARMENUFLAGS::FMENU_AUTOHIGHLIGHT,
            Some("Panel API"), None, None, None,
             vec!(
                 basic::FarMenuItem {
                     flags: basic::MENUITEMFLAGS::MIF_SELECTED,
                     text: WideString::from("PanelControl(): FCTL_CHECKPANELSEXIST"),
                     accel_key: basic::FarKey {
                         virtual_key_code: 0,
                         control_key_state: 0,
                     },
                 },
                 basic::FarMenuItem {
                     flags: basic::MENUITEMFLAGS::MIF_SELECTED,
                     text: WideString::from("PanelControl(): FCTL_ISACTIVEPANEL"),
                     accel_key: basic::FarKey {
                         virtual_key_code: 0,
                         control_key_state: 0,
                     },
                 },
                 basic::FarMenuItem {
                     flags: basic::MENUITEMFLAGS::MIF_SELECTED,
                     text: WideString::from("PanelControl(): FCTL_CLOSEPANEL"),
                     accel_key: basic::FarKey {
                         virtual_key_code: 0,
                         control_key_state: 0,
                     },
                 },
                 basic::FarMenuItem {
                     flags: basic::MENUITEMFLAGS::MIF_NONE,
                     text: WideString::from("PanelControl(): FCTL_GETPANELINFO"),
                     accel_key: basic::FarKey {
                         virtual_key_code: 0,
                         control_key_state: 0,
                     },
                 },
                 basic::FarMenuItem {
                     flags: basic::MENUITEMFLAGS::MIF_NONE,
                     text: WideString::from("PanelControl(): FCTL_GETCOLUMNTYPES"),
                     accel_key: basic::FarKey {
                         virtual_key_code: 0,
                         control_key_state: 0,
                     },
                 },
                 basic::FarMenuItem {
                     flags: basic::MENUITEMFLAGS::MIF_NONE,
                     text: WideString::from("PanelControl(): FCTL_GETCOLUMNWIDTHS"),
                     accel_key: basic::FarKey {
                         virtual_key_code: 0,
                         control_key_state: 0,
                     },
                 },
                 basic::FarMenuItem {
                     flags: basic::MENUITEMFLAGS::MIF_NONE,
                     text: WideString::from("PanelControl(): FCTL_GETPANELDIRECTORY"),
                     accel_key: basic::FarKey {
                         virtual_key_code: 0,
                         control_key_state: 0,
                     },
                 },
                 basic::FarMenuItem {
                     flags: basic::MENUITEMFLAGS::MIF_NONE,
                     text: WideString::from("PanelControl(): FCTL_GETPANELFORMAT"),
                     accel_key: basic::FarKey {
                         virtual_key_code: 0,
                         control_key_state: 0,
                     },
                 },
                 basic::FarMenuItem {
                     flags: basic::MENUITEMFLAGS::MIF_NONE,
                     text: WideString::from("PanelControl(): FCTL_GETPANELHOSTFILE"),
                     accel_key: basic::FarKey {
                         virtual_key_code: 0,
                         control_key_state: 0,
                     },
                 },
                 basic::FarMenuItem {
                     flags: basic::MENUITEMFLAGS::MIF_NONE,
                     text: WideString::from("PanelControl(): FCTL_GETPANELITEM"),
                     accel_key: basic::FarKey {
                         virtual_key_code: 0,
                         control_key_state: 0,
                     },
                 },
                 basic::FarMenuItem {
                     flags: basic::MENUITEMFLAGS::MIF_NONE,
                     text: WideString::from("PanelControl(): FCTL_GETPANELPREFIX"),
                     accel_key: basic::FarKey {
                         virtual_key_code: 0,
                         control_key_state: 0,
                     },
                 },
                 basic::FarMenuItem {
                     flags: basic::MENUITEMFLAGS::MIF_NONE,
                     text: WideString::from("PanelControl(): FCTL_GETSELECTEDPANELITEM"),
                     accel_key: basic::FarKey {
                         virtual_key_code: 0,
                         control_key_state: 0,
                     },
                 },
                 basic::FarMenuItem {
                     flags: basic::MENUITEMFLAGS::MIF_NONE,
                     text: WideString::from("PanelControl(): FCTL_GETCURRENTPANELITEM"),
                     accel_key: basic::FarKey {
                         virtual_key_code: 0,
                         control_key_state: 0,
                     },
                 },
                 basic::FarMenuItem {
                     flags: basic::MENUITEMFLAGS::MIF_NONE,
                     text: WideString::from("PanelControl(): FCTL_REDRAWPANEL"),
                     accel_key: basic::FarKey {
                         virtual_key_code: 0,
                         control_key_state: 0,
                     },
                 },
                 basic::FarMenuItem {
                     flags: basic::MENUITEMFLAGS::MIF_NONE,
                     text: WideString::from("PanelControl(): FCTL_SETACTIVEPANEL"),
                     accel_key: basic::FarKey {
                         virtual_key_code: 0,
                         control_key_state: 0,
                     },
                 },
                 basic::FarMenuItem {
                     flags: basic::MENUITEMFLAGS::MIF_NONE,
                     text: WideString::from("PanelControl(): FCTL_SETPANELDIRECTORY"),
                     accel_key: basic::FarKey {
                         virtual_key_code: 0,
                         control_key_state: 0,
                     },
                 },
                 basic::FarMenuItem {
                     flags: basic::MENUITEMFLAGS::MIF_NONE,
                     text: WideString::from("PanelControl(): FCTL_BEGINSELECTION/FCTL_SETSELECTION/FCTL_ENDSELECTION"),
                     accel_key: basic::FarKey {
                         virtual_key_code: 0,
                         control_key_state: 0,
                     },
                 },
                 basic::FarMenuItem {
                     flags: basic::MENUITEMFLAGS::MIF_NONE,
                     text: WideString::from("PanelControl(): FCTL_CLEARSELECTION"),
                     accel_key: basic::FarKey {
                         virtual_key_code: 0,
                         control_key_state: 0,
                     },
                 },
                 basic::FarMenuItem {
                     flags: basic::MENUITEMFLAGS::MIF_NONE,
                     text: WideString::from("PanelControl(): FCTL_SETSORTMODE"),
                     accel_key: basic::FarKey {
                         virtual_key_code: 0,
                         control_key_state: 0,
                     },
                 },
                 basic::FarMenuItem {
                     flags: basic::MENUITEMFLAGS::MIF_NONE,
                     text: WideString::from("PanelControl(): FCTL_SETSORTORDER"),
                     accel_key: basic::FarKey {
                         virtual_key_code: 0,
                         control_key_state: 0,
                     },
                 },
                 basic::FarMenuItem {
                     flags: basic::MENUITEMFLAGS::MIF_NONE,
                     text: WideString::from("PanelControl(): FCTL_SETVIEWMODE"),
                     accel_key: basic::FarKey {
                         virtual_key_code: 0,
                         control_key_state: 0,
                     },
                 },
                 basic::FarMenuItem {
                     flags: basic::MENUITEMFLAGS::MIF_NONE,
                     text: WideString::from("PanelControl(): FCTL_UPDATEPANEL"),
                     accel_key: basic::FarKey {
                         virtual_key_code: 0,
                         control_key_state: 0,
                     },
                 },
                 basic::FarMenuItem {
                     flags: basic::MENUITEMFLAGS::MIF_NONE,
                     text: WideString::from("PanelControl(): FCTL_SETDIRECTORIESFIRST"),
                     accel_key: basic::FarKey {
                         virtual_key_code: 0,
                         control_key_state: 0,
                     },
                 },
                 basic::FarMenuItem {
                     flags: basic::MENUITEMFLAGS::MIF_NONE,
                     text: WideString::from("PanelControl(): FCTL_GETCMDLINE"),
                     accel_key: basic::FarKey {
                         virtual_key_code: 0,
                         control_key_state: 0,
                     },
                 },
                 basic::FarMenuItem {
                     flags: basic::MENUITEMFLAGS::MIF_NONE,
                     text: WideString::from("PanelControl(): FCTL_GETCMDLINEPOS"),
                     accel_key: basic::FarKey {
                         virtual_key_code: 0,
                         control_key_state: 0,
                     },
                 },
                 basic::FarMenuItem {
                     flags: basic::MENUITEMFLAGS::MIF_NONE,
                     text: WideString::from("PanelControl(): FCTL_GETCMDLINESELECTION"),
                     accel_key: basic::FarKey {
                         virtual_key_code: 0,
                         control_key_state: 0,
                     },
                 },
                 basic::FarMenuItem {
                     flags: basic::MENUITEMFLAGS::MIF_NONE,
                     text: WideString::from("PanelControl(): FCTL_INSERTCMDLINE"),
                     accel_key: basic::FarKey {
                         virtual_key_code: 0,
                         control_key_state: 0,
                     },
                 },
                 basic::FarMenuItem {
                     flags: basic::MENUITEMFLAGS::MIF_NONE,
                     text: WideString::from("PanelControl(): FCTL_SETCMDLINE"),
                     accel_key: basic::FarKey {
                         virtual_key_code: 0,
                         control_key_state: 0,
                     },
                 },
                 basic::FarMenuItem {
                     flags: basic::MENUITEMFLAGS::MIF_NONE,
                     text: WideString::from("PanelControl(): FCTL_SETCMDLINEPOS"),
                     accel_key: basic::FarKey {
                         virtual_key_code: 0,
                         control_key_state: 0,
                     },
                 },
                 basic::FarMenuItem {
                     flags: basic::MENUITEMFLAGS::MIF_NONE,
                     text: WideString::from("PanelControl(): FCTL_SETCMDLINESELECTION"),
                     accel_key: basic::FarKey {
                         virtual_key_code: 0,
                         control_key_state: 0,
                     },
                 },
                 basic::FarMenuItem {
                     flags: basic::MENUITEMFLAGS::MIF_NONE,
                     text: WideString::from("PanelControl(): FCTL_SETUSERSCREEN"),
                     accel_key: basic::FarKey {
                         virtual_key_code: 0,
                         control_key_state: 0,
                     },
                 },
                 basic::FarMenuItem {
                     flags: basic::MENUITEMFLAGS::MIF_NONE,
                     text: WideString::from("PanelControl(): FCTL_GETUSERSCREEN"),
                     accel_key: basic::FarKey {
                         virtual_key_code: 0,
                         control_key_state: 0,
                     },
                 },
                 basic::FarMenuItem {
                     flags: basic::MENUITEMFLAGS::MIF_NONE,
                     text: WideString::from("FileFilterControl()"),
                     accel_key: basic::FarKey {
                         virtual_key_code: 0,
                         control_key_state: 0,
                     },
                 },
             )
        );

        if let Some(selected_item_val) = selected_item {
            match selected_item_val {
                0 => { self.check_panels_exist(); self.panel_api(); },
                1 => { self.is_active_panel(); self.panel_api(); },
                2 => { self.close_panel(); self.panel_api(); },
                3 => { self.get_panel_info(); self.panel_api(); },
                4 => { self.get_column_types(); self.panel_api(); },
                5 => { self.get_column_widths(); self.panel_api(); },
                6 => { self.get_panel_directory(); self.panel_api(); },
                7 => { self.get_panel_format(); self.panel_api(); },
                8 => { self.get_panel_host_file(); self.panel_api(); },
                9 => { self.get_panel_item(); self.panel_api(); },
                10 => { self.get_panel_prefix(); self.panel_api(); },
                11 => { self.get_selected_panel_item(); self.panel_api(); },
                12 => { self.get_current_panel_item(); self.panel_api(); },
                13 => { self.redraw_panel(); self.panel_api(); },
                14 => { self.set_active_panel(); self.panel_api(); },
                15 => { self.set_panel_directory(); self.panel_api(); },
                16 => { self.set_selection(); self.panel_api(); },
                17 => { self.clear_selection(); self.panel_api(); },
                18 => { self.set_sort_mode(); self.panel_api(); },
                19 => { self.set_sort_order(); self.panel_api(); },
                20 => { self.set_view_mode(); self.panel_api(); },
                21 => { self.update_panel(); self.panel_api(); },
                22 => { self.set_directories_first(); self.panel_api(); },
                23 => { self.get_cmd_line(); self.panel_api(); },
                24 => { self.get_cmd_line_pos(); self.panel_api(); },
                25 => { self.get_cmd_line_selection(); self.panel_api(); },
                26 => { self.insert_cmd_line(); self.panel_api(); },
                27 => { self.set_cmd_line(); self.panel_api(); },
                28 => { self.set_cmd_line_pos(); self.panel_api(); },
                29 => { self.set_cmd_line_selection(); self.panel_api(); },
                30 => { self.set_user_screen(); self.panel_api(); },
                31 => { self.get_user_screen(); self.panel_api(); },
                32 => { self.file_filter_control(); self.panel_api(); },
                _ => crate::unimplemented_api()
            }
        }
    }

    fn check_panels_exist(&mut self) {
        let result = panel::control::check_panels_exist();
        basic::message(basic::FARMESSAGEFLAGS::FMSG_MB_OK,
                       None,
                       basic::MessageItems::AllInOne(WideString::from(format!("\nPanels exist: '{}'", &result))),
                       0);
    }

    fn is_active_panel(&mut self) {
        let result = panel::control::is_active_panel(panel::Panel::Handle(42 as HANDLE));
        basic::message(basic::FARMESSAGEFLAGS::FMSG_MB_OK,
                       None,
                       basic::MessageItems::AllInOne(WideString::from(format!("\nActive panel: '{}'", &result))),
                       0);
    }

    fn close_panel(&mut self) {
        panel::control::close_panel(panel::Panel::Active, None);
    }

    fn get_panel_info(&mut self) {
        let result = panel::control::get_panel_info(panel::Panel::Active);

        let value = match result {
            Ok(value) => value,
            Err(e) => {
                error_dialog(e);
                unimplemented!();
            }
        };

        basic::message(basic::FARMESSAGEFLAGS::FMSG_MB_OK,
                       None,
                       basic::MessageItems::Lines(vec!(WideString::from("Panel info"),
                                                       WideString::from(format!("Items number: {}, selected items number: {}",
                                                               value.items_number, value.selected_items_number)))),
                       0);
    }

    fn get_column_types(&mut self) {
        let result = panel::control::get_column_types(panel::Panel::Active);
        let value = match result {
            Ok(value) => value,
            Err(e) => {
                error_dialog(e);
                unimplemented!();
            }
        };

        let types: Vec<String> = value.iter().map(|s| s.to_string_lossy()).collect();
        basic::message(basic::FARMESSAGEFLAGS::FMSG_MB_OK,
                       None,
                       basic::MessageItems::AllInOne(WideString::from(format!("\nColumn types: {:?}", &types))),
                       0);
    }

    fn get_column_widths(&mut self) {
        let result = panel::control::get_column_widths(panel::Panel::Active);
        let value = match result {
            Ok(value) => value,
            Err(e) => {
                error_dialog(e);
                unimplemented!();
            }
        };

        basic::message(basic::FARMESSAGEFLAGS::FMSG_MB_OK,
                       None,
                       basic::MessageItems::AllInOne(WideString::from(format!("\nColumn widths: {:?}", &value))),
                       0);
    }

    fn get_panel_directory(&mut self) {
        let result = panel::control::get_panel_directory(panel::Panel::Active);
        let value = match result {
            Ok(value) => value,
            Err(e) => {
                error_dialog(e);
                unimplemented!();
            }
        };

        basic::message(basic::FARMESSAGEFLAGS::FMSG_MB_OK,
                       None,
                       basic::MessageItems::Lines(vec!(WideString::from("Current directory"),
                                                       WideString::from(format!("Directory: '{}'", &value.name)),
                                                       WideString::from(format!("File: '{}'", &value.file)))),
                       0);
    }

    fn get_panel_format(&mut self) {
        let result = panel::control::get_panel_format(panel::Panel::Active);
        let value = match result {
            Ok(value) => value,
            Err(e) => {
                error_dialog(e);
                unimplemented!();
            }
        };

        basic::message(basic::FARMESSAGEFLAGS::FMSG_MB_OK,
                       None,
                       basic::MessageItems::AllInOne(WideString::from(format!("\nPanel format: '{}'", value))),
                       0);
    }

    fn get_panel_host_file(&mut self) {
        let result = panel::control::get_panel_host_file(panel::Panel::Active);
        let value = match result {
            Ok(value) => value,
            Err(e) => {
                error_dialog(e);
                unimplemented!();
            }
        };

        basic::message(basic::FARMESSAGEFLAGS::FMSG_MB_OK,
                       None,
                       basic::MessageItems::Lines(vec!(WideString::from("Panel host file"),
                                                       WideString::from(value))),
                       0);
    }

    fn get_panel_item(&mut self) {
        let item_num = basic::input_box(Some(WideString::from("Panel item number")),
                                        None,
                                        None,
                                        None,
                                        10,
                                        None,
                                        basic::INPUTBOXFLAGS::FIB_NONE);

        if item_num.is_none() {
            return;
        }

        let item_num: usize = match item_num.unwrap().to_string_lossy().parse() {
            Ok(value) => value,
            Err(_) => return
        };

        let result = panel::control::get_panel_item(panel::Panel::Active, item_num);
        let value = match result {
            Ok(value) => value,
            Err(e) => {
                error_dialog(e);
                unimplemented!();
            }
        };

        basic::message(basic::FARMESSAGEFLAGS::FMSG_MB_OK,
                       None,
                       basic::MessageItems::Lines(vec!(WideString::from("Panel item"),
                                                       WideString::from(format!("{}", &value.file_name)))),
                       0);
    }

    fn get_panel_prefix(&mut self) {
        let result = panel::control::get_panel_prefix(panel::Panel::Active);
        let value = match result {
            Ok(value) => value,
            Err(e) => {
                error_dialog(e);
                unimplemented!();
            }
        };

        basic::message(basic::FARMESSAGEFLAGS::FMSG_MB_OK,
                       None,
                       basic::MessageItems::AllInOne(WideString::from(format!("\nPanel prefix: '{}'", &value))),
                       0);
    }

    fn get_selected_panel_item(&mut self) {
        let sel_item_num = basic::input_box(Some(WideString::from("Selected item number")),
                                            None,
                                            None,
                                            None,
                                            10,
                                            None,
                                            basic::INPUTBOXFLAGS::FIB_NONE);

        if sel_item_num.is_none() {
            return;
        }
        let item_num: usize = match sel_item_num.unwrap().to_string_lossy().parse() {
            Ok(value) => value,
            Err(_) => return
        };

        let result = panel::control::get_selected_panel_item(panel::Panel::Active, item_num);

        let value = match result {
            Ok(value) => value,
            Err(e) => {
                error_dialog(e);
                unimplemented!();
            }
        };

        basic::message(basic::FARMESSAGEFLAGS::FMSG_MB_OK,
                       None,
                       basic::MessageItems::AllInOne(WideString::from(format!("\nSelected panel item: '{}'", &value.file_name))),
                       0);
    }

    fn get_current_panel_item(&mut self) {
        let result = panel::control::get_current_panel_item(panel::Panel::Active);
        let panel_item = match result {
            Ok(panel_item) => panel_item,
            Err(e) => {
                error_dialog(e);
                unimplemented!();
            }
        };

        basic::message(basic::FARMESSAGEFLAGS::FMSG_MB_OK,
                       None,
                       basic::MessageItems::AllInOne(WideString::from(format!("\nCurrent panel item: '{}'", &panel_item.file_name))),
                       0);
    }

    fn redraw_panel(&mut self) {
        let input_current_item = basic::input_box(Some(WideString::from("Current item")),
                                                  None,
                                                  None,
                                                  None,
                                                  10,
                                                  None,
                                                  basic::INPUTBOXFLAGS::FIB_NONE);

        let input_top_panel_item = basic::input_box(Some(WideString::from("Top panel item")),
                                                    None,
                                                    None,
                                                    None,
                                                    10,
                                                    None,
                                                    basic::INPUTBOXFLAGS::FIB_NONE);

        let current_item: Option<usize> = if let Some(pos) = input_current_item {
            if let Ok(pos) = pos.to_string_lossy().parse() {
                Some(pos)
            } else {
                None
            }
        } else {
            None
        };

        let top_panel_item: Option<usize> = if let Some(pos) = input_top_panel_item {
            if let Ok(pos) = pos.to_string_lossy().parse() {
                Some(pos)
            } else {
                None
            }
        } else {
            None
        };

        if current_item.is_some() && top_panel_item.is_some() {
            let _ = panel::control::redraw_panel(panel::Panel::Active, Some(panel::PanelRedrawInfo {
                current_item: current_item.unwrap(),
                top_panel_item: top_panel_item.unwrap(),
            }));
        } else {
            let _ = panel::control::redraw_panel(panel::Panel::Active, None);
        }
    }

    fn set_active_panel(&mut self) {
        panel::control::set_active_panel(panel::Panel::Passive);
    }

    fn set_panel_directory(&mut self) {
        let input = basic::input_box(Some(WideString::from("Directory")),
                                     None,
                                     None,
                                     None,
                                     10,
                                     None,
                                     basic::INPUTBOXFLAGS::FIB_NONE);

        match input {
            Some(dir) => {
                let _ = panel::control::set_panel_directory(panel::Panel::Active, self.guid, dir, WideString::new());
            },
            None => {},
        }
    }

    fn set_selection(&mut self) {
        let item_num = basic::input_box(Some(WideString::from("Item number to select")),
                                            None,
                                            None,
                                            None,
                                            10,
                                            None,
                                            basic::INPUTBOXFLAGS::FIB_NONE);

        if item_num.is_none() {
            return;
        }
        let item_num: usize = match item_num.unwrap().to_string_lossy().parse::<usize>() {
            Ok(value) => value,
            Err(_) => return
        };

        panel::control::begin_selection(panel::Panel::Active);
        panel::control::set_selection(panel::Panel::Active, item_num, true);
        panel::control::end_selection(panel::Panel::Active);
        panel::control::redraw_panel(panel::Panel::Active, None);
    }

    fn clear_selection(&mut self) {
        let item_num = basic::input_box(Some(WideString::from("Item number to unselect")),
                                        None,
                                        None,
                                        None,
                                        10,
                                        None,
                                        basic::INPUTBOXFLAGS::FIB_NONE);

        if item_num.is_none() {
            return;
        }
        let item_num: usize = match item_num.unwrap().to_string_lossy().parse::<usize>() {
            Ok(value) => value,
            Err(_) => return
        };

        panel::control::begin_selection(panel::Panel::Active);
        panel::control::clear_selection(panel::Panel::Active, item_num);
        panel::control::end_selection(panel::Panel::Active);
        panel::control::redraw_panel(panel::Panel::Active, None);
    }

    fn set_sort_mode(&mut self) {
        panel::control::set_sort_mode(panel::Panel::Active, panel::OPENPANELINFO_SORTMODES::SM_SIZE);
    }

    fn set_sort_order(&mut self) {
        panel::control::set_sort_order(panel::Panel::Active, panel::OPENPANELINFO_SORTORDERS::DESC);
    }

    fn set_view_mode(&mut self) {
        panel::control::set_view_mode(panel::Panel::Active, 9);
    }

    fn update_panel(&mut self) {
        panel::control::update_panel(panel::Panel::Active, false);
    }

    fn set_directories_first(&mut self) {
        panel::control::set_directories_first(panel::Panel::Active, false);
    }

    fn get_cmd_line(&mut self) {
        let result = panel::control::get_cmd_line(panel::Panel::Active);
        let cmd_line = match result {
            Ok(cmd_line) => cmd_line,
            Err(e) => {
                error_dialog(e);
                unimplemented!();
            }
        };

        basic::message(basic::FARMESSAGEFLAGS::FMSG_MB_OK,
                       None,
                       basic::MessageItems::AllInOne(WideString::from(format!("\nCommand line: '{}'", &cmd_line))),
                       0);
    }

    fn get_cmd_line_pos(&mut self) {
        let result = panel::control::get_cmd_line_pos(panel::Panel::Active);
        let pos = match result {
            Ok(pos) => pos,
            Err(e) => {
                error_dialog(e);
                unimplemented!();
            }
        };

        basic::message(basic::FARMESSAGEFLAGS::FMSG_MB_OK,
                       None,
                       basic::MessageItems::AllInOne(WideString::from(format!("\nCommand line cursor position: '{}'", pos))),
                       0);
    }

    fn get_cmd_line_selection(&mut self) {
        let result = panel::control::get_cmd_line_selection(panel::Panel::Active);
        let (sel_start, sel_end) = match result {
            Ok((sel_start, sel_end)) => (sel_start, sel_end),
            Err(e) => {
                error_dialog(e);
                unimplemented!();
            }
        };

        basic::message(basic::FARMESSAGEFLAGS::FMSG_MB_OK,
                       None,
                       basic::MessageItems::AllInOne(WideString::from(format!("Command line selection: [{},{})", sel_start, sel_end))),
                       0);
    }

    fn insert_cmd_line(&mut self) {
        let input = basic::input_box(Some(WideString::from("Text to insert to the command line")),
                                     None,
                                     None,
                                     None,
                                     10,
                                     None,
                                     basic::INPUTBOXFLAGS::FIB_NONE);

        match input {
            Some(text) => {
                let _ = panel::control::insert_cmd_line(panel::Panel::Active, text);
            },
            None => {},
        }
    }

    fn set_cmd_line(&mut self) {
        let input = basic::input_box(Some(WideString::from("Text to set to the command line")),
                                     None,
                                     None,
                                     None,
                                     10,
                                     None,
                                     basic::INPUTBOXFLAGS::FIB_NONE);

        match input {
            Some(text) => {
                let _ = panel::control::set_cmd_line(panel::Panel::Active, text);
            },
            None => {},
        }
    }

    fn set_cmd_line_pos(&mut self) {
        let input = basic::input_box(Some(WideString::from("Text to set to the command line")),
                                     None,
                                     None,
                                     None,
                                     10,
                                     None,
                                     basic::INPUTBOXFLAGS::FIB_NONE);

        match input {
            Some(text) => {
                let pos = text.to_string_lossy().parse();
                match pos {
                    Ok(pos) => {
                        let _ = panel::control::set_cmd_line_pos(panel::Panel::Active, pos);
                    },
                    Err(_) => {},
                }
            },
            None => {},
        }
    }

    fn set_cmd_line_selection(&mut self) {
        let mut reset_selection = false;
        let input_sel_start = basic::input_box(Some(WideString::from("Command line selection start position")),
                                     None,
                                     None,
                                     None,
                                     10,
                                     None,
                                     basic::INPUTBOXFLAGS::FIB_NONE);

        if input_sel_start.is_none() {
            reset_selection = true;
        }

        let input_sel_end = basic::input_box(Some(WideString::from("Command line selection end position")),
                                             None,
                                             None,
                                             None,
                                             10,
                                             None,
                                             basic::INPUTBOXFLAGS::FIB_NONE);

        if input_sel_end.is_none() {
            reset_selection = true;
        }

        let pos_start: Option<usize> = if let Some(pos) = input_sel_start {
            if let Ok(pos) = pos.to_string_lossy().parse() {
                Some(pos)
            } else {
                None
            }
        } else {
            None
        };

        let pos_end: Option<usize> = if let Some(pos) = input_sel_end {
            if let Ok(pos) = pos.to_string_lossy().parse() {
                Some(pos)
            } else {
                None
            }
        } else {
            None
        };

        if !reset_selection && pos_start.is_some() && pos_end.is_some() {
            let _ = panel::control::set_cmd_line_selection(panel::Panel::Active, Some((pos_start.unwrap(), pos_end.unwrap())));
        } else {
            let _ = panel::control::set_cmd_line_selection(panel::Panel::Active, None);
        }
    }

    fn set_user_screen(&mut self) {
        let result = panel::control::set_user_screen(panel::Panel::Active, true);
        let _ = match result {
            Ok(_) => {},
            Err(e) => error_dialog(e)
        };
    }

    fn get_user_screen(&mut self) {
        let result = panel::control::get_user_screen(panel::Panel::Active, true);
        let _ = match result {
            Ok(_) => {},
            Err(e) => error_dialog(e)
        };
    }

    fn file_filter_control(&mut self) {
        let result = panel::filter::file_filter(panel::Panel::Passive, panel::filter::FileFilterType::Panel);
        match result {
            Ok(filter) => {
                filter.starting_to_filter();
                filter.open_filters_menu();
                let result = panel::control::get_current_panel_item(panel::Panel::Active);
                let panel_item = match result {
                    Ok(panel_item) => panel_item,
                    Err(_) => unimplemented!(),
                };
                let is_match = filter.is_file_in_filter(&panel_item);
                debug!("match: {}, {}", is_match, panel_item.file_name);
            },
            Err(_) => {
                basic::message(basic::FARMESSAGEFLAGS::FMSG_MB_OK | basic::FARMESSAGEFLAGS::FMSG_WARNING | basic::FARMESSAGEFLAGS::FMSG_ERRORTYPE,
                               None,
                               basic::MessageItems::AllInOne(WideString::from("FileFilterControl()")),
                               0);
            },
        }
    }

    pub fn editor_api(&mut self) {
        let (selected_item, _) = basic::menu(None, None, None,
            basic::FARMENUFLAGS::FMENU_AUTOHIGHLIGHT,
            Some("Editor API"), None, None, None,
            vec!(basic::FarMenuItem {
                            flags: basic::MENUITEMFLAGS::MIF_SELECTED,
                            text: WideString::from("Editor()"),
                            accel_key: basic::FarKey {
                                virtual_key_code: 0,
                                control_key_state: 0,
                            }
                        },
                basic::FarMenuItem {
                            flags: basic::MENUITEMFLAGS::MIF_NONE,
                            text: WideString::from("EditorControl()"),
                            accel_key: basic::FarKey {
                                virtual_key_code: winuser::VK_F4 as u16,
                                control_key_state: 0,
                            }
                        },
            )
        );

        if let Some(selected_item_val) = selected_item {
            match selected_item_val {
                0 => { self.editor(); self.editor_api(); },
                1 => { crate::unimplemented_api(); self.editor_api(); },
                _ => crate::unimplemented_api()
            }
        }
    }

    fn editor(&mut self) {
        let input = basic::input_box(Some(WideString::from("File to edit")),
                                     None,
                                     None,
                                     None,
                                     256,
                                     None,
                                     basic::INPUTBOXFLAGS::FIB_NONE);

        match input {
            Some(path) => editor::open_editor(path),
            None => {},
        }
    }

    pub fn viewer_api(&mut self) {
        let (selected_item, _) = basic::menu(None, None, None,
            basic::FARMENUFLAGS::FMENU_AUTOHIGHLIGHT,
            Some("Viewer API"), None, None, None,
            vec!(basic::FarMenuItem {
                            flags: basic::MENUITEMFLAGS::MIF_SELECTED,
                            text: WideString::from("Viewer()"),
                            accel_key: basic::FarKey {
                                virtual_key_code: 0,
                                control_key_state: 0,
                            }
                        },
                basic::FarMenuItem {
                            flags: basic::MENUITEMFLAGS::MIF_NONE,
                            text: WideString::from("ViewerControl()"),
                            accel_key: basic::FarKey {
                                virtual_key_code: 0,
                                control_key_state: 0,
                            }
                        },
            )
        );

        if let Some(selected_item_val) = selected_item {
            match selected_item_val {
                0 => { self.viewer(); self.viewer_api(); },
                1 => { crate::unimplemented_api(); self.viewer_api(); },
                _ => crate::unimplemented_api()
            }
        }
    }

    fn viewer(&mut self) {
        let input = basic::input_box(Some(WideString::from("File to view")),
                                     None,
                                     None,
                                     None,
                                     256,
                                     None,
                                     basic::INPUTBOXFLAGS::FIB_NONE);

        match input {
            Some(path) => viewer::open_viewer(path),
            None => {},
        }
    }

    pub fn dialog_api(&mut self) {
        let dialog_guid: ffi::GUID = common::generate_guid();
        let test_dialog: TestDialog = TestDialog { };

        let dialog_items: Vec<dialog::FarDialogItem>;
        dialog_items = vec!(dialog::FarDialogItem::DoubleBox {
            x1: 3,
            y1: 1,
            x2: 36,
            y2: 8,
            flags: dialog::FARDIALOGITEMFLAGS::DIF_NONE,
            title: Some(WideString::from("Dialog")),
        },
        dialog::FarDialogItem::Text {
            x1: 4,
            y: 6,
            x2: 36,
            mask: None,
            flags: dialog::FARDIALOGITEMFLAGS::DIF_SEPARATOR,
            text: None,
        },
        dialog::FarDialogItem::Button {
            x: 12,
            y: 7,
            selected: true,
            flags: dialog::FARDIALOGITEMFLAGS::DIF_NONE,
            text: WideString::from("Ok"),
        },
        dialog::FarDialogItem::Button {
            x: 19,
            y: 7,
            selected: true,
            flags: dialog::FARDIALOGITEMFLAGS::DIF_NONE,
            text: WideString::from("Cancel"),
        });

        match dialog::Dialog::init(self.guid, dialog_guid, -1, -1, 40, 10,
                                     None, dialog_items,
                                     dialog::FARDIALOGFLAGS::FDLG_NONE, test_dialog) {
            Ok(dialog) => dialog.run(),
            Err(_) => {},
        }
    }

    pub fn misc_api(&mut self) {
        let (selected_item, _) = basic::menu(None, None, None,
            basic::FARMENUFLAGS::FMENU_AUTOHIGHLIGHT,
            Some("Miscellaneous API"), None, None, None,
            vec!(basic::FarMenuItem {
                            flags: basic::MENUITEMFLAGS::MIF_SELECTED,
                            text: WideString::from("ColorDialog()"),
                            accel_key: basic::FarKey {
                                virtual_key_code: 0,
                                control_key_state: 0,
                            }
                        }
            )
        );

        if let Some(selected_item_val) = selected_item {
            match selected_item_val {
                0 => { self.color_dialog(); self.misc_api(); },
                1 => crate::unimplemented_api(),
                _ => crate::unimplemented_api()
            }
        }
    }

    fn color_dialog(&mut self) {
        let color = misc::show_color_chooser_dialog(misc::COLORDIALOGFLAGS::CDF_NONE);
        match color {
            Some(color) => {
                let f_rgba: misc::rgba = color.foreground_rgba();
                let b_rgba: misc::rgba = color.background_rgba();
                let fr = f_rgba.r;
                let fg = f_rgba.g;
                let fb = f_rgba.b;
                let fa = f_rgba.a;
                let br = b_rgba.r;
                let bg = b_rgba.g;
                let bb = b_rgba.b;
                let ba = b_rgba.a;
                basic::message(basic::FARMESSAGEFLAGS::FMSG_LEFTALIGN | basic::FARMESSAGEFLAGS::FMSG_ALLINONE,
                               None, basic::MessageItems::AllInOne(
                        WideString::from(format!("{}\nForeground:\nR:{} G:{} B:{} A:{}\nBackground:\nR:{} G:{} B:{} A:{}\n{}\n{}",
                                "Color".to_string(),
                                fr,
                                fg,
                                fb,
                                fa,
                                br,
                                bg,
                                bb,
                                ba,
                                basic::DIALOG_SEPARATOR.to_string(),
                                basic::get_msg(&Lng::MessageButton)
                        ))), 1);
            },
            _ => {
                basic::message(basic::FARMESSAGEFLAGS::FMSG_LEFTALIGN | basic::FARMESSAGEFLAGS::FMSG_ALLINONE,
                               None, basic::MessageItems::AllInOne(
                        WideString::from(format!("{}\n{}\n{}\n{}",
                                "Color".to_string(),
                                "Colors are not selected".to_string(),
                                basic::DIALOG_SEPARATOR.to_string(),
                                basic::get_msg(&Lng::MessageButton)
                        ))), 1);
            }
        }
    }

}

impl FarPlugin for Plugin {

    fn basic_exports(&mut self) -> &mut dyn basic::ExportFunctions {
        self
    }

    fn panel_exports(&mut self) -> Option<&mut dyn panel::ExportFunctions> {
        Some(self)
    }

    fn settings_exports(&mut self) -> Option<&mut dyn settings::ExportFunctions> {
        Some(self)
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
            title: WideString::from("API Showcase"),
            description: WideString::from("API showcase plugin written in Rust"),
            author: WideString::from("Dmitry Pelevin <dpelevin@gmail.com>"),
        }
    }

    #[allow(unused_variables)]
    fn set_startup_info(&mut self, plugin_startup_info: basic::PluginStartupInfo) {
        self.module_name = plugin_startup_info.module_name;
    }

    fn get_plugin_info(&mut self) -> basic::PluginInfo {
        basic::PluginInfo {
            flags: basic::PLUGIN_FLAGS::PF_EDITOR,
            command_prefix: Some(WideString::from("rust")),
            plugin_menu: vec!(basic::MenuItem {
                guid: GUID {
                    Data1: 0x788f13f7,
                    Data2: 0x9133,
                    Data3: 0x4106,
                    Data4: [0x86, 0x82, 0xb1, 0xbf, 0x45, 0xa6, 0xd3, 0xa6]
                },
                label: basic::get_msg(&Lng::MenuItemTitle)
            }),
            disk_menu: vec!(basic::MenuItem {
                guid: GUID {
                    Data1: 0x788f13f8,
                    Data2: 0x9133,
                    Data3: 0x4106,
                    Data4: [0x86, 0x82, 0xb1, 0xbf, 0x45, 0xa6, 0xd3, 0xa6]
                },
                label: basic::get_msg(&Lng::MenuItemTitle)
            }),
            plugin_config: vec!(basic::MenuItem {
                guid: GUID {
                    Data1: 0x788f13f8,
                    Data2: 0x9133,
                    Data3: 0x4106,
                    Data4: [0x86, 0x82, 0xb1, 0xbf, 0x45, 0xa6, 0xd3, 0xa6]
                },
                label: basic::get_msg(&Lng::MenuItemTitle)
            })
        }
    }

    fn open(&mut self, open_from: basic::OpenFrom) -> HANDLE {
        return match open_from {
            basic::OpenFrom::LeftDiskMenu => {
                basic::message(basic::FARMESSAGEFLAGS::FMSG_MB_OK | basic::FARMESSAGEFLAGS::FMSG_ALLINONE,
                               None,
                               basic::MessageItems::AllInOne(WideString::from(format!("\n{}",
                                    basic::get_msg(&Lng::MessageFromLeftDiskMenu)))),
                               0);
                42 as HANDLE
            },
            basic::OpenFrom::PluginsMenu => {
                self.showcase();
                ptr::null_mut()
            },
            basic::OpenFrom::FindList => ptr::null_mut(),
            basic::OpenFrom::Shortcut(_data) => {
                basic::message(basic::FARMESSAGEFLAGS::FMSG_MB_OK | basic::FARMESSAGEFLAGS::FMSG_ALLINONE,
                               None,
                               basic::MessageItems::AllInOne(WideString::from(format!("\nOpened from a shortcut"))),
                               0);
                42 as HANDLE
            },
            basic::OpenFrom::CommandLine(data) => {
                basic::message(basic::FARMESSAGEFLAGS::FMSG_MB_OK,
                               None,
                               basic::MessageItems::Lines(vec!(basic::get_msg(&Lng::MessageTitleCommandline),
                                                                   data.command_line)),
                               0);
                ptr::null_mut()
            },
            basic::OpenFrom::Editor => ptr::null_mut(),
            basic::OpenFrom::Viewer => ptr::null_mut(),
            basic::OpenFrom::FilePanel => ptr::null_mut(),
            basic::OpenFrom::Dialog(_data) => ptr::null_mut(),
            basic::OpenFrom::Analyse(data) => {
                basic::message(basic::FARMESSAGEFLAGS::FMSG_MB_OK,
                               None,
                               basic::MessageItems::Lines(vec!(WideString::new(),
                                                                   basic::get_msg(&Lng::MessageFromAnalyse),
                                                                   data.info.file_name)),
                               0);
                ptr::null_mut()
            },
            basic::OpenFrom::RightDiskMenu => {
                basic::message(basic::FARMESSAGEFLAGS::FMSG_MB_OK | basic::FARMESSAGEFLAGS::FMSG_ALLINONE,
                               None,
                               basic::MessageItems::AllInOne(WideString::from(format!("\n{}",
                                    basic::get_msg(&Lng::MessageFromRightDiskMenu)))),
                               0);
                84 as HANDLE
            },
            basic::OpenFrom::FromMacro => ptr::null_mut(),
            basic::OpenFrom::LuaMacro => ptr::null_mut(),
        };
    }

    #[allow(unused_variables)]
    fn exit_far(&mut self, info: &basic::ExitInfo) {
        trace!(">exit_far()");

        trace!("<exit_far()");
    }

}

impl panel::ExportFunctions for Plugin {

    fn analyse(&mut self, info: panel::AnalyseInfo) -> crate::HANDLE {
        basic::message(basic::FARMESSAGEFLAGS::FMSG_MB_OK,
                       None,
                       basic::MessageItems::Lines(vec!(WideString::from("AnalyseW"),
                                                           WideString::from(info.file_name),
                                                           WideString::from(format!("Data buffer size: {}", info.buffer.len())))),
                       0);
        let result: crate::HANDLE = 1 as crate::HANDLE;//ptr::null_mut();
        result
    }

    fn close_analyse(&mut self, _info: panel::CloseAnalyseInfo) {
        basic::message(basic::FARMESSAGEFLAGS::FMSG_MB_OK | basic::FARMESSAGEFLAGS::FMSG_ALLINONE,
                       None,
                       basic::MessageItems::AllInOne(WideString::from(format!("CloseAnalyseW\n"))),
                       0);
    }

    fn get_open_panel_info(&mut self, handle: HANDLE) -> &panel::OpenPanelInfo {
        trace!(">get_open_panel_info()");
        if !self.panels.contains_key(&handle) {
            let root: PathBuf = dirs::home_dir().unwrap();
            let drive_path: PathBuf = root.clone();
            let cur_dir = WideString::from(root.clone().canonicalize().unwrap().to_str().unwrap());
            let panel_title = WideString::from(format!("rust:{}", &cur_dir));

            self.panels.insert(handle, PanelState {
                root,
                path: PathBuf::new(),
                open_panel_info: panel::OpenPanelInfo {
                    flags: basic::OPENPANELINFO_FLAGS::OPIF_ADDDOTS | basic::OPENPANELINFO_FLAGS::OPIF_USEFREESIZE,
                    host_file: None,
                    cur_dir,
                    format: None,
                    panel_title,
                    info_lines: vec!(panel::InfoPanelLine{
                        text: basic::get_msg(&Lng::PanelMessageLine1),
                        data: basic::get_msg(&Lng::PanelMessageData1),
                        flags: panel::INFOPANELLINE_FLAGS::IPLFLAGS_NONE,
                    },panel::InfoPanelLine{
                        text: basic::get_msg(&Lng::PanelMessageSeparator1),
                        data: WideString::new(),
                        flags: panel::INFOPANELLINE_FLAGS::IPLFLAGS_SEPARATOR,
                    },panel::InfoPanelLine{
                        text: basic::get_msg(&Lng::PanelMessageLine2),
                        data: basic::get_msg(&Lng::PanelMessageData2),
                        flags: panel::INFOPANELLINE_FLAGS::IPLFLAGS_NONE,
                    }),
                    descr_files: None,
                    panel_modes_array: Vec::new(),
                    start_panel_mode: 0,
                    start_sort_mode: panel::OPENPANELINFO_SORTMODES::SM_DEFAULT,
                    start_sort_order: panel::OPENPANELINFO_SORTORDERS::ASC,
                    key_bar: None,
                    shortcut_data: None,
                    free_size: match fs2::free_space(drive_path.as_path()) {
                        Ok(size) => size,
                        Err(_) => 0
                    }
                },
                panel_items: None,
                make_directory_name: None
            });
        }

        let state: &mut PanelState = self.panels.get_mut(&handle).unwrap();

        let cur_dir = WideString::from(state.current_path().to_str().unwrap());
        let panel_title = WideString::from(format!("rust:{}", &cur_dir));

        state.open_panel_info.cur_dir = cur_dir;
        state.open_panel_info.panel_title = panel_title;

        trace!("<get_open_panel_info()");
        &state.open_panel_info
    }

    fn get_find_data(&mut self, info: panel::GetFindDataInfo) -> Result<&panel::PluginPanelItems> {
        trace!(">get_find_data()");
        let state: &mut PanelState = self.panels.get_mut(&info.handle).unwrap();
        let current_path = state.current_path();

        if state.panel_items.is_none() {
            let items: Vec<panel::PluginPanelItem>;
            if let Ok(paths) = fs::read_dir(&current_path) {
                items = paths.filter(|f| f.is_ok())
                    .map(|f| f.unwrap())
                    .map(|f| {
                        if let Ok(metadata) = f.metadata() {
                            let creation_time = metadata.creation_time();
                            let last_access_time = metadata.last_access_time();
                            let last_write_time = metadata.last_write_time();
                            let file_attributes = panel::FILE_ATTRIBUTES::from_bits_truncate(metadata.file_attributes() as usize);
                            panel::PluginPanelItem {
                                creation_time: filetime_from_u64(creation_time),
                                last_access_time: filetime_from_u64(last_access_time),
                                last_write_time: filetime_from_u64(last_write_time),
                                change_time: filetime_from_u64(last_write_time),
                                file_size: metadata.len(),
                                allocation_size: 0,
                                file_name: WideString::from(f.file_name().to_str().unwrap_or("<no_file_name>")),
                                alternate_file_name: None,
                                description: Some(WideString::from(format!("Description for file '{}'", f.file_name().to_str().unwrap_or("<no_file_name>")))),
                                owner: None,
                                flags: panel::PLUGINPANELITEMFLAGS::PPIF_PROCESSDESCR,
                                file_attributes,
                                number_of_links: 0,
                                crc32: 0
                            }
                        } else {
                            panel::PluginPanelItem {
                                creation_time: panel::FILETIME {
                                    dwLowDateTime: 0,
                                    dwHighDateTime: 0,
                                },
                                last_access_time: panel::FILETIME {
                                    dwLowDateTime: 0,
                                    dwHighDateTime: 0,
                                },
                                last_write_time: panel::FILETIME {
                                    dwLowDateTime: 0,
                                    dwHighDateTime: 0,
                                },
                                change_time: panel::FILETIME {
                                    dwLowDateTime: 0,
                                    dwHighDateTime: 0,
                                },
                                file_size: 0,
                                allocation_size: 0,
                                file_name: WideString::from(f.file_name().to_str().unwrap_or("<no_file_name>")),
                                alternate_file_name: None,
                                description: None,
                                owner: None,
                                flags: panel::PLUGINPANELITEMFLAGS::PPIF_PROCESSDESCR,
                                file_attributes: panel::FILE_ATTRIBUTES::empty(),
                                number_of_links: 0,
                                crc32: 0
                            }
                        }
                    }).collect();
            } else {
                items = Vec::new();
            }
            state.panel_items = Some(panel::PluginPanelItems::from(items));
        }
        let panel_items_ref = state.panel_items.as_ref().unwrap();
        trace!("<get_find_data()");
        return Ok(panel_items_ref);
    }

    fn compare(&mut self, info: panel::CompareInfo) -> Option<Ordering> {
        Some(info.item1.file_name.to_string_lossy().cmp(&info.item2.file_name.to_string_lossy()))
    }

    fn delete_files(&mut self, info: panel::DeleteFilesInfo) -> Result<()> {
        trace!(">delete_files()");
        let state: &mut PanelState = self.panels.get_mut(&info.panel).unwrap();
        let current_path = state.current_path();

        for item in &info.panel_items {
            let mut file_path = PathBuf::from(&current_path);
            file_path.push(&item.file_name.to_string_lossy());
            trace!("removing file '{}' with size {}", &file_path.to_str().unwrap(), &item.file_size);
            let path = file_path.as_path();
            if path.is_file() {
                let _ = fs::remove_file(&path);
            } else if path.is_dir() {
                let _ = fs::remove_dir(&path);
            }
        }
        trace!("<delete_files()");
        Ok(())
    }

    fn free_find_data(&mut self, handle: HANDLE) {
        trace!(">free_find_data()");
        let state: &mut PanelState = self.panels.get_mut(&handle).unwrap();
        state.panel_items = None;
        trace!("<free_find_data()");
    }

    fn set_directory(&mut self, handle: HANDLE, path: &WideString) -> Result<()> {
        trace!(">set_directory()");
        let state: &mut PanelState = self.panels.get_mut(&handle).unwrap();
        let new_path = PathBuf::from(path.to_string_lossy());
        trace!("New path: {:?}", &path.to_string_lossy());
        state.apply_path_segment(new_path);
        trace!("Resolved path: {:?}", state.current_path());
        trace!("<set_directory()");
        Ok(())
    }

    fn get_files(&mut self, info: &mut panel::GetFilesInfo) -> Result<panel::ReturnCode> {
        let state: &mut PanelState = self.panels.get_mut(&info.panel).unwrap();
        let current_path = state.current_path();

        trace!("destination: {}", info.dest_path);
        for item in &info.panel_items {
            trace!("file: {}", item.file_name);
            let mut from = current_path.clone();
            from.push(&item.file_name.to_string_lossy());
            let mut to = PathBuf::from(&info.dest_path.to_string_lossy());
            to.push(&item.file_name.to_string_lossy());

            if !info.move_file {
                trace!("copy: '{}' '{}'", from.to_str().unwrap(), to.to_str().unwrap());
                match fs::copy(&from, &to) {
                    Ok(_) => trace!("copy is done"),
                    Err(e) => trace!("copy is failed: '{}'", e)
                }
            } else {
                trace!("rename: '{}' '{}'", from.to_str().unwrap(), to.to_str().unwrap());
                match fs::rename(&from, &to) {
                    Ok(_) => trace!("rename is done"),
                    Err(e) => trace!("rename is failed: '{}'", e)
                }
            }
        }

        Ok(panel::ReturnCode::Success)
    }

    fn make_directory(&mut self, info: &mut panel::MakeDirectoryInfo) -> Result<panel::ReturnCode> {
        trace!(">make_directory()");
        let state: &mut PanelState = self.panels.get_mut(&info.panel).unwrap();
        let current_path = state.current_path();

        if let Some(name) = info.name.upgrade() {
            trace!("directory: '{}', silent: {}", &name, &info.op_mode.contains(panel::OPERATION_MODES::OPM_SILENT));
        }
        let mut directory_path = PathBuf::from(&current_path);

        let result: Result<ReturnCode>;
        let mut cancelled = false;
        if !&info.op_mode.contains(panel::OPERATION_MODES::OPM_SILENT) {
            let input = basic::input_box(Some(basic::get_msg(&Lng::MessageTitleCreateDirectory)),
                                         Some(basic::get_msg(&Lng::MessageCreateDirectoryName)),
                                         None,
                                         None,
                                         100,
                                         None,
                                         basic::INPUTBOXFLAGS::FIB_NONE);

            match input {
                Some(name) => {
                    state.make_directory_name = Some(Rc::from(WideString::from(name)));
                    info.name = Rc::downgrade(state.make_directory_name.as_ref().unwrap());
                },
                None => cancelled = true
            }
        }

        result = if !cancelled {
            match info.name.upgrade() {
                Some(name) => {
                    directory_path.push(name.to_string_lossy());
                    match fs::create_dir(&directory_path) {
                        Ok(_, ) => {
                            Ok(panel::ReturnCode::Success)
                        },
                        Err(e) => {
                            trace!("|make_directory(): {}", e);
                            Err(e.into())
                        }
                    }
                },
                None => unimplemented!(),
            }
        } else {
            Ok(panel::ReturnCode::UserCancel)
        };

        trace!("<make_directory()");
        return result;
    }

    fn process_panel_event(&mut self, _info: panel::ProcessPanelEventInfo) -> bool {
        return false;
    }

    fn process_panel_input(&mut self, _info: panel::ProcessPanelInputInfo) -> bool {
        return false;
    }

    fn put_files(&mut self, info: panel::PutFilesInfo) -> Result<panel::PutFilesReturnCode> {
        trace!(">put_files_info()");
        let result;
        trace!("|put_files_info() source path: '{}', move file: {}, ", &info.src_path, &info.move_file);
        let state: &mut PanelState = self.panels.get_mut(&info.panel).unwrap();
        let current_path = state.current_path();

        for item in &info.panel_item {
            trace!("|put_files_info() item: {}, ", &item.file_name);
            let mut src_file_path = PathBuf::from(&info.src_path.to_string_lossy());
            src_file_path.push(&item.file_name.to_string_lossy());
            let mut dst_file_path = PathBuf::from(&current_path);
            dst_file_path.push(&item.file_name.to_string_lossy());
            if info.move_file {
                let _ = fs::rename(&src_file_path, &dst_file_path);
            } else {
                let _ = fs::copy(&src_file_path, &dst_file_path);
            }

        }
        result = Ok(panel::PutFilesReturnCode::Success);
        trace!("<put_files_info()");
        return result;
    }
}

impl settings::ExportFunctions for Plugin {

    #[allow(unused_variables)]
    fn configure(&self, info: &settings::ConfigureInfo) -> libc::intptr_t {
        trace!(">configure()");
        basic::message(basic::FARMESSAGEFLAGS::FMSG_MB_OK,
                       None,
                       basic::MessageItems::Lines(vec!(basic::get_msg(&Lng::MessageTitleConfiguration),
                                                           basic::get_msg(&Lng::MessageConfiguration))),
                       0);

        trace!("<configure()");
        return 0;
    }

}

fn filetime_from_u64(filetime: u64) -> panel::FILETIME {
    panel::FILETIME {
        dwLowDateTime: filetime as u32,
        dwHighDateTime: (filetime >> 32) as u32,
    }
}

fn unimplemented_api() {
    basic::message(basic::FARMESSAGEFLAGS::FMSG_MB_OK | basic::FARMESSAGEFLAGS::FMSG_WARNING,
                   None,
                   basic::MessageItems::Lines(vec!(WideString::new(),
                                                       basic::get_msg(&Lng::MessageApiIsNotImplemented))),
                   0);
}

fn error_dialog(e: farmanager::Error) {
    basic::message(basic::FARMESSAGEFLAGS::FMSG_MB_OK | basic::FARMESSAGEFLAGS::FMSG_WARNING,
                   None,
                   basic::MessageItems::AllInOne(WideString::from(format!("{}\n{}: {}\n{}: {}",
                                                         basic::get_msg(&Lng::ErrorTitle),
                                                         basic::get_msg(&Lng::ErrorCause),
                                                         e.as_fail(),
                                                         basic::get_msg(&Lng::ErrorBacktrace),
                                                         e.backtrace()))),
                   0);
    basic::message(basic::FARMESSAGEFLAGS::FMSG_MB_OK | basic::FARMESSAGEFLAGS::FMSG_WARNING,
                   None,
                   basic::MessageItems::AllInOne(WideString::from(format!("{}\n{}: {}\n{}: {}",
                                                         basic::get_msg(&Lng::ErrorTitle),
                                                         basic::get_msg(&Lng::ErrorCause),
                                                         e.as_fail(),
                                                         basic::get_msg(&Lng::ErrorBacktrace),
                                                         e.backtrace()))),
                   0);
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
