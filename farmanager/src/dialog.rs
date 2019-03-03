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

pub enum ButtonSelection {
    Deselected = 0,
    Selected = 1
}

impl From<u8> for ButtonSelection {
    fn from(value: u8) -> Self {
        match value {
            0 => ButtonSelection::Deselected,
            1 => ButtonSelection::Selected,
            _ => unimplemented!()
        }
    }
}

impl Into<DialogItemSelection> for ButtonSelection {
    fn into(self) -> DialogItemSelection {
        DialogItemSelection { value: self as u8 }
    }
}

pub enum CheckBoxSelection {
    Deselected = 0,
    Selected = 1,
    Undefined = 2
}

impl From<u8> for CheckBoxSelection {
    fn from(value: u8) -> Self {
        match value {
            0 => CheckBoxSelection::Deselected,
            1 => CheckBoxSelection::Selected,
            2 => CheckBoxSelection::Undefined,
            _ => unimplemented!()
        }
    }
}

impl Into<DialogItemSelection> for CheckBoxSelection {
    fn into(self) -> DialogItemSelection {
        DialogItemSelection { value: self as u8 }
    }
}

pub enum RadioButtonSelection {
    Previous = 0,
    Active = 1
}

impl From<u8> for RadioButtonSelection {
    fn from(value: u8) -> Self {
        match value {
            0 => RadioButtonSelection::Previous,
            1 => RadioButtonSelection::Active,
            _ => unimplemented!()
        }
    }
}

impl Into<DialogItemSelection> for RadioButtonSelection {
    fn into(self) -> DialogItemSelection {
        DialogItemSelection { value: self as u8 }
    }
}

pub struct DialogItemSelection {
    value: u8
}

impl DialogItemSelection {

    pub fn as_raw(self) -> u8 {
        self.value
    }

    pub fn as_button(self) -> CheckBoxSelection {
        CheckBoxSelection::from(self.value)
    }

    pub fn as_check_box(self) -> CheckBoxSelection {
        CheckBoxSelection::from(self.value)
    }

    pub fn as_radio_button(self) -> RadioButtonSelection {
        RadioButtonSelection::from(self.value)
    }
}

pub enum FarDialogItem {
    CheckBox { x: isize, y: isize, selected: DialogItemSelection, flags: FARDIALOGITEMFLAGS, text: WideString },
    Text { x1: isize, y: isize, x2: isize, mask: Option<WideString>, flags: FARDIALOGITEMFLAGS, text: Option<WideString> },
    VText { x: isize, y1: isize, y2: isize, mask: Option<WideString>, flags: FARDIALOGITEMFLAGS, text: Option<WideString> },
    SingleBox { x1: isize, y1: isize, x2: isize, y2: isize, flags: FARDIALOGITEMFLAGS, title: Option<WideString> },
    DoubleBox { x1: isize, y1: isize, x2: isize, y2: isize, flags: FARDIALOGITEMFLAGS, title: Option<WideString> },
    Edit { x1: isize, y: isize, x2: isize, history: Option<WideString>, flags: FARDIALOGITEMFLAGS, text: Option<WideString> },
    FixEdit { x1: isize, y: isize, x2: isize, history: Option<WideString>, mask: Option<WideString>, flags: FARDIALOGITEMFLAGS, text: Option<WideString> },
    PswEdit { x1: isize, y: isize, x2: isize, flags: FARDIALOGITEMFLAGS, text: Option<WideString> },
    RadioButton { x: isize, y: isize, selected: DialogItemSelection, flags: FARDIALOGITEMFLAGS, text: WideString },
    Button { x: isize, y: isize, selected: DialogItemSelection, flags: FARDIALOGITEMFLAGS, text: WideString },
}

