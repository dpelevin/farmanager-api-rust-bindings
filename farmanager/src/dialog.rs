use std::cell::RefCell;
use std::collections::HashMap;
use std::mem;
use std::ptr;

use failure::*;
use libc::*;
use log::*;

use crate::common::string::WideString;
use crate::far_api;
use crate::ffi;
pub use crate::ffi::FARDIALOGFLAGS as FARDIALOGFLAGS;
pub use crate::ffi::FARDIALOGITEMFLAGS as FARDIALOGITEMFLAGS;
pub use crate::ffi::FARMESSAGE as FARMESSAGE;

pub enum FarDialogItem {
    Text { x1: isize, y: isize, x2: isize, mask: Option<WideString>, flags: FARDIALOGITEMFLAGS, text: Option<WideString> },
    SingleBox { x1: isize, y1: isize, x2: isize, y2: isize, flags: FARDIALOGITEMFLAGS, title: Option<WideString> },
    DoubleBox { x1: isize, y1: isize, x2: isize, y2: isize, flags: FARDIALOGITEMFLAGS, title: Option<WideString> },
    Button { x: isize, y: isize, selected: bool, flags: FARDIALOGITEMFLAGS, text: WideString },
}

impl Into<ffi::FarDialogItem> for FarDialogItem {
    fn into(self) -> ffi::FarDialogItem {
        match self {
            FarDialogItem::Text { x1, y, x2, mask, flags, text } => ffi::FarDialogItem {
                item_type: ffi::FARDIALOGITEMTYPES::DI_TEXT,
                x1,
                y1: y,
                x2,
                y2: y,
                param: ffi::FarDialogItemParam { reserved: 0 },
                history: ptr::null(),
                mask: match mask {
                    Some(v) => v.as_ptr(),
                    None => ptr::null(),
                },
                flags,
                data: match text {
                    Some(v) => v.as_ptr(),
                    None => ptr::null(),
                },
                max_length: 0,
                user_data: 0,
                reserved: [0; 2]
            },
            FarDialogItem::SingleBox { x1, y1, x2, y2, flags, title } => ffi::FarDialogItem {
                item_type: ffi::FARDIALOGITEMTYPES::DI_SINGLEBOX,
                x1,
                y1,
                x2,
                y2,
                param: ffi::FarDialogItemParam { reserved: 0 },
                history: ptr::null(),
                mask: ptr::null(),
                flags,
                data: match title {
                    Some(v) => v.as_ptr(),
                    None => ptr::null(),
                },
                max_length: 0,
                user_data: 0,
                reserved: [0; 2]
            },
            FarDialogItem::DoubleBox { x1, y1, x2, y2, flags, title } => ffi::FarDialogItem {
                item_type: ffi::FARDIALOGITEMTYPES::DI_DOUBLEBOX,
                x1,
                y1,
                x2,
                y2,
                param: ffi::FarDialogItemParam { reserved: 0 },
                history: ptr::null(),
                mask: ptr::null(),
                flags,
                data: match title {
                    Some(v) => v.as_ptr(),
                    None => ptr::null(),
                },
                max_length: 0,
                user_data: 0,
                reserved: [0; 2]
            },
            FarDialogItem::Button { x, y, selected, flags, text } => ffi::FarDialogItem {
                item_type: ffi::FARDIALOGITEMTYPES::DI_BUTTON,
                x1: x,
                y1: y,
                x2: 0,
                y2: y,
                param: ffi::FarDialogItemParam {
                    selected: match selected {
                        true => 1,
                        false => 0
                    }
                },
                history: ptr::null(),
                mask: ptr::null(),
                flags,
                data: text.as_ptr(),
                max_length: 0,
                user_data: 0,
                reserved: [0; 2]
            },
        }
    }
}

pub enum FarMessage {
    DmFirst,
    DmClose,
    DmEnable,
    DmEnableRedraw,
    DmGetDlgItem,
    DmGetDlgRect,
    DmGetText,
    DmKey,
    DmMoveDialog,
    DmSetDlgItem,
    DmSetFocus,
    DmRedraw,
    DmSetText,
    DmSetMaxTextLength,
    DmShowDialog,
    DmGetFocus,
    DmGetCursorPos,
    DmSetCursorPos,
    DmSetTextPtr,
    DmShowItem,
    DmAddHistory,

    DmGetCheck,
    DmSetCheck,
    DmSet3State,

    DmListSort,
    DmListGetItem,
    DmListGetCurpos,
    DmListSetCurPos,
    DmListDelete,
    DmListAdd,
    DmListAddStr,
    DmListUpdate,
    DmListInsert,
    DmListFindString,
    DmListInfo,
    DmListGetData,
    DmListSetData,
    DmListSetTitles,
    DmListGetTitles,

    DmResizeDialog,
    DmSetItemPosition,

    DmGetDropDownOpened,
    DmSetDropdownOpened,

    DmSetHistory,

    DmGetItemPosition,
    DmSetInputNotify,
    /* DmSetMouseEventNotify = DmSetInputNotify,*/

    DmEditUnchangedFlag,

    DmGetItemData,
    DmSetItemData,

    DmListSet,

    DmGetCursorSize,
    DmSetCursorSize,

    DmListGetDataSize,

    DmGetSelection,
    DmSetSelection,

    DmGetEditPosition,
    DmSetEditPosition,

    DmSetComboboxEvent,
    DmGetComboboxEvent,

    DmGetConstTextPtr,
    DmGetDlgItemShort,
    DmSetDlgItemShort,

    DmGetDialogInfo,

    DmGetDialogTitle,

    DnFirst,
    DnBtnClick { id: isize, state: u8 },
    DnCtlColorDialog,
    DnCtlColorDlgItem,
    DnCtlColorDlgList,
    DnDrawDialog,
    DnDrawDlgItem,
    DnEditChange,
    DnEnterIdle,
    DnGotFocus,
    DnHelp,
    DnHotKey,
    DnInitDialog,
    DnKillFocus,
    DnListChange,
    DnDragged,
    DnResizeConsole,
    DnDrawDialogDone,
    DnListHotKey,
    DnInput,
    DnControlInput,
    DnClose,
    DnGetValue,
    DnDropdownOpened,
    DnDrawDlgItemDone,
    DmUser,
}

impl FarMessage {
    // TODO remove Option from return type as support for all FarMessages is implemented
    fn from(msg: ffi::FARMESSAGE, param1: libc::intptr_t, param2: *mut libc::c_void) -> Option<Self> {
        match msg {
            FARMESSAGE::DN_BTNCLICK => Some(FarMessage::DnBtnClick { id: param1, state: param2 as u8 } ),
            _ => None
        }
    }