impl Into<ffi::FarDialogItem> for FarDialogItem {
    fn into(self) -> ffi::FarDialogItem {
        match self {
            FarDialogItem::CheckBox { x, y, selected, flags, text } => ffi::FarDialogItem {
                item_type: ffi::FARDIALOGITEMTYPES::DI_CHECKBOX,
                x1: x,
                y1: y,
                x2: 0,
                y2: y,
                param: ffi::FarDialogItemParam { selected: selected.value as libc::intptr_t },
                history: ptr::null(),
                mask: ptr::null(),
                flags,
                data: text.as_ptr(),
                max_length: 0,
                user_data: 0,
                reserved: [0; 2]
            },
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
            FarDialogItem::VText { x, y1 , y2, mask, flags, text } => ffi::FarDialogItem {
                item_type: ffi::FARDIALOGITEMTYPES::DI_VTEXT,
                x1: x,
                y1,
                x2: x,
                y2,
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
            FarDialogItem::Edit { x1, y, x2, history, flags, text } => ffi::FarDialogItem {
                item_type: ffi::FARDIALOGITEMTYPES::DI_EDIT,
                x1,
                y1: y,
                x2,
                y2: y,
                param: ffi::FarDialogItemParam { reserved: 0 },
                history: match history {
                    Some(v) => v.as_ptr(),
                    None => ptr::null(),
                },
                mask: ptr::null(),
                flags,
                data: match text {
                    Some(v) => v.as_ptr(),
                    None => ptr::null(),
                },
                max_length: 0,
                user_data: 0,
                reserved: [0; 2]
            },
            FarDialogItem::FixEdit { x1, y, x2, history, mask, flags, text } => ffi::FarDialogItem {
                item_type: ffi::FARDIALOGITEMTYPES::DI_FIXEDIT,
                x1,
                y1: y,
                x2,
                y2: y,
                param: ffi::FarDialogItemParam { reserved: 0 },
                history: match history {
                    Some(v) => v.as_ptr(),
                    None => ptr::null(),
                },
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
            FarDialogItem::PswEdit { x1, y, x2, flags, text } => ffi::FarDialogItem {
                item_type: ffi::FARDIALOGITEMTYPES::DI_PSWEDIT,
                x1,
                y1: y,
                x2,
                y2: y,
                param: ffi::FarDialogItemParam { reserved: 0 },
                history: ptr::null(),
                mask: ptr::null(),
                flags,
                data: match text {
                    Some(v) => v.as_ptr(),
                    None => ptr::null(),
                },
                max_length: 0,
                user_data: 0,
                reserved: [0; 2]
            },
            FarDialogItem::RadioButton { x, y, selected, flags, text } => ffi::FarDialogItem {
                item_type: ffi::FARDIALOGITEMTYPES::DI_RADIOBUTTON,
                x1: x,
                y1: y,
                x2: 0,
                y2: y,
                param: ffi::FarDialogItemParam { selected: selected.value as libc::intptr_t },
                history: ptr::null(),
                mask: ptr::null(),
                flags,
                data: text.as_ptr(),
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
                param: ffi::FarDialogItemParam { selected: selected.value as libc::intptr_t },
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

// TODO replace generic variant definitions
pub enum FarMessage {
    DmFirst { param1: libc::intptr_t, param2: *mut libc::c_void },
    DmClose { param1: libc::intptr_t, param2: *mut libc::c_void },
    DmEnable { param1: libc::intptr_t, param2: *mut libc::c_void },
    DmEnableRedraw { param1: libc::intptr_t, param2: *mut libc::c_void },
    DmGetDlgItem { param1: libc::intptr_t, param2: *mut libc::c_void },
    DmGetDlgRect { param1: libc::intptr_t, param2: *mut libc::c_void },
    DmGetText { param1: libc::intptr_t, param2: *mut libc::c_void },
    DmKey { param1: libc::intptr_t, param2: *mut libc::c_void },
    DmMoveDialog { param1: libc::intptr_t, param2: *mut libc::c_void },
    DmSetDlgItem { param1: libc::intptr_t, param2: *mut libc::c_void },
    DmSetFocus { param1: libc::intptr_t, param2: *mut libc::c_void },
    DmRedraw { param1: libc::intptr_t, param2: *mut libc::c_void },
    DmSetText { param1: libc::intptr_t, param2: *mut libc::c_void },
    DmSetMaxTextLength { param1: libc::intptr_t, param2: *mut libc::c_void },
    DmShowDialog { param1: libc::intptr_t, param2: *mut libc::c_void },
    DmGetFocus { param1: libc::intptr_t, param2: *mut libc::c_void },
    DmGetCursorPos { param1: libc::intptr_t, param2: *mut libc::c_void },
    DmSetCursorPos { param1: libc::intptr_t, param2: *mut libc::c_void },
    DmSetTextPtr { param1: libc::intptr_t, param2: *mut libc::c_void },
    DmShowItem { param1: libc::intptr_t, param2: *mut libc::c_void },
    DmAddHistory { param1: libc::intptr_t, param2: *mut libc::c_void },
    DmGetCheck { param1: libc::intptr_t, param2: *mut libc::c_void },
    DmSetCheck { param1: libc::intptr_t, param2: *mut libc::c_void },
    DmSet3State { param1: libc::intptr_t, param2: *mut libc::c_void },
    DmListSort { param1: libc::intptr_t, param2: *mut libc::c_void },
    DmListGetItem { param1: libc::intptr_t, param2: *mut libc::c_void },
    DmListGetCurpos { param1: libc::intptr_t, param2: *mut libc::c_void },
    DmListSetCurPos { param1: libc::intptr_t, param2: *mut libc::c_void },
    DmListDelete { param1: libc::intptr_t, param2: *mut libc::c_void },
    DmListAdd { param1: libc::intptr_t, param2: *mut libc::c_void },
    DmListAddStr { param1: libc::intptr_t, param2: *mut libc::c_void },
    DmListUpdate { param1: libc::intptr_t, param2: *mut libc::c_void },
    DmListInsert { param1: libc::intptr_t, param2: *mut libc::c_void },
    DmListFindString { param1: libc::intptr_t, param2: *mut libc::c_void },
    DmListInfo { param1: libc::intptr_t, param2: *mut libc::c_void },
    DmListGetData { param1: libc::intptr_t, param2: *mut libc::c_void },
    DmListSetData { param1: libc::intptr_t, param2: *mut libc::c_void },
    DmListSetTitles { param1: libc::intptr_t, param2: *mut libc::c_void },
    DmListGetTitles { param1: libc::intptr_t, param2: *mut libc::c_void },
    DmResizeDialog { param1: libc::intptr_t, param2: *mut libc::c_void },
    DmSetItemPosition { param1: libc::intptr_t, param2: *mut libc::c_void },
    DmGetDropDownOpened { param1: libc::intptr_t, param2: *mut libc::c_void },
    DmSetDropdownOpened { param1: libc::intptr_t, param2: *mut libc::c_void },
    DmSetHistory { param1: libc::intptr_t, param2: *mut libc::c_void },
    DmGetItemPosition { param1: libc::intptr_t, param2: *mut libc::c_void },
    DmSetInputNotify { param1: libc::intptr_t, param2: *mut libc::c_void },
    /* DmSetMouseEventNotify = DmSetInputNotify,*/
    DmEditUnchangedFlag { param1: libc::intptr_t, param2: *mut libc::c_void },
    DmGetItemData { param1: libc::intptr_t, param2: *mut libc::c_void },
    DmSetItemData { param1: libc::intptr_t, param2: *mut libc::c_void },
    DmListSet { param1: libc::intptr_t, param2: *mut libc::c_void },
    DmGetCursorSize { param1: libc::intptr_t, param2: *mut libc::c_void },
    DmSetCursorSize { param1: libc::intptr_t, param2: *mut libc::c_void },
    DmListGetDataSize { param1: libc::intptr_t, param2: *mut libc::c_void },
    DmGetSelection { param1: libc::intptr_t, param2: *mut libc::c_void },
    DmSetSelection { param1: libc::intptr_t, param2: *mut libc::c_void },
    DmGetEditPosition { param1: libc::intptr_t, param2: *mut libc::c_void },
    DmSetEditPosition { param1: libc::intptr_t, param2: *mut libc::c_void },
    DmSetComboboxEvent { param1: libc::intptr_t, param2: *mut libc::c_void },
    DmGetComboboxEvent { param1: libc::intptr_t, param2: *mut libc::c_void },
    DmGetConstTextPtr { param1: libc::intptr_t, param2: *mut libc::c_void },
    DmGetDlgItemShort { param1: libc::intptr_t, param2: *mut libc::c_void },
    DmSetDlgItemShort { param1: libc::intptr_t, param2: *mut libc::c_void },
    DmGetDialogInfo { param1: libc::intptr_t, param2: *mut libc::c_void },
    DmGetDialogTitle { param1: libc::intptr_t, param2: *mut libc::c_void },
    DnFirst { param1: libc::intptr_t, param2: *mut libc::c_void },
    DnBtnClick { id: isize, state: DialogItemSelection },
    DnCtlColorDialog { param1: libc::intptr_t, param2: *mut libc::c_void },
    DnCtlColorDlgItem { param1: libc::intptr_t, param2: *mut libc::c_void },
    DnCtlColorDlgList { param1: libc::intptr_t, param2: *mut libc::c_void },
    DnDrawDialog { param1: libc::intptr_t, param2: *mut libc::c_void },
    DnDrawDlgItem { param1: libc::intptr_t, param2: *mut libc::c_void },
    DnEditChange { param1: libc::intptr_t, param2: *mut libc::c_void },
    DnEnterIdle { param1: libc::intptr_t, param2: *mut libc::c_void },
    DnGotFocus { param1: libc::intptr_t, param2: *mut libc::c_void },
    DnHelp { param1: libc::intptr_t, param2: *mut libc::c_void },
    DnHotKey { param1: libc::intptr_t, param2: *mut libc::c_void },
    DnInitDialog { param1: libc::intptr_t, param2: *mut libc::c_void },
    DnKillFocus { param1: libc::intptr_t, param2: *mut libc::c_void },
    DnListChange { param1: libc::intptr_t, param2: *mut libc::c_void },
    DnDragged { param1: libc::intptr_t, param2: *mut libc::c_void },
    DnResizeConsole { param1: libc::intptr_t, param2: *mut libc::c_void },
    DnDrawDialogDone { param1: libc::intptr_t, param2: *mut libc::c_void },
    DnListHotKey { param1: libc::intptr_t, param2: *mut libc::c_void },
    DnInput { param1: libc::intptr_t, param2: *mut libc::c_void },
    DnControlInput { param1: libc::intptr_t, param2: *mut libc::c_void },
    DnClose { param1: libc::intptr_t, param2: *mut libc::c_void },
    DnGetValue { param1: libc::intptr_t, param2: *mut libc::c_void },
    DnDropdownOpened { param1: libc::intptr_t, param2: *mut libc::c_void },
    DnDrawDlgItemDone { param1: libc::intptr_t, param2: *mut libc::c_void },
    DmUser { param1: libc::intptr_t, param2: *mut libc::c_void },
}

impl FarMessage {
    fn from(msg: ffi::FARMESSAGE, param1: libc::intptr_t, param2: *mut libc::c_void) -> Option<Self> {
        match msg {
            FARMESSAGE::DM_FIRST => { Some(FarMessage::DmFirst { param1 , param2 }) },
            FARMESSAGE::DM_CLOSE => { Some(FarMessage::DmClose { param1 , param2 }) },
            FARMESSAGE::DM_ENABLE => { Some(FarMessage::DmEnable { param1 , param2 }) },
            FARMESSAGE::DM_ENABLEREDRAW => { Some(FarMessage::DmEnableRedraw { param1 , param2 }) },
            FARMESSAGE::DM_GETDLGDATA => { None },
            FARMESSAGE::DM_GETDLGITEM => { Some(FarMessage::DmGetDlgItem { param1 , param2 }) },
            FARMESSAGE::DM_GETDLGRECT => { Some(FarMessage::DmGetDlgRect { param1 , param2 }) },
            FARMESSAGE::DM_GETTEXT => { Some(FarMessage::DmGetText { param1 , param2 }) },
            FARMESSAGE::DM_KEY => { Some(FarMessage::DmKey { param1 , param2 }) },
            FARMESSAGE::DM_MOVEDIALOG => { Some(FarMessage::DmMoveDialog { param1 , param2 }) },
            FARMESSAGE::DM_SETDLGDATA => { None },
            FARMESSAGE::DM_SETDLGITEM => { Some(FarMessage::DmSetDlgItem { param1 , param2 }) },
            FARMESSAGE::DM_SETFOCUS => { Some(FarMessage::DmSetFocus { param1 , param2 }) },
            FARMESSAGE::DM_REDRAW => { Some(FarMessage::DmRedraw { param1 , param2 }) },
            FARMESSAGE::DM_SETTEXT => { Some(FarMessage::DmSetText { param1 , param2 }) },
            FARMESSAGE::DM_SETMAXTEXTLENGTH => { Some(FarMessage::DmSetMaxTextLength { param1 , param2 }) },
            FARMESSAGE::DM_SHOWDIALOG => { Some(FarMessage::DmShowDialog { param1 , param2 }) },
            FARMESSAGE::DM_GETFOCUS => { Some(FarMessage::DmGetFocus { param1 , param2 }) },
            FARMESSAGE::DM_GETCURSORPOS => { Some(FarMessage::DmGetCursorPos { param1 , param2 }) },
            FARMESSAGE::DM_SETCURSORPOS => { Some(FarMessage::DmSetCursorPos { param1 , param2 }) },
            FARMESSAGE::DM_SETTEXTPTR => { Some(FarMessage::DmSetTextPtr { param1 , param2 }) },
            FARMESSAGE::DM_SHOWITEM => { Some(FarMessage::DmShowItem { param1 , param2 }) },
            FARMESSAGE::DM_ADDHISTORY => { Some(FarMessage::DmAddHistory { param1 , param2 }) },
            FARMESSAGE::DM_GETCHECK => { Some(FarMessage::DmGetCheck { param1 , param2 }) },
            FARMESSAGE::DM_SETCHECK => { Some(FarMessage::DmSetCheck { param1 , param2 }) },
            FARMESSAGE::DM_SET3STATE => { Some(FarMessage::DmSet3State { param1 , param2 }) },
            FARMESSAGE::DM_LISTSORT => { Some(FarMessage::DmListSort { param1 , param2 }) },
            FARMESSAGE::DM_LISTGETITEM => { Some(FarMessage::DmListGetItem { param1 , param2 }) },
            FARMESSAGE::DM_LISTGETCURPOS => { Some(FarMessage::DmListGetCurpos { param1 , param2 }) },
            FARMESSAGE::DM_LISTSETCURPOS => { Some(FarMessage::DmListSetCurPos { param1 , param2 }) },
            FARMESSAGE::DM_LISTDELETE => { Some(FarMessage::DmListDelete { param1 , param2 }) },
            FARMESSAGE::DM_LISTADD => { Some(FarMessage::DmListAdd { param1 , param2 }) },
            FARMESSAGE::DM_LISTADDSTR => { Some(FarMessage::DmListAddStr { param1 , param2 }) },
            FARMESSAGE::DM_LISTUPDATE => { Some(FarMessage::DmListUpdate { param1 , param2 }) },
            FARMESSAGE::DM_LISTINSERT => { Some(FarMessage::DmListInsert { param1 , param2 }) },
            FARMESSAGE::DM_LISTFINDSTRING => { Some(FarMessage::DmListFindString { param1 , param2 }) },
            FARMESSAGE::DM_LISTINFO => { Some(FarMessage::DmListInfo { param1 , param2 }) },
            FARMESSAGE::DM_LISTGETDATA => { Some(FarMessage::DmListGetData { param1 , param2 }) },
            FARMESSAGE::DM_LISTSETDATA => { Some(FarMessage::DmListSetData { param1 , param2 }) },
            FARMESSAGE::DM_LISTSETTITLES => { Some(FarMessage::DmListSetTitles { param1 , param2 }) },
            FARMESSAGE::DM_LISTGETTITLES => { Some(FarMessage::DmListGetTitles { param1 , param2 }) },
            FARMESSAGE::DM_RESIZEDIALOG => { Some(FarMessage::DmResizeDialog { param1 , param2 }) },
            FARMESSAGE::DM_SETITEMPOSITION => { Some(FarMessage::DmSetItemPosition { param1 , param2 }) },
            FARMESSAGE::DM_GETDROPDOWNOPENED => { Some(FarMessage::DmGetDropDownOpened { param1 , param2 }) },
            FARMESSAGE::DM_SETDROPDOWNOPENED => { Some(FarMessage::DmSetDropdownOpened { param1 , param2 }) },
            FARMESSAGE::DM_SETHISTORY => { Some(FarMessage::DmSetHistory { param1 , param2 }) },
            FARMESSAGE::DM_GETITEMPOSITION => { Some(FarMessage::DmGetItemPosition { param1 , param2 }) },
            FARMESSAGE::DM_SETINPUTNOTIFY => { Some(FarMessage::DmSetInputNotify { param1 , param2 }) },
            /* FARMESSAGE::DM_SETMOUSEEVENTNOTIFY          = FARMESSAGE::DM_SETINPUTNOTIFY,*/
            FARMESSAGE::DM_EDITUNCHANGEDFLAG => { Some(FarMessage::DmEditUnchangedFlag { param1 , param2 }) },
            FARMESSAGE::DM_GETITEMDATA => { Some(FarMessage::DmGetItemData { param1 , param2 }) },
            FARMESSAGE::DM_SETITEMDATA => { Some(FarMessage::DmSetItemData { param1 , param2 }) },
            FARMESSAGE::DM_LISTSET => { Some(FarMessage::DmListSet { param1 , param2 }) },
            FARMESSAGE::DM_GETCURSORSIZE => { Some(FarMessage::DmGetCursorSize { param1 , param2 }) },
            FARMESSAGE::DM_SETCURSORSIZE => { Some(FarMessage::DmSetCursorSize { param1 , param2 }) },
            FARMESSAGE::DM_LISTGETDATASIZE => { Some(FarMessage::DmListGetDataSize { param1 , param2 }) },
            FARMESSAGE::DM_GETSELECTION => { Some(FarMessage::DmGetSelection { param1 , param2 }) },
            FARMESSAGE::DM_SETSELECTION => { Some(FarMessage::DmSetSelection { param1 , param2 }) },
            FARMESSAGE::DM_GETEDITPOSITION => { Some(FarMessage::DmGetEditPosition { param1 , param2 }) },
            FARMESSAGE::DM_SETEDITPOSITION => { Some(FarMessage::DmSetEditPosition { param1 , param2 }) },
            FARMESSAGE::DM_SETCOMBOBOXEVENT => { Some(FarMessage::DmSetComboboxEvent { param1 , param2 }) },
            FARMESSAGE::DM_GETCOMBOBOXEVENT => { Some(FarMessage::DmGetComboboxEvent { param1 , param2 }) },
            FARMESSAGE::DM_GETCONSTTEXTPTR => { Some(FarMessage::DmGetConstTextPtr { param1 , param2 }) },
            FARMESSAGE::DM_GETDLGITEMSHORT => { Some(FarMessage::DmGetDlgItemShort { param1 , param2 }) },
            FARMESSAGE::DM_SETDLGITEMSHORT => { Some(FarMessage::DmSetDlgItemShort { param1 , param2 }) },
            FARMESSAGE::DM_GETDIALOGINFO => { Some(FarMessage::DmGetDialogInfo { param1 , param2 }) },
            FARMESSAGE::DM_GETDIALOGTITLE => { Some(FarMessage::DmGetDialogTitle { param1 , param2 }) },
            FARMESSAGE::DN_FIRST => { Some(FarMessage::DnFirst { param1 , param2 }) },
            FARMESSAGE::DN_BTNCLICK => {
                Some(FarMessage::DnBtnClick { id: param1 , state: DialogItemSelection { value: param2 as u8 } } )
            },
            FARMESSAGE::DN_CTLCOLORDIALOG => { Some(FarMessage::DnCtlColorDialog { param1 , param2 }) },
            FARMESSAGE::DN_CTLCOLORDLGITEM => { Some(FarMessage::DnCtlColorDlgItem { param1 , param2 }) },
            FARMESSAGE::DN_CTLCOLORDLGLIST => { Some(FarMessage::DnCtlColorDlgList { param1 , param2 }) },
            FARMESSAGE::DN_DRAWDIALOG => { Some(FarMessage::DnDrawDialog { param1 , param2 }) },
            FARMESSAGE::DN_DRAWDLGITEM => { Some(FarMessage::DnDrawDlgItem { param1 , param2 }) },
            FARMESSAGE::DN_EDITCHANGE => { Some(FarMessage::DnEditChange { param1 , param2 }) },
            FARMESSAGE::DN_ENTERIDLE => { Some(FarMessage::DnEnterIdle { param1 , param2 }) },
            FARMESSAGE::DN_GOTFOCUS => { Some(FarMessage::DnGotFocus { param1 , param2 }) },
            FARMESSAGE::DN_HELP => { Some(FarMessage::DnHelp { param1 , param2 }) },
            FARMESSAGE::DN_HOTKEY => { Some(FarMessage::DnHotKey { param1 , param2 }) },
            FARMESSAGE::DN_INITDIALOG => { Some(FarMessage::DnInitDialog { param1 , param2 }) },
            FARMESSAGE::DN_KILLFOCUS => { Some(FarMessage::DnKillFocus { param1 , param2 }) },
            FARMESSAGE::DN_LISTCHANGE => { Some(FarMessage::DnListChange { param1 , param2 }) },
            FARMESSAGE::DN_DRAGGED => { Some(FarMessage::DnDragged { param1 , param2 }) },
            FARMESSAGE::DN_RESIZECONSOLE => { Some(FarMessage::DnResizeConsole { param1 , param2 }) },
            FARMESSAGE::DN_DRAWDIALOGDONE => { Some(FarMessage::DnDrawDialogDone { param1 , param2 }) },
            FARMESSAGE::DN_LISTHOTKEY => { Some(FarMessage::DnListHotKey { param1 , param2 }) },
            FARMESSAGE::DN_INPUT => { Some(FarMessage::DnInput { param1 , param2 }) },
            FARMESSAGE::DN_CONTROLINPUT => { Some(FarMessage::DnControlInput { param1 , param2 }) },
            FARMESSAGE::DN_CLOSE => { Some(FarMessage::DnClose { param1 , param2 }) },
            FARMESSAGE::DN_GETVALUE => { Some(FarMessage::DnGetValue { param1 , param2 }) },
            FARMESSAGE::DN_DROPDOWNOPENED => { Some(FarMessage::DnDropdownOpened { param1 , param2 }) },
            FARMESSAGE::DN_DRAWDLGITEMDONE => { Some(FarMessage::DnDrawDlgItemDone { param1 , param2 }) },
            FARMESSAGE::DM_USER => { Some(FarMessage::DmUser { param1 , param2 }) },
        }
    }

    fn into(self) -> (ffi::FARMESSAGE, libc::intptr_t, *mut libc::c_void) {
        match self {
            FarMessage::DmFirst { param1, param2 } => (ffi::FARMESSAGE::DM_FIRST, param1, param2),
            FarMessage::DmClose { param1, param2 } => (ffi::FARMESSAGE::DM_CLOSE, param1, param2),
            FarMessage::DmEnable { param1, param2 } => (ffi::FARMESSAGE::DM_ENABLE, param1, param2),
            FarMessage::DmEnableRedraw { param1, param2 } => (ffi::FARMESSAGE::DM_ENABLEREDRAW, param1, param2),
            FarMessage::DmGetDlgItem { param1, param2 } => (ffi::FARMESSAGE::DM_GETDLGITEM, param1, param2),
            FarMessage::DmGetDlgRect { param1, param2 } => (ffi::FARMESSAGE::DM_GETDLGRECT, param1, param2),
            FarMessage::DmGetText { param1, param2 } => (ffi::FARMESSAGE::DM_GETTEXT, param1, param2),
            FarMessage::DmKey { param1, param2 } => (ffi::FARMESSAGE::DM_KEY, param1, param2),
            FarMessage::DmMoveDialog { param1, param2 } => (ffi::FARMESSAGE::DM_MOVEDIALOG, param1, param2),
            FarMessage::DmSetDlgItem { param1, param2 } => (ffi::FARMESSAGE::DM_SETDLGITEM, param1, param2),
            FarMessage::DmSetFocus { param1, param2 } => (ffi::FARMESSAGE::DM_SETFOCUS, param1, param2),
            FarMessage::DmRedraw { param1, param2 } => (ffi::FARMESSAGE::DM_REDRAW, param1, param2),
            FarMessage::DmSetText { param1, param2 } => (ffi::FARMESSAGE::DM_SETTEXT, param1, param2),
            FarMessage::DmSetMaxTextLength { param1, param2 } => (ffi::FARMESSAGE::DM_SETMAXTEXTLENGTH, param1, param2),
            FarMessage::DmShowDialog { param1, param2 } => (ffi::FARMESSAGE::DM_SHOWDIALOG, param1, param2),
            FarMessage::DmGetFocus { param1, param2 } => (ffi::FARMESSAGE::DM_GETFOCUS, param1, param2),
            FarMessage::DmGetCursorPos { param1, param2 } => (ffi::FARMESSAGE::DM_GETCURSORPOS, param1, param2),
            FarMessage::DmSetCursorPos { param1, param2 } => (ffi::FARMESSAGE::DM_SETCURSORPOS, param1, param2),
            FarMessage::DmSetTextPtr { param1, param2 } => (ffi::FARMESSAGE::DM_SETTEXTPTR, param1, param2),
            FarMessage::DmShowItem { param1, param2 } => (ffi::FARMESSAGE::DM_SHOWITEM, param1, param2),
            FarMessage::DmAddHistory { param1, param2 } => (ffi::FARMESSAGE::DM_ADDHISTORY, param1, param2),
            FarMessage::DmGetCheck { param1, param2 } => (ffi::FARMESSAGE::DM_GETCHECK, param1, param2),
            FarMessage::DmSetCheck { param1, param2 } => (ffi::FARMESSAGE::DM_SETCHECK, param1, param2),
            FarMessage::DmSet3State { param1, param2 } => (ffi::FARMESSAGE::DM_SET3STATE, param1, param2),
            FarMessage::DmListSort { param1, param2 } => (ffi::FARMESSAGE::DM_LISTSORT, param1, param2),
            FarMessage::DmListGetItem { param1, param2 } => (ffi::FARMESSAGE::DM_LISTGETITEM, param1, param2),
            FarMessage::DmListGetCurpos { param1, param2 } => (ffi::FARMESSAGE::DM_LISTGETCURPOS, param1, param2),
            FarMessage::DmListSetCurPos { param1, param2 } => (ffi::FARMESSAGE::DM_LISTSETCURPOS, param1, param2),
            FarMessage::DmListDelete { param1, param2 } => (ffi::FARMESSAGE::DM_LISTDELETE, param1, param2),
            FarMessage::DmListAdd { param1, param2 } => (ffi::FARMESSAGE::DM_LISTADD, param1, param2),
            FarMessage::DmListAddStr { param1, param2 } => (ffi::FARMESSAGE::DM_LISTADDSTR, param1, param2),
            FarMessage::DmListUpdate { param1, param2 } => (ffi::FARMESSAGE::DM_LISTUPDATE, param1, param2),
            FarMessage::DmListInsert { param1, param2 } => (ffi::FARMESSAGE::DM_LISTINSERT, param1, param2),
            FarMessage::DmListFindString { param1, param2 } => (ffi::FARMESSAGE::DM_LISTFINDSTRING, param1, param2),
            FarMessage::DmListInfo { param1, param2 } => (ffi::FARMESSAGE::DM_LISTINFO, param1, param2),
            FarMessage::DmListGetData { param1, param2 } => (ffi::FARMESSAGE::DM_LISTGETDATA, param1, param2),
            FarMessage::DmListSetData { param1, param2 } => (ffi::FARMESSAGE::DM_LISTSETDATA, param1, param2),
            FarMessage::DmListSetTitles { param1, param2 } => (ffi::FARMESSAGE::DM_LISTSETTITLES, param1, param2),
            FarMessage::DmListGetTitles { param1, param2 } => (ffi::FARMESSAGE::DM_LISTGETTITLES, param1, param2),
            FarMessage::DmResizeDialog { param1, param2 } => (ffi::FARMESSAGE::DM_RESIZEDIALOG, param1, param2),
            FarMessage::DmSetItemPosition { param1, param2 } => (ffi::FARMESSAGE::DM_SETITEMPOSITION, param1, param2),
            FarMessage::DmGetDropDownOpened { param1, param2 } => (ffi::FARMESSAGE::DM_GETDROPDOWNOPENED, param1, param2),
            FarMessage::DmSetDropdownOpened { param1, param2 } => (ffi::FARMESSAGE::DM_SETDROPDOWNOPENED, param1, param2),
            FarMessage::DmSetHistory { param1, param2 } => (ffi::FARMESSAGE::DM_SETHISTORY, param1, param2),
            FarMessage::DmGetItemPosition { param1, param2 } => (ffi::FARMESSAGE::DM_GETITEMPOSITION, param1, param2),
            FarMessage::DmSetInputNotify { param1, param2 } => (ffi::FARMESSAGE::DM_SETINPUTNOTIFY, param1, param2),
            FarMessage::DmEditUnchangedFlag { param1, param2 } => (ffi::FARMESSAGE::DM_EDITUNCHANGEDFLAG, param1, param2),
            FarMessage::DmGetItemData { param1, param2 } => (ffi::FARMESSAGE::DM_GETITEMDATA, param1, param2),
            FarMessage::DmSetItemData { param1, param2 } => (ffi::FARMESSAGE::DM_SETITEMDATA, param1, param2),
            FarMessage::DmListSet { param1, param2 } => (ffi::FARMESSAGE::DM_LISTSET, param1, param2),
            FarMessage::DmGetCursorSize { param1, param2 } => (ffi::FARMESSAGE::DM_GETCURSORSIZE, param1, param2),
            FarMessage::DmSetCursorSize { param1, param2 } => (ffi::FARMESSAGE::DM_SETCURSORSIZE, param1, param2),
            FarMessage::DmListGetDataSize { param1, param2 } => (ffi::FARMESSAGE::DM_LISTGETDATASIZE, param1, param2),
            FarMessage::DmGetSelection { param1, param2 } => (ffi::FARMESSAGE::DM_GETSELECTION, param1, param2),
            FarMessage::DmSetSelection { param1, param2 } => (ffi::FARMESSAGE::DM_SETSELECTION, param1, param2),
            FarMessage::DmGetEditPosition { param1, param2 } => (ffi::FARMESSAGE::DM_GETEDITPOSITION, param1, param2),
            FarMessage::DmSetEditPosition { param1, param2 } => (ffi::FARMESSAGE::DM_SETEDITPOSITION, param1, param2),
            FarMessage::DmSetComboboxEvent { param1, param2 } => (ffi::FARMESSAGE::DM_SETCOMBOBOXEVENT, param1, param2),
            FarMessage::DmGetComboboxEvent { param1, param2 } => (ffi::FARMESSAGE::DM_GETCOMBOBOXEVENT, param1, param2),
            FarMessage::DmGetConstTextPtr { param1, param2 } => (ffi::FARMESSAGE::DM_GETCONSTTEXTPTR, param1, param2),
            FarMessage::DmGetDlgItemShort { param1, param2 } => (ffi::FARMESSAGE::DM_GETDLGITEMSHORT, param1, param2),
            FarMessage::DmSetDlgItemShort { param1, param2 } => (ffi::FARMESSAGE::DM_SETDLGITEMSHORT, param1, param2),
            FarMessage::DmGetDialogInfo { param1, param2 } => (ffi::FARMESSAGE::DM_GETDIALOGINFO, param1, param2),
            FarMessage::DmGetDialogTitle { param1, param2 } => (ffi::FARMESSAGE::DM_GETDIALOGTITLE, param1, param2),
            FarMessage::DnFirst { param1, param2 } => (ffi::FARMESSAGE::DN_FIRST, param1, param2),
            FarMessage::DnBtnClick { id, state } => (ffi::FARMESSAGE::DN_BTNCLICK, id, state.value as *mut libc::c_void),
            FarMessage::DnCtlColorDialog { param1, param2 } => (ffi::FARMESSAGE::DN_CTLCOLORDIALOG, param1, param2),
            FarMessage::DnCtlColorDlgItem { param1, param2 } => (ffi::FARMESSAGE::DN_CTLCOLORDLGITEM, param1, param2),
            FarMessage::DnCtlColorDlgList { param1, param2 } => (ffi::FARMESSAGE::DN_CTLCOLORDLGLIST, param1, param2),
            FarMessage::DnDrawDialog { param1, param2 } => (ffi::FARMESSAGE::DN_DRAWDIALOG, param1, param2),
            FarMessage::DnDrawDlgItem { param1, param2 } => (ffi::FARMESSAGE::DN_DRAWDLGITEM, param1, param2),
            FarMessage::DnEditChange { param1, param2 } => (ffi::FARMESSAGE::DN_EDITCHANGE, param1, param2),
            FarMessage::DnEnterIdle { param1, param2 } => (ffi::FARMESSAGE::DN_ENTERIDLE, param1, param2),
            FarMessage::DnGotFocus { param1, param2 } => (ffi::FARMESSAGE::DN_GOTFOCUS, param1, param2),
            FarMessage::DnHelp { param1, param2 } => (ffi::FARMESSAGE::DN_HELP, param1, param2),
            FarMessage::DnHotKey { param1, param2 } => (ffi::FARMESSAGE::DN_HOTKEY, param1, param2),
            FarMessage::DnInitDialog { param1, param2 } => (ffi::FARMESSAGE::DN_INITDIALOG, param1, param2),
            FarMessage::DnKillFocus { param1, param2 } => (ffi::FARMESSAGE::DN_KILLFOCUS, param1, param2),
            FarMessage::DnListChange { param1, param2 } => (ffi::FARMESSAGE::DN_LISTCHANGE, param1, param2),
            FarMessage::DnDragged { param1, param2 } => (ffi::FARMESSAGE::DN_DRAGGED, param1, param2),
            FarMessage::DnResizeConsole { param1, param2 } => (ffi::FARMESSAGE::DN_RESIZECONSOLE, param1, param2),
            FarMessage::DnDrawDialogDone { param1, param2 } => (ffi::FARMESSAGE::DN_DRAWDIALOGDONE, param1, param2),
            FarMessage::DnListHotKey { param1, param2 } => (ffi::FARMESSAGE::DN_LISTHOTKEY, param1, param2),
            FarMessage::DnInput { param1, param2 } => (ffi::FARMESSAGE::DN_INPUT, param1, param2),
            FarMessage::DnControlInput { param1, param2 } => (ffi::FARMESSAGE::DN_CONTROLINPUT, param1, param2),
            FarMessage::DnClose { param1, param2 } => (ffi::FARMESSAGE::DN_CLOSE, param1, param2),
            FarMessage::DnGetValue { param1, param2 } => (ffi::FARMESSAGE::DN_GETVALUE, param1, param2),
            FarMessage::DnDropdownOpened { param1, param2 } => (ffi::FARMESSAGE::DN_DROPDOWNOPENED, param1, param2),
            FarMessage::DnDrawDlgItemDone { param1, param2 } => (ffi::FARMESSAGE::DN_DRAWDLGITEMDONE, param1, param2),
            FarMessage::DmUser { param1, param2 } => (ffi::FARMESSAGE::DM_USER, param1, param2),
        }
    }
}

pub trait FarDialog {
    fn dlg_proc(&mut self, h_dlg: crate::HANDLE, msg: FarMessage) -> isize;
}

pub struct Dialog<F: FarDialog> {
    handle: ffi::HANDLE,
    #[allow(dead_code)]
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
    trace!(">process_dialog_event()");
    trace!("<process_dialog_event()");
    0
}