    fn into(self) -> (ffi::FARMESSAGE, libc::intptr_t, *mut libc::c_void) {
        match self {
            FarMessage::DmFirst => (ffi::FARMESSAGE::DM_FIRST, 0, ptr::null_mut()),
            FarMessage::DmClose => (ffi::FARMESSAGE::DM_CLOSE, 0, ptr::null_mut()),
            FarMessage::DmEnable => (ffi::FARMESSAGE::DM_ENABLE, 0, ptr::null_mut()),
            FarMessage::DmEnableRedraw => (ffi::FARMESSAGE::DM_ENABLEREDRAW, 0, ptr::null_mut()),
            FarMessage::DmGetDlgItem => (ffi::FARMESSAGE::DM_GETDLGITEM, 0, ptr::null_mut()),
            FarMessage::DmGetDlgRect => (ffi::FARMESSAGE::DM_GETDLGRECT, 0, ptr::null_mut()),
            FarMessage::DmGetText => (ffi::FARMESSAGE::DM_GETTEXT, 0, ptr::null_mut()),
            FarMessage::DmKey => (ffi::FARMESSAGE::DM_KEY, 0, ptr::null_mut()),
            FarMessage::DmMoveDialog => (ffi::FARMESSAGE::DM_MOVEDIALOG, 0, ptr::null_mut()),
            FarMessage::DmSetDlgItem => (ffi::FARMESSAGE::DM_SETDLGITEM, 0, ptr::null_mut()),
            FarMessage::DmSetFocus => (ffi::FARMESSAGE::DM_SETFOCUS, 0, ptr::null_mut()),
            FarMessage::DmRedraw => (ffi::FARMESSAGE::DM_REDRAW, 0, ptr::null_mut()),
            FarMessage::DmSetText => (ffi::FARMESSAGE::DM_SETTEXT, 0, ptr::null_mut()),
            FarMessage::DmSetMaxTextLength => (ffi::FARMESSAGE::DM_SETMAXTEXTLENGTH, 0, ptr::null_mut()),
            FarMessage::DmShowDialog => (ffi::FARMESSAGE::DM_SHOWDIALOG, 0, ptr::null_mut()),
            FarMessage::DmGetFocus => (ffi::FARMESSAGE::DM_GETFOCUS, 0, ptr::null_mut()),
            FarMessage::DmGetCursorPos => (ffi::FARMESSAGE::DM_GETCURSORPOS, 0, ptr::null_mut()),
            FarMessage::DmSetCursorPos => (ffi::FARMESSAGE::DM_SETCURSORPOS, 0, ptr::null_mut()),
            FarMessage::DmSetTextPtr => (ffi::FARMESSAGE::DM_SETTEXTPTR, 0, ptr::null_mut()),
            FarMessage::DmShowItem => (ffi::FARMESSAGE::DM_SHOWITEM, 0, ptr::null_mut()),
            FarMessage::DmAddHistory => (ffi::FARMESSAGE::DM_ADDHISTORY, 0, ptr::null_mut()),
            FarMessage::DmGetCheck => (ffi::FARMESSAGE::DM_GETCHECK, 0, ptr::null_mut()),
            FarMessage::DmSetCheck => (ffi::FARMESSAGE::DM_SETCHECK, 0, ptr::null_mut()),
            FarMessage::DmSet3State => (ffi::FARMESSAGE::DM_SET3STATE, 0, ptr::null_mut()),
            FarMessage::DmListSort => (ffi::FARMESSAGE::DM_LISTSORT, 0, ptr::null_mut()),
            FarMessage::DmListGetItem => (ffi::FARMESSAGE::DM_LISTGETITEM, 0, ptr::null_mut()),
            FarMessage::DmListGetCurpos => (ffi::FARMESSAGE::DM_LISTGETCURPOS, 0, ptr::null_mut()),
            FarMessage::DmListSetCurPos => (ffi::FARMESSAGE::DM_LISTSETCURPOS, 0, ptr::null_mut()),
            FarMessage::DmListDelete => (ffi::FARMESSAGE::DM_LISTDELETE, 0, ptr::null_mut()),
            FarMessage::DmListAdd => (ffi::FARMESSAGE::DM_LISTADD, 0, ptr::null_mut()),
            FarMessage::DmListAddStr => (ffi::FARMESSAGE::DM_LISTADDSTR, 0, ptr::null_mut()),
            FarMessage::DmListUpdate => (ffi::FARMESSAGE::DM_LISTUPDATE, 0, ptr::null_mut()),
            FarMessage::DmListInsert => (ffi::FARMESSAGE::DM_LISTINSERT, 0, ptr::null_mut()),
            FarMessage::DmListFindString => (ffi::FARMESSAGE::DM_LISTFINDSTRING, 0, ptr::null_mut()),
            FarMessage::DmListInfo => (ffi::FARMESSAGE::DM_LISTINFO, 0, ptr::null_mut()),
            FarMessage::DmListGetData => (ffi::FARMESSAGE::DM_LISTGETDATA, 0, ptr::null_mut()),
            FarMessage::DmListSetData => (ffi::FARMESSAGE::DM_LISTSETDATA, 0, ptr::null_mut()),
            FarMessage::DmListSetTitles => (ffi::FARMESSAGE::DM_LISTSETTITLES, 0, ptr::null_mut()),
            FarMessage::DmListGetTitles => (ffi::FARMESSAGE::DM_LISTGETTITLES, 0, ptr::null_mut()),
            FarMessage::DmResizeDialog => (ffi::FARMESSAGE::DM_RESIZEDIALOG, 0, ptr::null_mut()),
            FarMessage::DmSetItemPosition => (ffi::FARMESSAGE::DM_SETITEMPOSITION, 0, ptr::null_mut()),
            FarMessage::DmGetDropDownOpened => (ffi::FARMESSAGE::DM_GETDROPDOWNOPENED, 0, ptr::null_mut()),
            FarMessage::DmSetDropdownOpened => (ffi::FARMESSAGE::DM_SETDROPDOWNOPENED, 0, ptr::null_mut()),
            FarMessage::DmSetHistory => (ffi::FARMESSAGE::DM_SETHISTORY, 0, ptr::null_mut()),
            FarMessage::DmGetItemPosition => (ffi::FARMESSAGE::DM_GETITEMPOSITION, 0, ptr::null_mut()),
            FarMessage::DmSetInputNotify => (ffi::FARMESSAGE::DM_SETINPUTNOTIFY, 0, ptr::null_mut()),
            FarMessage::DmEditUnchangedFlag => (ffi::FARMESSAGE::DM_EDITUNCHANGEDFLAG, 0, ptr::null_mut()),
            FarMessage::DmGetItemData => (ffi::FARMESSAGE::DM_GETITEMDATA, 0, ptr::null_mut()),
            FarMessage::DmSetItemData => (ffi::FARMESSAGE::DM_SETITEMDATA, 0, ptr::null_mut()),
            FarMessage::DmListSet => (ffi::FARMESSAGE::DM_LISTSET, 0, ptr::null_mut()),
            FarMessage::DmGetCursorSize => (ffi::FARMESSAGE::DM_GETCURSORSIZE, 0, ptr::null_mut()),
            FarMessage::DmSetCursorSize => (ffi::FARMESSAGE::DM_SETCURSORSIZE, 0, ptr::null_mut()),
            FarMessage::DmListGetDataSize => (ffi::FARMESSAGE::DM_LISTGETDATASIZE, 0, ptr::null_mut()),
            FarMessage::DmGetSelection => (ffi::FARMESSAGE::DM_GETSELECTION, 0, ptr::null_mut()),
            FarMessage::DmSetSelection => (ffi::FARMESSAGE::DM_SETSELECTION, 0, ptr::null_mut()),
            FarMessage::DmGetEditPosition => (ffi::FARMESSAGE::DM_GETEDITPOSITION, 0, ptr::null_mut()),
            FarMessage::DmSetEditPosition => (ffi::FARMESSAGE::DM_SETEDITPOSITION, 0, ptr::null_mut()),
            FarMessage::DmSetComboboxEvent => (ffi::FARMESSAGE::DM_SETCOMBOBOXEVENT, 0, ptr::null_mut()),
            FarMessage::DmGetComboboxEvent => (ffi::FARMESSAGE::DM_GETCOMBOBOXEVENT, 0, ptr::null_mut()),
            FarMessage::DmGetConstTextPtr => (ffi::FARMESSAGE::DM_GETCONSTTEXTPTR, 0, ptr::null_mut()),
            FarMessage::DmGetDlgItemShort => (ffi::FARMESSAGE::DM_GETDLGITEMSHORT, 0, ptr::null_mut()),
            FarMessage::DmSetDlgItemShort => (ffi::FARMESSAGE::DM_SETDLGITEMSHORT, 0, ptr::null_mut()),
            FarMessage::DmGetDialogInfo => (ffi::FARMESSAGE::DM_GETDIALOGINFO, 0, ptr::null_mut()),
            FarMessage::DmGetDialogTitle => (ffi::FARMESSAGE::DM_GETDIALOGTITLE, 0, ptr::null_mut()),
            FarMessage::DnFirst => (ffi::FARMESSAGE::DN_FIRST, 0, ptr::null_mut()),
            FarMessage::DnBtnClick { id, state } => (ffi::FARMESSAGE::DN_BTNCLICK, id, state as *mut libc::c_void),
            FarMessage::DnCtlColorDialog => (ffi::FARMESSAGE::DN_CTLCOLORDIALOG, 0, ptr::null_mut()),
            FarMessage::DnCtlColorDlgItem => (ffi::FARMESSAGE::DN_CTLCOLORDLGITEM, 0, ptr::null_mut()),
            FarMessage::DnCtlColorDlgList => (ffi::FARMESSAGE::DN_CTLCOLORDLGLIST, 0, ptr::null_mut()),
            FarMessage::DnDrawDialog => (ffi::FARMESSAGE::DN_DRAWDIALOG, 0, ptr::null_mut()),
            FarMessage::DnDrawDlgItem => (ffi::FARMESSAGE::DN_DRAWDLGITEM, 0, ptr::null_mut()),
            FarMessage::DnEditChange => (ffi::FARMESSAGE::DN_EDITCHANGE, 0, ptr::null_mut()),
            FarMessage::DnEnterIdle => (ffi::FARMESSAGE::DN_ENTERIDLE, 0, ptr::null_mut()),
            FarMessage::DnGotFocus => (ffi::FARMESSAGE::DN_GOTFOCUS, 0, ptr::null_mut()),
            FarMessage::DnHelp => (ffi::FARMESSAGE::DN_HELP, 0, ptr::null_mut()),
            FarMessage::DnHotKey => (ffi::FARMESSAGE::DN_HOTKEY, 0, ptr::null_mut()),
            FarMessage::DnInitDialog => (ffi::FARMESSAGE::DN_INITDIALOG, 0, ptr::null_mut()),
            FarMessage::DnKillFocus => (ffi::FARMESSAGE::DN_KILLFOCUS, 0, ptr::null_mut()),
            FarMessage::DnListChange => (ffi::FARMESSAGE::DN_LISTCHANGE, 0, ptr::null_mut()),
            FarMessage::DnDragged => (ffi::FARMESSAGE::DN_DRAGGED, 0, ptr::null_mut()),
            FarMessage::DnResizeConsole => (ffi::FARMESSAGE::DN_RESIZECONSOLE, 0, ptr::null_mut()),
            FarMessage::DnDrawDialogDone => (ffi::FARMESSAGE::DN_DRAWDIALOGDONE, 0, ptr::null_mut()),
            FarMessage::DnListHotKey => (ffi::FARMESSAGE::DN_LISTHOTKEY, 0, ptr::null_mut()),
            FarMessage::DnInput => (ffi::FARMESSAGE::DN_INPUT, 0, ptr::null_mut()),
            FarMessage::DnControlInput => (ffi::FARMESSAGE::DN_CONTROLINPUT, 0, ptr::null_mut()),
            FarMessage::DnClose => (ffi::FARMESSAGE::DN_CLOSE, 0, ptr::null_mut()),
            FarMessage::DnGetValue => (ffi::FARMESSAGE::DN_GETVALUE, 0, ptr::null_mut()),
            FarMessage::DnDropdownOpened => (ffi::FARMESSAGE::DN_DROPDOWNOPENED, 0, ptr::null_mut()),
            FarMessage::DnDrawDlgItemDone => (ffi::FARMESSAGE::DN_DRAWDLGITEMDONE, 0, ptr::null_mut()),
            FarMessage::DmUser => (ffi::FARMESSAGE::DM_USER, 0, ptr::null_mut()),
        }
    }
}

pub trait FarDialog {
    fn dlg_proc(&mut self, h_dlg: crate::HANDLE, msg: FarMessage) -> isize;
}

pub struct Dialog<F: FarDialog> {
    handle: ffi::HANDLE,
    internal: Box<F>
}

impl<F: FarDialog> Dialog<F> {

    pub fn init(plugin_id: crate::GUID, id: crate::GUID, x1: isize, y1: isize, x2: isize, y2: isize,
                help_topic: Option<WideString>, dialog_items: Vec<FarDialogItem>,
                flags: ffi::FARDIALOGFLAGS, dialog: F) -> crate::Result<Self> {

        let mut internal = Box::new(dialog);

        let help_topic = match help_topic {
            Some(text) => text.as_ptr(),
            None => ptr::null(),
        };
        let mut dialog_items_ffi: Vec<ffi::FarDialogItem> = Vec::new();
        for di in dialog_items {
            dialog_items_ffi.push(di.into());
        }
        let handle: ffi::HANDLE = far_api(|far_api: &mut ffi::PluginStartupInfo| {
            far_api.dialog_init(&plugin_id, &id, x1, y1, x2, y2, help_topic, dialog_items_ffi.as_ptr(), dialog_items_ffi.len(), 0, flags, callback::<F>, &mut *internal as *mut F as *mut libc::c_void)
        });

        if handle == ffi::INVALID_HANDLE_VALUE {
            return Err(format_err!(""));
        }

        extern "C" fn callback<F>(h_dlg: ffi::HANDLE, msg: libc::intptr_t, param1: libc::intptr_t, param2: *mut libc::c_void) -> libc::intptr_t where F: FarDialog {
            let dlg_ptr: *mut F = far_api(|far_api: &mut ffi::PluginStartupInfo| {
                far_api.send_dlg_message(h_dlg, ffi::FARMESSAGE::DM_GETDLGDATA as isize, 0, ptr::null())
            }) as *mut F;

            let dlg = unsafe { &mut *dlg_ptr };
            let far_msg = FarMessage::from( unsafe { mem::transmute(msg as i32) }, param1, param2);
            let result = match far_msg {
                Some(msg) => {
                    dlg.dlg_proc(h_dlg, msg)
                },
                // TODO remove together with match as soon as support for all FarMessages is implemented
                None => far_api(|far_api: &mut ffi::PluginStartupInfo| {
                    far_api.def_dlg_proc(h_dlg, msg as libc::intptr_t, param1, param2)
                }),
            };
            return result;
        }

        Ok(Dialog {
            handle,
            internal
        })
    }

    pub fn run(&self) {
        far_api(|far_api: &mut ffi::PluginStartupInfo| {
            far_api.dialog_run(self.handle);
        })
    }
}

impl<F: FarDialog> Drop for Dialog<F> {
    fn drop(&mut self) {
        far_api(|far_api: &mut ffi::PluginStartupInfo| {
            far_api.dialog_free(self.handle);
        });
    }
}

pub fn def_dlg_proc(h_dlg: crate::HANDLE, msg: FarMessage) -> isize {
    let (ffi_msg, param1, param2) = msg.into();
    return far_api(|far_api: &mut ffi::PluginStartupInfo| {
        far_api.def_dlg_proc(h_dlg, ffi_msg as libc::intptr_t, param1, param2)
    })
}

#[allow(unused_variables)]
#[cfg(feature = "dialog")]
#[no_mangle]
#[export_name="ProcessDialogEventW"]
pub extern "system" fn process_dialog_event(info: *const ffi::ProcessDialogEventInfo) -> libc::intptr_t {
//    trace!(">process_dialog_event()");
//    trace!("<process_dialog_event()");
    0
}