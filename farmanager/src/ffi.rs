#![allow(non_camel_case_types)]

use std::clone::Clone;
use std::fmt::{Debug, Error, Formatter};
use std::mem::transmute;
use std::ptr;
use std::result::Result;

use bitflags::bitflags;
use libc::{c_double, c_int, c_longlong, c_uchar, c_ulonglong, c_void, intptr_t, size_t, uintptr_t};
pub use winapi::ctypes::wchar_t as wchar_t;
pub use winapi::shared::guiddef::GUID as GUID;
pub(crate) use winapi::shared::minwindef::{BOOL, WORD};
pub use winapi::shared::minwindef::DWORD as DWORD;
pub use winapi::shared::minwindef::FALSE as FALSE;
pub use winapi::shared::minwindef::FILETIME;
pub use winapi::shared::minwindef::TRUE as TRUE;
pub use winapi::shared::windef::{COLORREF, RECT};
pub use winapi::um::handleapi::INVALID_HANDLE_VALUE as INVALID_HANDLE_VALUE;
use winapi::um::minwinbase::LPSECURITY_ATTRIBUTES;
use winapi::um::wincon::{COORD, KEY_EVENT};
pub use winapi::um::wincon::CTRL_BREAK_EVENT as CTRL_BREAK_EVENT;
pub use winapi::um::wincon::INPUT_RECORD as INPUT_RECORD;
pub use winapi::um::winnt::HANDLE as HANDLE;

pub const FARMANAGERVERSION_MAJOR: DWORD = 3;
pub const FARMANAGERVERSION_MINOR: DWORD = 0;
pub const FARMANAGERVERSION_REVISION: DWORD = 0;
pub const FARMANAGERVERSION_BUILD: DWORD = 5300;
pub const FARMANAGERVERSION_STAGE: VersionStage = VersionStage::VS_RELEASE;

pub const FARMACRO_KEY_EVENT: DWORD = (KEY_EVENT | 0x8000) as DWORD;

pub const CP_UNICODE: uintptr_t = 1200;
pub const CP_REVERSEBOM: uintptr_t = 1201;
pub const CP_DEFAULT: uintptr_t = (-1 as intptr_t) as uintptr_t;// -1
pub const CP_REDETECT: uintptr_t = (-2 as intptr_t) as uintptr_t;// -2

bitflags! {
    pub struct FARCOLORFLAGS: c_ulonglong {
        const FCF_FG_4BIT       = 0x0000000000000001;
        const FCF_BG_4BIT       = 0x0000000000000002;
        const FCF_4BITMASK      = 0x0000000000000003; // FCF_FG_4BIT|FCF_BG_4BIT

        const FCF_RAWATTR_MASK  = 0x000000000000FF00; // stored console attributes

        const FCF_EXTENDEDFLAGS = 0xFFFFFFFFFFFFFFFC; // ~FCF_4BITMASK

        const FCF_FG_BOLD       = 0x1000000000000000;
        const FCF_FG_ITALIC     = 0x2000000000000000;
        const FCF_FG_UNDERLINE  = 0x4000000000000000;
        const FCF_STYLEMASK     = 0x7000000000000000; // FCF_FG_BOLD|FCF_FG_ITALIC|FCF_FG_UNDERLINE

        const FCF_NONE          = 0;
    }
}

#[repr(C)] #[derive(PartialEq, Eq, Default, Clone, Copy)]
pub struct rgba {
    pub r: c_uchar,
    pub g: c_uchar,
    pub b: c_uchar,
    pub a: c_uchar,
}

#[repr(C)] #[derive(PartialEq, Eq, Default, Clone, Copy)]
pub struct FarColorForeground {
    pub foreground_color: COLORREF,
    pub foreground_rgba: rgba,
}

#[repr(C)] #[derive(PartialEq, Eq, Default, Clone, Copy)]
pub struct FarColorBackground {
    pub background_color: COLORREF,
    pub background_rgba: rgba,
}

#[repr(C)] #[derive(PartialEq, Eq, Clone, Copy)]
pub struct FarColor {
    pub flags: FARCOLORFLAGS,
    pub foreground_color: COLORREF,
    pub background_color: COLORREF,
    reserved: *const c_void,
}

impl FarColor {

    pub fn foreground_rgba(&self) -> rgba {
        unsafe { transmute(self.foreground_color) }
    }

    pub fn background_rgba(&self) -> rgba {
        unsafe { transmute(self.background_color) }
    }

}

impl Default for FarColor {

    fn default() -> FarColor {
        FarColor {
            flags: FARCOLORFLAGS::FCF_4BITMASK,
            foreground_color: COLORREF::default(),
            background_color: COLORREF::default(),
            /*Foreground: FarColorForeground::default(), Background: FarColorBackground::default(), */
            reserved: ptr::null()
        }
    }

}

bitflags! {
    pub struct COLORDIALOGFLAGS: c_ulonglong {
        const CDF_NONE = 0;
    }
}

pub type FARAPICOLORDIALOG = extern fn(plugin_id: *const GUID, flags: COLORDIALOGFLAGS, color: *const FarColor) -> BOOL;

bitflags! {
    pub struct FARMESSAGEFLAGS: c_ulonglong {
        const FMSG_WARNING             = 0x0000000000000001;
        const FMSG_ERRORTYPE           = 0x0000000000000002;
        const FMSG_KEEPBACKGROUND      = 0x0000000000000004;
        const FMSG_LEFTALIGN           = 0x0000000000000008;
        const FMSG_ALLINONE            = 0x0000000000000010;
        const FMSG_MB_OK               = 0x0000000000010000;
        const FMSG_MB_OKCANCEL         = 0x0000000000020000;
        const FMSG_MB_ABORTRETRYIGNORE = 0x0000000000030000;
        const FMSG_MB_YESNO            = 0x0000000000040000;
        const FMSG_MB_YESNOCANCEL      = 0x0000000000050000;
        const FMSG_MB_RETRYCANCEL      = 0x0000000000060000;
        const FMSG_NONE                = 0;
    }
}

pub type FARAPIMESSAGE = extern fn(plugin_id: *const GUID, id: *const GUID, flags: FARMESSAGEFLAGS, help_topic: *const wchar_t, items: *const *const wchar_t, items_number: size_t, buttons_number: intptr_t) -> intptr_t;

#[repr(C)] #[derive(Clone, Copy)]
pub enum FARDIALOGITEMTYPES {
    DI_TEXT                         =  0,
    DI_VTEXT                        =  1,
    DI_SINGLEBOX                    =  2,
    DI_DOUBLEBOX                    =  3,
    DI_EDIT                         =  4,
    DI_PSWEDIT                      =  5,
    DI_FIXEDIT                      =  6,
    DI_BUTTON                       =  7,
    DI_CHECKBOX                     =  8,
    DI_RADIOBUTTON                  =  9,
    DI_COMBOBOX                     = 10,
    DI_LISTBOX                      = 11,

    DI_USERCONTROL                  =255,
}

#[inline]
pub fn is_edit(item_type: FARDIALOGITEMTYPES) -> BOOL {
    match item_type {
        FARDIALOGITEMTYPES::DI_EDIT => TRUE,
        FARDIALOGITEMTYPES::DI_FIXEDIT => TRUE,
        FARDIALOGITEMTYPES::DI_PSWEDIT => TRUE,
        FARDIALOGITEMTYPES::DI_COMBOBOX => TRUE,
        _ => FALSE,
    }
}

bitflags! {
    pub struct FARDIALOGITEMFLAGS: c_ulonglong {
        const DIF_BOXCOLOR              = 0x0000000000000200;
        const DIF_GROUP                 = 0x0000000000000400;
        const DIF_LEFTTEXT              = 0x0000000000000800;
        const DIF_MOVESELECT            = 0x0000000000001000;
        const DIF_SHOWAMPERSAND         = 0x0000000000002000;
        const DIF_CENTERGROUP           = 0x0000000000004000;
        const DIF_NOBRACKETS            = 0x0000000000008000;
        const DIF_MANUALADDHISTORY      = 0x0000000000008000;
        const DIF_SEPARATOR             = 0x0000000000010000;
        const DIF_SEPARATOR2            = 0x0000000000020000;
        const DIF_EDITOR                = 0x0000000000020000;
        const DIF_LISTNOAMPERSAND       = 0x0000000000020000;
        const DIF_LISTNOBOX             = 0x0000000000040000;
        const DIF_HISTORY               = 0x0000000000040000;
        const DIF_BTNNOCLOSE            = 0x0000000000040000;
        const DIF_CENTERTEXT            = 0x0000000000040000;
        const DIF_SEPARATORUSER         = 0x0000000000080000;
        const DIF_SETSHIELD             = 0x0000000000080000;
        const DIF_EDITEXPAND            = 0x0000000000080000;
        const DIF_DROPDOWNLIST          = 0x0000000000100000;
        const DIF_USELASTHISTORY        = 0x0000000000200000;
        const DIF_MASKEDIT              = 0x0000000000400000;
        const DIF_LISTTRACKMOUSE        = 0x0000000000400000;
        const DIF_LISTTRACKMOUSEINFOCUS = 0x0000000000800000;
        const DIF_SELECTONENTRY         = 0x0000000000800000;
        const DIF_3STATE                = 0x0000000000800000;
        const DIF_EDITPATH              = 0x0000000001000000;
        const DIF_LISTWRAPMODE          = 0x0000000001000000;
        const DIF_NOAUTOCOMPLETE        = 0x0000000002000000;
        const DIF_LISTAUTOHIGHLIGHT     = 0x0000000002000000;
        const DIF_LISTNOCLOSE           = 0x0000000004000000;
        const DIF_EDITPATHEXEC          = 0x0000000004000000;
        const DIF_HIDDEN                = 0x0000000010000000;
        const DIF_READONLY              = 0x0000000020000000;
        const DIF_NOFOCUS               = 0x0000000040000000;
        const DIF_DISABLE               = 0x0000000080000000;
        const DIF_DEFAULTBUTTON         = 0x0000000100000000;
        const DIF_FOCUS                 = 0x0000000200000000;
        const DIF_RIGHTTEXT             = 0x0000000400000000;
        const DIF_WORDWRAP              = 0x0000000800000000;
        const DIF_NONE                  = 0;
    }
}

#[repr(C)] #[derive(Clone, Copy)]
pub enum FARMESSAGE {
    DM_FIRST                        = 0,
    DM_CLOSE                        = 1,
    DM_ENABLE                       = 2,
    DM_ENABLEREDRAW                 = 3,
    DM_GETDLGDATA                   = 4,
    DM_GETDLGITEM                   = 5,
    DM_GETDLGRECT                   = 6,
    DM_GETTEXT                      = 7,
    DM_KEY                          = 9,
    DM_MOVEDIALOG                   = 10,
    DM_SETDLGDATA                   = 11,
    DM_SETDLGITEM                   = 12,
    DM_SETFOCUS                     = 13,
    DM_REDRAW                       = 14,
    DM_SETTEXT                      = 15,
    DM_SETMAXTEXTLENGTH             = 16,
    DM_SHOWDIALOG                   = 17,
    DM_GETFOCUS                     = 18,
    DM_GETCURSORPOS                 = 19,
    DM_SETCURSORPOS                 = 20,
    DM_SETTEXTPTR                   = 22,
    DM_SHOWITEM                     = 23,
    DM_ADDHISTORY                   = 24,

    DM_GETCHECK                     = 25,
    DM_SETCHECK                     = 26,
    DM_SET3STATE                    = 27,

    DM_LISTSORT                     = 28,
    DM_LISTGETITEM                  = 29,
    DM_LISTGETCURPOS                = 30,
    DM_LISTSETCURPOS                = 31,
    DM_LISTDELETE                   = 32,
    DM_LISTADD                      = 33,
    DM_LISTADDSTR                   = 34,
    DM_LISTUPDATE                   = 35,
    DM_LISTINSERT                   = 36,
    DM_LISTFINDSTRING               = 37,
    DM_LISTINFO                     = 38,
    DM_LISTGETDATA                  = 39,
    DM_LISTSETDATA                  = 40,
    DM_LISTSETTITLES                = 41,
    DM_LISTGETTITLES                = 42,

    DM_RESIZEDIALOG                 = 43,
    DM_SETITEMPOSITION              = 44,

    DM_GETDROPDOWNOPENED            = 45,
    DM_SETDROPDOWNOPENED            = 46,

    DM_SETHISTORY                   = 47,

    DM_GETITEMPOSITION              = 48,
    DM_SETINPUTNOTIFY               = 49,
    /* DM_SETMOUSEEVENTNOTIFY          = DM_SETINPUTNOTIFY,*/

    DM_EDITUNCHANGEDFLAG            = 50,

    DM_GETITEMDATA                  = 51,
    DM_SETITEMDATA                  = 52,

    DM_LISTSET                      = 53,

    DM_GETCURSORSIZE                = 54,
    DM_SETCURSORSIZE                = 55,

    DM_LISTGETDATASIZE              = 56,

    DM_GETSELECTION                 = 57,
    DM_SETSELECTION                 = 58,

    DM_GETEDITPOSITION              = 59,
    DM_SETEDITPOSITION              = 60,

    DM_SETCOMBOBOXEVENT             = 61,
    DM_GETCOMBOBOXEVENT             = 62,

    DM_GETCONSTTEXTPTR              = 63,
    DM_GETDLGITEMSHORT              = 64,
    DM_SETDLGITEMSHORT              = 65,

    DM_GETDIALOGINFO                = 66,

    DM_GETDIALOGTITLE               = 67,

    DN_FIRST                        = 4096,
    DN_BTNCLICK                     = 4097,
    DN_CTLCOLORDIALOG               = 4098,
    DN_CTLCOLORDLGITEM              = 4099,
    DN_CTLCOLORDLGLIST              = 4100,
    DN_DRAWDIALOG                   = 4101,
    DN_DRAWDLGITEM                  = 4102,
    DN_EDITCHANGE                   = 4103,
    DN_ENTERIDLE                    = 4104,
    DN_GOTFOCUS                     = 4105,
    DN_HELP                         = 4106,
    DN_HOTKEY                       = 4107,
    DN_INITDIALOG                   = 4108,
    DN_KILLFOCUS                    = 4109,
    DN_LISTCHANGE                   = 4110,
    DN_DRAGGED                      = 4111,
    DN_RESIZECONSOLE                = 4112,
    DN_DRAWDIALOGDONE               = 4113,
    DN_LISTHOTKEY                   = 4114,
    DN_INPUT                        = 4115,
    DN_CONTROLINPUT                 = 4116,
    DN_CLOSE                        = 4117,
    DN_GETVALUE                     = 4118,
    DN_DROPDOWNOPENED               = 4119,
    DN_DRAWDLGITEMDONE              = 4120,

    DM_USER                         = 0x4000,

}

#[repr(C)] #[derive(Clone, Copy)]
pub enum FARCHECKEDSTATE {
    BSTATE_UNCHECKED = 0,
    BSTATE_CHECKED   = 1,
    BSTATE_3STATE    = 2,
    BSTATE_TOGGLE    = 3,
}

#[repr(C)] #[derive(Clone, Copy)]
pub enum FARCOMBOBOXEVENTTYPE {
    CBET_KEY         = 0x00000001,
    CBET_MOUSE       = 0x00000002,
}

bitflags! {
    pub struct LISTITEMFLAGS: c_ulonglong {
        const LIF_SELECTED           = 0x0000000000010000;
        const LIF_CHECKED            = 0x0000000000020000;
        const LIF_SEPARATOR          = 0x0000000000040000;
        const LIF_DISABLE            = 0x0000000000080000;
        const LIF_GRAYED             = 0x0000000000100000;
        const LIF_HIDDEN             = 0x0000000000200000;
        const LIF_DELETEUSERDATA     = 0x0000000080000000;
        const LIF_NONE               = 0;
    }
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct FarListItem {
    pub flags: LISTITEMFLAGS,
    pub text: *const wchar_t,
    pub user_dara: intptr_t,
    pub reserved: intptr_t,
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct FarListUpdate {
    pub struct_size: size_t,
    pub index: intptr_t,
    pub item: FarListItem,
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct FarListInsert {
    pub struct_size: size_t,
    pub index: intptr_t,
    pub item: FarListItem,
}

#[repr(C)] #[derive(Clone, Copy)]
struct FarListGetItem {
    pub struct_size: size_t,
    pub item_index: intptr_t,
    pub item: FarListItem,
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct FarListPos {
    pub struct_size: size_t,
    pub select_pos: intptr_t,
    pub top_pos: intptr_t,
}

bitflags! {
    pub struct FARLISTFINDFLAGS: c_ulonglong {
        const LIFIND_EXACTMATCH = 0x0000000000000001;
        const LIFIND_NONE = 0;
    }
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct FarListFind {
    pub struct_size: size_t,
    pub start_index: intptr_t,
    pub pattern: *const wchar_t,
    pub flags: FARLISTFINDFLAGS,
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct FarListDelete {
    pub struct_size: size_t,
    pub start_index: intptr_t,
    pub count: intptr_t,
}

bitflags! {
    pub struct FARLISTINFOFLAGS: c_ulonglong {
        const LINFO_SHOWNOBOX = 0x0000000000000400;
        const LINFO_AUTOHIGHLIGHT = 0x0000000000000800;
        const LINFO_REVERSEHIGHLIGHT = 0x0000000000001000;
        const LINFO_WRAPMODE = 0x0000000000008000;
        const LINFO_SHOWAMPERSAND = 0x0000000000010000;
        const LINFO_NONE = 0;
    }
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct FarListInfo {
    pub struct_size: size_t,
    pub flags: FARLISTINFOFLAGS,
    pub items_number: size_t,
    pub select_pos: intptr_t,
    pub top_pos: intptr_t,
    pub max_height: intptr_t,
    pub max_length: intptr_t,
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct FarListItemData {
    pub struct_size: size_t,
    pub index: intptr_t,
    pub data_size: size_t,
    pub data: *mut c_void,
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct FarList {
    pub struct_size: size_t,
    pub items_number: size_t,
    pub items: *mut FarListItem,
}

#[repr(C)] #[derive(Clone, Copy)]
struct FarListTitles {
    pub struct_size: size_t,
    pub title_size: size_t,
    pub title: *const wchar_t,
    pub bottom_size: size_t,
    pub bottom: *const wchar_t,
}

#[repr(C)] #[derive(Clone, Copy)]
struct FarDialogItemColors {
    pub struct_size: size_t,
    pub flags: c_ulonglong,
    pub colors_count: size_t,
    pub colors: *mut FarColor,
}

#[repr(C)] #[derive(PartialEq, Eq, Clone, Copy)]
pub struct FAR_CHAR_INFO {
    pub char: wchar_t,
    pub attributes: FarColor,
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct FarDialogItem {
    pub item_type: FARDIALOGITEMTYPES,
    pub x1: intptr_t,
    pub y1: c_longlong,
    pub x2: c_longlong,
    pub y2: c_longlong,
    pub param: *mut c_void,
    pub history: *const wchar_t,
    pub mask: *const wchar_t,
    pub flags: FARDIALOGITEMFLAGS,
    pub data: *const wchar_t,
    pub max_length: size_t, // terminate 0 not included (if == 0 string size is unlimited)
    pub user_data: intptr_t,
    pub reserved: [intptr_t; 2],
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct FarDialogItemData {
    pub struct_size: size_t,
    pub ptr_length: size_t,
    pub ptr_data: *mut wchar_t,
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct FarDialogEvent {
    pub struct_size: size_t,
    pub h_dlg: HANDLE,
    pub msg: intptr_t,
    pub param1: intptr_t,
    pub param2: *mut c_void,
    pub result: intptr_t,
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct OpenDlgPluginData {
    pub struct_size: size_t,
    pub h_dlg: HANDLE,
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct DialogInfo {
    pub struct_size: size_t,
    pub id: GUID,
    pub owner: GUID,
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct FarGetDialogItem {
    pub struct_size: size_t,
    pub size: size_t,
    pub  item: *mut FarDialogItem,
}

bitflags! {
    pub struct FARDIALOGFLAGS: c_ulonglong {
        const FDLG_WARNING = 0x0000000000000001;
        const FDLG_SMALLDIALOG = 0x0000000000000002;
        const FDLG_NODRAWSHADOW = 0x0000000000000004;
        const FDLG_NODRAWPANEL = 0x0000000000000008;
        const FDLG_KEEPCONSOLETITLE = 0x0000000000000010;
        const FDLG_NONMODAL = 0x0000000000000020;
        const FDLG_NONE = 0;
    }
}

pub type FARWINDOWPROC = extern fn(h_dlg: HANDLE, msg: intptr_t, param1: intptr_t, param2: *const c_void) -> intptr_t;

pub type FARAPISENDDLGMESSAGE = extern fn(h_dlg: HANDLE, msg: intptr_t, param1: intptr_t, param2: *const c_void) -> intptr_t;

pub type FARAPIDEFDLGPROC = extern fn(h_dlg: HANDLE, msg: intptr_t, param1: intptr_t, param2: *const c_void) -> intptr_t;

pub type FARAPIDIALOGINIT = extern fn(plugin_id: *const GUID, id: *const GUID, x1: intptr_t,
    y1: intptr_t, x2: intptr_t, y2: intptr_t, help_topic: *const wchar_t,
    item: *const FarDialogItem, items_number: size_t, reserved: intptr_t, flags: FARDIALOGFLAGS,
    dlg_proc: FARWINDOWPROC, param: *const c_void) -> HANDLE;

pub type FARAPIDIALOGRUN = extern fn(h_dlg: HANDLE) -> intptr_t;

pub type FARAPIDIALOGFREE = extern fn(h_dlg: HANDLE) -> intptr_t;

#[repr(C)] #[derive(Clone, Copy)]
pub struct FarKey {
    pub virtual_key_code: WORD,
    pub control_key_state: DWORD,
}

bitflags! {
    pub struct MENUITEMFLAGS: c_ulonglong {
        const MIF_SELECTED   = 0x000000000010000;
        const MIF_CHECKED    = 0x000000000020000;
        const MIF_SEPARATOR  = 0x000000000040000;
        const MIF_DISABLE    = 0x000000000080000;
        const MIF_GRAYED     = 0x000000000100000;
        const MIF_HIDDEN     = 0x000000000200000;
        const MIF_SUBMENU    = 0x000000000400000;
        const MIF_NONE       = 0;
    }
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct FarMenuItem {
    pub flags: MENUITEMFLAGS,
    pub text: *const wchar_t,
    pub accel_key: FarKey,
    pub user_data: intptr_t,
    pub reserved: [intptr_t; 2],
}

bitflags! {
    pub struct FARMENUFLAGS: c_ulonglong {
        const FMENU_SHOWAMPERSAND        = 0x0000000000000001;
        const FMENU_WRAPMODE             = 0x0000000000000002;
        const FMENU_AUTOHIGHLIGHT        = 0x0000000000000004;
        const FMENU_REVERSEAUTOHIGHLIGHT = 0x0000000000000008;
        const FMENU_CHANGECONSOLETITLE   = 0x0000000000000010;
        const FMENU_SHOWNOBOX            = 0x0000000000000020;
        const FMENU_NONE                 = 0;
    }
}

pub type FARAPIMENU = extern fn(
    plugin_id: *const GUID,
    id: *const GUID,
    x: intptr_t,
    y: intptr_t,
    max_height: intptr_t,
    flags: FARMENUFLAGS,
    title: *const wchar_t,
    bottom: *const wchar_t,
    help_topic: *const wchar_t,
    break_keys: *const FarKey,
    break_code: *const intptr_t,
    item: *const FarMenuItem,
    items_number: size_t
    ) -> intptr_t;

bitflags! {
    pub struct PLUGINPANELITEMFLAGS : c_ulonglong {
        const PPIF_SELECTED               = 0x0000000040000000;
        const PPIF_PROCESSDESCR           = 0x0000000080000000;
        const PPIF_NONE                   = 0;
    }
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct FarPanelItemFreeInfo {
    pub struct_size: size_t,
    pub h_plugin: HANDLE,
}

pub type FARPANELITEMFREECALLBACK = extern fn(user_data: *const c_void, info: *const FarPanelItemFreeInfo);

#[repr(C)] #[derive(Clone, Copy)]
pub struct UserDataItem {
    pub data: *mut c_void,
    pub free_data: Option<FARPANELITEMFREECALLBACK>,
}

impl Debug for UserDataItem {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.pad("UserDataItem")
    }
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct PluginPanelItem {
    pub creation_time: FILETIME,
    pub last_access_time: FILETIME,
    pub last_write_time: FILETIME,
    pub change_time: FILETIME,
    pub file_size: c_ulonglong,
    pub allocation_size: c_ulonglong,
    pub file_name: *const wchar_t,
    pub alternate_file_name: *const wchar_t,
    pub description: *const wchar_t,
    pub owner: *const wchar_t,
    pub custom_column_data: *const *const wchar_t, // TODO check it
    pub custom_column_number: size_t,
    pub flags: PLUGINPANELITEMFLAGS,
    pub user_data: UserDataItem,
    pub file_attributes: uintptr_t,
    pub number_of_links: uintptr_t,
    pub crc32: uintptr_t,
    pub reserved: [intptr_t; 2],
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct FarGetPluginPanelItem {
    pub struct_size: size_t,
    pub size: size_t,
    pub item: *mut PluginPanelItem,
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct SortingPanelItem {
    pub creation_time: FILETIME,
    pub last_access_time: FILETIME,
    pub last_write_time: FILETIME,
    pub change_time: FILETIME,
    pub file_size: c_ulonglong,
    pub allocation_size: c_ulonglong,
    pub file_name: *const wchar_t,
    pub alternate_file_name: *const wchar_t,
    pub description: *const wchar_t,
    pub owner: *const wchar_t,
    pub custom_column_data: *const *const wchar_t, // TODO check it
    pub custom_column_number: size_t,
    pub flags: PLUGINPANELITEMFLAGS,
    pub user_data: UserDataItem,
    pub file_attributes: uintptr_t,
    pub number_of_links: uintptr_t,
    pub crc32: uintptr_t,
    pub position: intptr_t,
    pub sortgroup: intptr_t,
    pub number_of_streams: uintptr_t,
    pub streams_size: c_ulonglong,
}

bitflags! {
    pub struct PANELINFOFLAGS: c_ulonglong {
        const PFLAGS_SHOWHIDDEN         = 0x0000000000000001;
        const PFLAGS_HIGHLIGHT          = 0x0000000000000002;
        const PFLAGS_REVERSESORTORDER   = 0x0000000000000004;
        const PFLAGS_USESORTGROUPS      = 0x0000000000000008;
        const PFLAGS_SELECTEDFIRST      = 0x0000000000000010;
        const PFLAGS_REALNAMES          = 0x0000000000000020;
        const PFLAGS_PANELLEFT          = 0x0000000000000080;
        const PFLAGS_DIRECTORIESFIRST   = 0x0000000000000100;
        const PFLAGS_USECRC32           = 0x0000000000000200;
        const PFLAGS_PLUGIN             = 0x0000000000000800;
        const PFLAGS_VISIBLE            = 0x0000000000001000;
        const PFLAGS_FOCUS              = 0x0000000000002000;
        const PFLAGS_ALTERNATIVENAMES   = 0x0000000000004000;
        const PFLAGS_SHORTCUT           = 0x0000000000008000;
        const PFLAGS_NONE               = 0;
    }
}

#[repr(C)] #[derive(Clone, Copy)]
pub enum PANELINFOTYPE {
    PTYPE_FILEPANEL                 = 0,
    PTYPE_TREEPANEL                 = 1,
    PTYPE_QVIEWPANEL                = 2,
    PTYPE_INFOPANEL                 = 3,
}

#[repr(C)] #[derive(Clone, Copy)]
pub enum OPENPANELINFO_SORTMODES {
    SM_DEFAULT                   =  0,
    SM_UNSORTED                  =  1,
    SM_NAME                      =  2,
    SM_EXT                       =  3,
    SM_MTIME                     =  4,
    SM_CTIME                     =  5,
    SM_ATIME                     =  6,
    SM_SIZE                      =  7,
    SM_DESCR                     =  8,
    SM_OWNER                     =  9,
    SM_COMPRESSEDSIZE            = 10,
    SM_NUMLINKS                  = 11,
    SM_NUMSTREAMS                = 12,
    SM_STREAMSSIZE               = 13,
    SM_FULLNAME                  = 14,
    SM_CHTIME                    = 15,
    SM_COUNT                     = 16,
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct PanelInfo {
    pub struct_size: size_t,
    pub plugin_handle: HANDLE,
    pub owner_guid: GUID,
    pub flags: PANELINFOFLAGS,
    pub items_number: size_t,
    pub selected_items_number: size_t,
    pub panel_rect: RECT,
    pub current_item: size_t,
    pub top_panel_item: size_t,
    pub view_mode: intptr_t,
    pub panel_type: PANELINFOTYPE,
    pub sort_mode: OPENPANELINFO_SORTMODES,
}


#[repr(C)] #[derive(Clone, Copy)]
pub struct PanelRedrawInfo {
    pub struct_size: size_t,
    pub current_item: size_t,
    pub top_panel_item: size_t,
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct CmdLineSelect {
    pub struct_size: size_t,
    pub sel_start: intptr_t,
    pub sel_end: intptr_t,
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct FarPanelDirectory {
    pub struct_size: size_t,
    pub name: *const wchar_t,
    pub param: *const wchar_t,
    pub plugin_id: GUID,
    pub file: *const wchar_t,
}

pub const PANEL_NONE: HANDLE = (-1 as intptr_t) as HANDLE;
pub const PANEL_ACTIVE: HANDLE = (-1 as intptr_t) as HANDLE;
pub const PANEL_PASSIVE: HANDLE = (-2 as intptr_t) as HANDLE;
pub const PANEL_STOP: HANDLE = (-1 as intptr_t) as HANDLE;

#[repr(C)] #[derive(Clone, Copy)]
pub enum FILE_CONTROL_COMMANDS {
    FCTL_CLOSEPANEL                 = 0,
    FCTL_GETPANELINFO               = 1,
    FCTL_UPDATEPANEL                = 2,
    FCTL_REDRAWPANEL                = 3,
    FCTL_GETCMDLINE                 = 4,
    FCTL_SETCMDLINE                 = 5,
    FCTL_SETSELECTION               = 6,
    FCTL_SETVIEWMODE                = 7,
    FCTL_INSERTCMDLINE              = 8,
    FCTL_SETUSERSCREEN              = 9,
    FCTL_SETPANELDIRECTORY          = 10,
    FCTL_SETCMDLINEPOS              = 11,
    FCTL_GETCMDLINEPOS              = 12,
    FCTL_SETSORTMODE                = 13,
    FCTL_SETSORTORDER               = 14,
    FCTL_SETCMDLINESELECTION        = 15,
    FCTL_GETCMDLINESELECTION        = 16,
    FCTL_CHECKPANELSEXIST           = 17,
    FCTL_GETUSERSCREEN              = 19,
    FCTL_ISACTIVEPANEL              = 20,
    FCTL_GETPANELITEM               = 21,
    FCTL_GETSELECTEDPANELITEM       = 22,
    FCTL_GETCURRENTPANELITEM        = 23,
    FCTL_GETPANELDIRECTORY          = 24,
    FCTL_GETCOLUMNTYPES             = 25,
    FCTL_GETCOLUMNWIDTHS            = 26,
    FCTL_BEGINSELECTION             = 27,
    FCTL_ENDSELECTION               = 28,
    FCTL_CLEARSELECTION             = 29,
    FCTL_SETDIRECTORIESFIRST        = 30,
    FCTL_GETPANELFORMAT             = 31,
    FCTL_GETPANELHOSTFILE           = 32,
    FCTL_GETPANELPREFIX             = 34,
    FCTL_SETACTIVEPANEL             = 35,
}

pub type FARAPITEXT = extern fn(
    x: intptr_t,
    y: intptr_t,
    color: *const FarColor,
    str: *const wchar_t,
    );

pub type FARAPISAVESCREEN = extern fn(x1: intptr_t, y1: intptr_t, x2: intptr_t, y2: intptr_t) -> HANDLE;

pub type FARAPIRESTORESCREEN = extern fn(h_screen: HANDLE);

pub type FARAPIGETDIRLIST = extern fn(
    dir: *const wchar_t,
    p_panel_item: *mut *mut PluginPanelItem,
    p_items_number: *mut size_t,
) -> intptr_t;

pub type FARAPIGETPLUGINDIRLIST = extern fn(
    plugin_id: *const GUID,
    h_panel: HANDLE,
    dir: *const wchar_t,
    p_panel_item: *mut *mut PluginPanelItem,
    p_items_number: *mut size_t,
) -> intptr_t;

pub type FARAPIFREEDIRLIST = extern fn(panel_item: *mut PluginPanelItem, n_items_number: size_t);
pub type FARAPIFREEPLUGINDIRLIST = extern fn(h_panel: HANDLE, panel_item: *mut PluginPanelItem, n_items_number: size_t);

bitflags! {
    pub struct VIEWER_FLAGS: c_ulonglong {
        const VF_NONMODAL              = 0x0000000000000001;
        const VF_DELETEONCLOSE         = 0x0000000000000002;
        const VF_ENABLE_F6             = 0x0000000000000004;
        const VF_DISABLEHISTORY        = 0x0000000000000008;
        const VF_IMMEDIATERETURN       = 0x0000000000000100;
        const VF_DELETEONLYFILEONCLOSE = 0x0000000000000200;
        const VF_NONE                  = 0;
    }
}

pub type FARAPIVIEWER = extern fn(
    file_name: *const wchar_t,
    title: *const wchar_t,
    x1: intptr_t,
    y1: intptr_t,
    x2: intptr_t,
    y2: intptr_t,
    flags: VIEWER_FLAGS,
    code_page: uintptr_t,
) -> intptr_t;

bitflags! {
    pub struct EDITOR_FLAGS: c_ulonglong {
        const EF_NONMODAL              = 0x0000000000000001;
        const EF_CREATENEW             = 0x0000000000000002;
        const EF_ENABLE_F6             = 0x0000000000000004;
        const EF_DISABLEHISTORY        = 0x0000000000000008;
        const EF_DELETEONCLOSE         = 0x0000000000000010;
        const EF_IMMEDIATERETURN       = 0x0000000000000100;
        const EF_DELETEONLYFILEONCLOSE = 0x0000000000000200;
        const EF_LOCKED                = 0x0000000000000400;
        const EF_DISABLESAVEPOS        = 0x0000000000000800;
        const EF_OPENMODE_MASK         = 0x00000000F0000000;
        const EF_OPENMODE_QUERY        = 0x0000000000000000;
        const EF_OPENMODE_NEWIFOPEN    = 0x0000000010000000;
        const EF_OPENMODE_USEEXISTING  = 0x0000000020000000;
        const EF_OPENMODE_BREAKIFOPEN  = 0x0000000030000000;
        const EF_OPENMODE_RELOADIFOPEN = 0x0000000040000000;
        const EN_NONE                  = 0;
    }
}

#[repr(C)] #[derive(Clone, Copy)]
pub enum EDITOR_EXITCODE {
    EEC_OPEN_ERROR          = 0,
    EEC_MODIFIED            = 1,
    EEC_NOT_MODIFIED        = 2,
    EEC_LOADING_INTERRUPTED = 3,
}

pub type FARAPIEDITOR = extern fn(
    file_name: *const wchar_t,
    title: *const wchar_t,
    x1: intptr_t,
    y1: intptr_t,
    x2: intptr_t,
    y2: intptr_t,
    flags: EDITOR_FLAGS,
    start_line: intptr_t,
    start_char: intptr_t,
    code_page: uintptr_t,
) -> intptr_t;

pub type FARAPIGETMSG = extern "system" fn(plugin_id: *const GUID, msg_id: intptr_t) -> *const wchar_t;

bitflags! {
    pub struct FARHELPFLAGS: c_ulonglong {
        const FHELP_NOSHOWERROR = 0x0000000080000000;
        const FHELP_SELFHELP    = 0x0000000000000000;
        const FHELP_FARHELP     = 0x0000000000000001;
        const FHELP_CUSTOMFILE  = 0x0000000000000002;
        const FHELP_CUSTOMPATH  = 0x0000000000000004;
        const FHELPGUID        = 0x0000000000000008;
        const FHELP_USECONTENTS = 0x0000000040000000;
        const FHELP_NONE        = 0;
    }
}

pub type FARAPISHOWHELP = extern "system" fn(
    module_name: *const wchar_t,
    topic: *const wchar_t,
    flags: FARHELPFLAGS,
) -> BOOL;

#[repr(C)] #[derive(Clone, Copy)]
pub enum ADVANCED_CONTROL_COMMANDS {
    ACTL_GETFARMANAGERVERSION       = 0,
    ACTL_WAITKEY                    = 2,
    ACTL_GETCOLOR                   = 3,
    ACTL_GETARRAYCOLOR              = 4,
    ACTL_GETWINDOWINFO              = 6,
    ACTL_GETWINDOWCOUNT             = 7,
    ACTL_SETCURRENTWINDOW           = 8,
    ACTL_COMMIT                     = 9,
    ACTL_GETFARHWND                 = 10,
    ACTL_SETARRAYCOLOR              = 16,
    ACTL_REDRAWALL                  = 19,
    ACTL_SYNCHRO                    = 20,
    ACTL_SETPROGRESSSTATE           = 21,
    ACTL_SETPROGRESSVALUE           = 22,
    ACTL_QUIT                       = 23,
    ACTL_GETFARRECT                 = 24,
    ACTL_GETCURSORPOS               = 25,
    ACTL_SETCURSORPOS               = 26,
    ACTL_PROGRESSNOTIFY             = 27,
    ACTL_GETWINDOWTYPE              = 28,
}

#[repr(C)] #[derive(Clone, Copy)]
pub enum FAR_MACRO_CONTROL_COMMANDS {
    MCTL_LOADALL           = 0,
    MCTL_SAVEALL           = 1,
    MCTL_SENDSTRING        = 2,
    MCTL_GETSTATE          = 5,
    MCTL_GETAREA           = 6,
    MCTL_ADDMACRO          = 7,
    MCTL_DELMACRO          = 8,
    MCTL_GETLASTERROR      = 9,
    MCTL_EXECSTRING        = 10,
}

bitflags! {
    pub struct FARKEYMACROFLAGS: c_ulonglong {
        const KMFLAGS_SILENTCHECK         = 0x0000000000000001;
        const KMFLAGS_NOSENDKEYSTOPLUGINS = 0x0000000000000002;
        const KMFLAGS_ENABLEOUTPUT        = 0x0000000000000004;
        const KMFLAGS_LANGMASK            = 0x0000000000000070; // 3 bits reserved for 8 languages
        const KMFLAGS_LUA                 = 0x0000000000000000;
        const KMFLAGS_MOONSCRIPT          = 0x0000000000000010;
        const KMFLAGS_NONE                = 0;
    }
}

#[repr(C)] #[derive(Clone, Copy)]
pub enum FARMACROSENDSTRINGCOMMAND {
    MSSC_POST              =0,
    MSSC_CHECK             =2,
}

#[repr(C)] #[derive(Clone, Copy)]
pub enum FARMACROAREA {
    MACROAREA_OTHER                      =   0,   // Reserved
    MACROAREA_SHELL                      =   1,   // File panels
    MACROAREA_VIEWER                     =   2,   // Internal viewer program
    MACROAREA_EDITOR                     =   3,   // Editor
    MACROAREA_DIALOG                     =   4,   // Dialogs
    MACROAREA_SEARCH                     =   5,   // Quick search in panels
    MACROAREA_DISKS                      =   6,   // Menu of disk selection
    MACROAREA_MAINMENU                   =   7,   // Main menu
    MACROAREA_MENU                       =   8,   // Other menus
    MACROAREA_HELP                       =   9,   // Help system
    MACROAREA_INFOPANEL                  =  10,   // Info panel
    MACROAREA_QVIEWPANEL                 =  11,   // Quick view panel
    MACROAREA_TREEPANEL                  =  12,   // Folders tree panel
    MACROAREA_FINDFOLDER                 =  13,   // Find folder
    MACROAREA_USERMENU                   =  14,   // User menu
    MACROAREA_SHELLAUTOCOMPLETION        =  15,   // Autocompletion list in command line
    MACROAREA_DIALOGAUTOCOMPLETION       =  16,   // Autocompletion list in dialogs
    MACROAREA_GRABBER                    =  17,   // Mode of copying text from the screen
    MACROAREA_DESKTOP                    =  18,   // Desktop
    MACROAREA_COMMON                     = 255,
}

#[repr(C)] #[derive(Clone, Copy)]
pub enum FARMACROSTATE {
    MACROSTATE_NOMACRO          = 0,
    MACROSTATE_EXECUTING        = 1,
    MACROSTATE_EXECUTING_COMMON = 2,
    MACROSTATE_RECORDING        = 3,
    MACROSTATE_RECORDING_COMMON = 4,
}

#[repr(C)] #[derive(Clone, Copy)]
pub enum FARMACROPARSEERRORCODE {
    MPEC_SUCCESS = 0,
    MPEC_ERROR   = 1,
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct MacroParseResult {
    pub struct_size: size_t,
    pub err_code: DWORD,
    pub err_pos: COORD,
    pub err_src: *const wchar_t,
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct MacroSendMacroText {
    pub struct_size: size_t,
    pub flags: FARKEYMACROFLAGS,
    pub a_key: INPUT_RECORD,
    pub sequence_text: *const wchar_t,
}

bitflags! {
    pub struct FARADDKEYMACROFLAGS: c_ulonglong {
        const AKMFLAGS_NONE                = 0;
    }
}

pub type FARMACROCALLBACK = extern "system" fn(id: *const c_void, flags: FARADDKEYMACROFLAGS) -> intptr_t;

#[repr(C)] #[derive(Clone, Copy)]
pub struct MacroAddMacro {
    pub struct_size: size_t,
    pub id: *mut c_void,
    pub sequence_text: *const wchar_t,
    pub description: *const wchar_t,
    pub flags: FARKEYMACROFLAGS,
    pub a_key: INPUT_RECORD,
    pub area: FARMACROAREA,
    pub callback: FARMACROCALLBACK,
    pub priority: intptr_t,
}

#[repr(C)] #[derive(Clone, Copy)]
pub enum FARMACROVARTYPE {
    FMVT_UNKNOWN                = 0,
    FMVT_INTEGER                = 1,
    FMVT_STRING                 = 2,
    FMVT_DOUBLE                 = 3,
    FMVT_BOOLEAN                = 4,
    FMVT_BINARY                 = 5,
    FMVT_POINTER                = 6,
    FMVT_NIL                    = 7,
    FMVT_ARRAY                  = 8,
    FMVT_PANEL                  = 9,
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct FarMacroValue_Binary {
    pub data: *mut c_void,
    pub size: size_t,
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct FarMacroValue_Array {
    pub values: *mut FarMacroValue,
    pub count: size_t,
}

#[repr(C)]
pub struct FarMacroValue {
    pub var_type: FARMACROVARTYPE,
    pub value: FarMacroValueValue,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union FarMacroValueValue {
    pub integer: c_longlong,
    pub boolean: c_longlong,
    pub double: c_double,
    pub string: *const wchar_t,
    pub pointer: *mut c_void,
    pub binary: FarMacroValueValueBinary,
    pub array: FarMacroValueValueArray,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FarMacroValueValueBinary {
    pub data: *mut c_void,
    pub size: size_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FarMacroValueValueArray {
    pub values: *mut FarMacroValue,
    pub count: size_t,
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct FarMacroCall {
    pub struct_size: size_t,
    pub count: size_t,
    pub values: *mut FarMacroValue,
    pub callback: extern "system" fn(callback_data: *const c_void, values: *const FarMacroValue, count: size_t),
    pub callback_data: *mut c_void,
}

#[repr(C)]
pub struct FarGetValue {
    pub struct_size: size_t,
    pub value_type: intptr_t,
    pub value: FarMacroValue,
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct MacroExecuteString {
    pub struct_size: size_t,
    pub flags: FARKEYMACROFLAGS,
    pub sequence_text: *const wchar_t,
    pub in_count: size_t,
    pub in_values: *mut FarMacroValue,
    pub out_count: size_t,
    pub out_values: *const FarMacroValue,
}

pub struct FarMacroLoad
{
    pub struct_size: size_t,
    pub path: *const wchar_t,
    pub flags: c_ulonglong,
}

bitflags! {
    pub struct FARSETCOLORFLAGS: c_ulonglong {
        const FSETCLR_REDRAW                 = 0x0000000000000001;
        const FSETCLR_NONE                   = 0;
    }
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct FarSetColors {
    pub struct_size: size_t,
    pub flags: FARSETCOLORFLAGS,
    pub start_index: size_t,
    pub colors_count: size_t,
    pub colors: *mut FarColor,
}

#[repr(C)] #[derive(Clone, Copy)]
pub enum WINDOWINFO_TYPE {
    WTYPE_PANELS                    = 1,
    WTYPE_VIEWER                    = 2,
    WTYPE_EDITOR                    = 3,
    WTYPE_DIALOG                    = 4,
    WTYPE_VMENU                     = 5,
    WTYPE_HELP                      = 6,
    WTYPE_COMBOBOX                  = 7,
    WTYPE_GRABBER                   = 8,
    WTYPE_HMENU                     = 9,
}

bitflags! {
    pub struct WINDOWINFO_FLAGS: c_ulonglong {
        const WIF_MODIFIED = 0x0000000000000001;
        const WIF_CURRENT  = 0x0000000000000002;
        const WIF_MODAL    = 0x0000000000000004;
        const WIF_NONE     = 0;
    }
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct WindowInfo {
    pub struct_size: size_t,
    pub id: intptr_t,
    pub type_name: *mut wchar_t,
    pub name: *mut wchar_t,
    pub type_name_size: intptr_t,
    pub name_size: intptr_t,
    pub pos: intptr_t,
    pub info_type: WINDOWINFO_TYPE,
    pub flags: WINDOWINFO_FLAGS,
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct WindowType {
    pub struct_size: size_t,
    pub info_type: WINDOWINFO_TYPE,
}

#[repr(C)] #[derive(Clone, Copy)]
pub enum TASKBARPROGRESSTATE {
    TBPS_NOPROGRESS    = 0x0,
    TBPS_INDETERMINATE = 0x1,
    TBPS_NORMAL        = 0x2,
    TBPS_ERROR         = 0x4,
    TBPS_PAUSED        = 0x8,
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct ProgressValue {
    pub struct_size: size_t,
    pub completed: c_ulonglong,
    pub total: c_ulonglong,
}

#[repr(C)] #[derive(Clone, Copy)]
pub enum VIEWER_CONTROL_COMMANDS {
    VCTL_GETINFO                    = 0,
    VCTL_QUIT                       = 1,
    VCTL_REDRAW                     = 2,
    VCTL_SETKEYBAR                  = 3,
    VCTL_SETPOSITION                = 4,
    VCTL_SELECT                     = 5,
    VCTL_SETMODE                    = 6,
    VCTL_GETFILENAME                = 7,
}

bitflags! {
    pub struct VIEWER_OPTIONS: c_ulonglong {
        const VOPT_SAVEFILEPOSITION   = 0x0000000000000001;
        const VOPT_AUTODETECTCODEPAGE = 0x0000000000000002;
        const VOPT_SHOWTITLEBAR       = 0x0000000000000004;
        const VOPT_SHOWKEYBAR         = 0x0000000000000008;
        const VOPT_SHOWSCROLLBAR      = 0x0000000000000010;
        const VOPT_QUICKVIEW          = 0x0000000000000020;
        const VOPT_NONE               = 0;
    }
}

#[repr(C)] #[derive(Clone, Copy)]
pub enum VIEWER_SETMODE_TYPES {
    VSMT_VIEWMODE                   = 0,
    VSMT_WRAP                       = 1,
    VSMT_WORDWRAP                   = 2,
}

bitflags! {
    pub struct VIEWER_SETMODEFLAGS_TYPES: c_ulonglong {
        const VSMFL_REDRAW    = 0x0000000000000001;
        const VSMFL_NONE      = 0;
    }
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct ViewerSetMode {
    pub struct_size: size_t,
    pub mode_type: VIEWER_SETMODE_TYPES,
    pub param: ViewerSetModeParam,
    pub flags: VIEWER_SETMODEFLAGS_TYPES,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union ViewerSetModeParam {
    pub i_param: intptr_t,
    pub wsz_param: *mut wchar_t,
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct ViewerSelect {
    pub struct_size: size_t,
    pub block_start_pos: c_longlong,
    pub block_len: c_longlong,
}

bitflags! {
    pub struct VIEWER_SETPOS_FLAGS: c_ulonglong {
        const VSP_NOREDRAW    = 0x0000000000000001;
        const VSP_PERCENT     = 0x0000000000000002;
        const VSP_RELATIVE    = 0x0000000000000004;
        const VSP_NORETNEWPOS = 0x0000000000000008;
        const VSP_NONE        = 0;
    }
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct ViewerSetPosition {
    pub struct_size: size_t,
    pub flags: VIEWER_SETPOS_FLAGS,
    pub start_pos: c_longlong,
    pub left_pos: c_longlong,
}

bitflags! {
    pub struct VIEWER_MODE_FLAGS: c_ulonglong {
        const VMF_WRAP     = 0x0000000000000001;
        const VMF_WORDWRAP = 0x0000000000000002;
        const VMF_NONE     = 0;
    }
}

#[repr(C)] #[derive(Clone, Copy)]
pub enum VIEWER_MODE_TYPE {
    VMT_TEXT    = 0,
    VMT_HEX     = 1,
    VMT_DUMP    = 2,
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct ViewerMode {
    pub code_page: uintptr_t,
    pub flags: VIEWER_MODE_FLAGS,
    pub view_mode: VIEWER_MODE_TYPE,
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct ViewerInfo {
    pub struct_size: size_t,
    pub viewer_id: intptr_t,
    pub tab_size: intptr_t,
    pub cur_mode: ViewerMode,
    pub file_size: c_longlong,
    pub file_pos: c_longlong,
    pub left_pos: c_longlong,
    pub options: VIEWER_OPTIONS,
    pub window_size_x: intptr_t,
    pub window_size_y: intptr_t,
}

#[repr(C)] #[derive(Clone, Copy)]
pub enum VIEWER_EVENTS {
    VE_READ       = 0,
    VE_CLOSE      = 1,

    VE_GOTFOCUS   = 6,
    VE_KILLFOCUS  = 7,
}

#[repr(C)] #[derive(Clone, Copy)]
pub enum EDITOR_EVENTS {
    EE_READ       = 0,
    EE_SAVE       = 1,
    EE_REDRAW     = 2,
    EE_CLOSE      = 3,

    EE_GOTFOCUS   = 6,
    EE_KILLFOCUS  = 7,
    EE_CHANGE     = 8,
}

#[repr(C)] #[derive(Clone, Copy)]
pub enum DIALOG_EVENTS {
    DE_DLGPROCINIT    = 0,
    DE_DEFDLGPROCINIT = 1,
    DE_DLGPROCEND     = 2,
}

#[repr(C)] #[derive(Clone, Copy)]
pub enum SYNCHRO_EVENTS {
    SE_COMMONSYNCHRO  = 0,
}

pub const EEREDRAW_ALL: *const c_void = ptr::null();
pub const CURRENT_EDITOR: i32 = -1;

#[repr(C)] #[derive(Clone, Copy)]
pub enum EDITOR_CONTROL_COMMANDS {
    ECTL_GETSTRING                  = 0,
    ECTL_SETSTRING                  = 1,
    ECTL_INSERTSTRING               = 2,
    ECTL_DELETESTRING               = 3,
    ECTL_DELETECHAR                 = 4,
    ECTL_INSERTTEXT                 = 5,
    ECTL_GETINFO                    = 6,
    ECTL_SETPOSITION                = 7,
    ECTL_SELECT                     = 8,
    ECTL_REDRAW                     = 9,
    ECTL_TABTOREAL                  = 10,
    ECTL_REALTOTAB                  = 11,
    ECTL_EXPANDTABS                 = 12,
    ECTL_SETTITLE                   = 13,
    ECTL_READINPUT                  = 14,
    ECTL_PROCESSINPUT               = 15,
    ECTL_ADDCOLOR                   = 16,
    ECTL_GETCOLOR                   = 17,
    ECTL_SAVEFILE                   = 18,
    ECTL_QUIT                       = 19,
    ECTL_SETKEYBAR                  = 20,

    ECTL_SETPARAM                   = 22,
    ECTL_GETBOOKMARKS               = 23,
    ECTL_DELETEBLOCK                = 25,
    ECTL_ADDSESSIONBOOKMARK         = 26,
    ECTL_PREVSESSIONBOOKMARK        = 27,
    ECTL_NEXTSESSIONBOOKMARK        = 28,
    ECTL_CLEARSESSIONBOOKMARKS      = 29,
    ECTL_DELETESESSIONBOOKMARK      = 30,
    ECTL_GETSESSIONBOOKMARKS        = 31,
    ECTL_UNDOREDO                   = 32,
    ECTL_GETFILENAME                = 33,
    ECTL_DELCOLOR                   = 34,
    ECTL_SUBSCRIBECHANGEEVENT       = 36,
    ECTL_UNSUBSCRIBECHANGEEVENT     = 37,
    ECTL_GETTITLE                   = 38,
}

#[repr(C)] #[derive(Clone, Copy)]
pub enum EDITOR_SETPARAMETER_TYPES {
    ESPT_TABSIZE                    = 0,
    ESPT_EXPANDTABS                 = 1,
    ESPT_AUTOINDENT                 = 2,
    ESPT_CURSORBEYONDEOL            = 3,
    ESPT_CHARCODEBASE               = 4,
    ESPT_CODEPAGE                   = 5,
    ESPT_SAVEFILEPOSITION           = 6,
    ESPT_LOCKMODE                   = 7,
    ESPT_SETWORDDIV                 = 8,
    ESPT_GETWORDDIV                 = 9,
    ESPT_SHOWWHITESPACE             = 10,
    ESPT_SETBOM                     = 11,
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct EditorSetParameter {
    pub struct_size: size_t,
    pub parameter_type: EDITOR_SETPARAMETER_TYPES,
    pub param: EditorSetParameterParam,
    pub flags: c_ulonglong,
    pub size: size_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union EditorSetParameterParam {
    pub i_param: intptr_t,
    pub wsz_param: *mut wchar_t,
    pub reserved: intptr_t
}

#[repr(C)] #[derive(Clone, Copy)]
pub enum EDITOR_UNDOREDO_COMMANDS {
    EUR_BEGIN                       = 0,
    EUR_END                         = 1,
    EUR_UNDO                        = 2,
    EUR_REDO                        = 3,
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct EditorUndoRedo {
    pub struct_size: size_t,
    pub command: EDITOR_UNDOREDO_COMMANDS,
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct EditorGetString {
    pub struct_size: size_t,
    pub string_number: intptr_t,
    pub string_length: intptr_t,
    pub string_text: *const wchar_t,
    pub string_eol: *const wchar_t,
    pub sel_start: intptr_t,
    pub sel_end: intptr_t,
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct EditorSetString {
    pub struct_size: size_t,
    pub string_number: intptr_t,
    pub string_length: intptr_t,
    pub string_text: *const wchar_t,
    pub string_eol: *const wchar_t,
}

#[repr(C)] #[derive(Clone, Copy)]
pub enum EXPAND_TABS {
    EXPAND_NOTABS                   = 0,
    EXPAND_ALLTABS                  = 1,
    EXPAND_NEWTABS                  = 2,
}

#[repr(C)] #[derive(Clone, Copy)]
pub enum EDITOR_OPTIONS {
    EOPT_EXPANDALLTABS     = 0x00000001,
    EOPT_PERSISTENTBLOCKS  = 0x00000002,
    EOPT_DELREMOVESBLOCKS  = 0x00000004,
    EOPT_AUTOINDENT        = 0x00000008,
    EOPT_SAVEFILEPOSITION  = 0x00000010,
    EOPT_AUTODETECTCODEPAGE= 0x00000020,
    EOPT_CURSORBEYONDEOL   = 0x00000040,
    EOPT_EXPANDONLYNEWTABS = 0x00000080,
    EOPT_SHOWWHITESPACE    = 0x00000100,
    EOPT_BOM               = 0x00000200,
    EOPT_SHOWLINEBREAK     = 0x00000400,
    EOPT_SHOWTITLEBAR      = 0x00000800,
    EOPT_SHOWKEYBAR        = 0x00001000,
    EOPT_SHOWSCROLLBAR     = 0x00002000,
}

#[repr(C)] #[derive(Clone, Copy)]
pub enum EDITOR_BLOCK_TYPES {
    BTYPE_NONE                      = 0,
    BTYPE_STREAM                    = 1,
    BTYPE_COLUMN                    = 2,
}

#[repr(C)] #[derive(Clone, Copy)]
pub enum EDITOR_CURRENTSTATE {
    ECSTATE_MODIFIED       = 0x00000001,
    ECSTATE_SAVED          = 0x00000002,
    ECSTATE_LOCKED         = 0x00000004,
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct EditorInfo {
    pub struct_size: size_t,
    pub editor_id: intptr_t,
    pub window_size_x: intptr_t,
    pub window_size_y: intptr_t,
    pub total_lines: intptr_t,
    pub cur_line: intptr_t,
    pub cur_pos: intptr_t,
    pub cur_tab_pos: intptr_t,
    pub top_screen_line: intptr_t,
    pub left_pos: intptr_t,
    pub overtype: intptr_t,
    pub block_type: intptr_t,
    pub block_start_line: intptr_t,
    pub options: uintptr_t,
    pub tab_size: intptr_t,
    pub bookmark_count: size_t,
    pub session_bookmark_count: size_t,
    pub cur_state: uintptr_t,
    pub code_page: uintptr_t,
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct EditorBookmarks {
    pub struct_size: size_t,
    pub size: size_t,
    pub count: size_t,
    pub line: *mut intptr_t,
    pub cursor: *mut intptr_t,
    pub screen_line: *mut intptr_t,
    pub left_pos: *mut intptr_t,
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct EditorSetPosition {
    pub struct_size: size_t,
    pub cur_line: intptr_t,
    pub cur_pos: intptr_t,
    pub cur_tab_pos: intptr_t,
    pub top_screen_line: intptr_t,
    pub left_pos: intptr_t,
    pub overtype: intptr_t,
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct EditorSelect {
    pub struct_size: size_t,
    pub block_type: intptr_t,
    pub block_start_line: intptr_t,
    pub block_start_pos: intptr_t,
    pub block_width: intptr_t,
    pub block_height: intptr_t,
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct EditorConvertPos {
    pub struct_size: size_t,
    pub string_number: intptr_t,
    pub src_pos: intptr_t,
    pub dest_pos: intptr_t,
}

bitflags! {
    pub struct EDITORCOLORFLAGS: c_ulonglong {
        const ECF_TABMARKFIRST   = 0x0000000000000001;
        const ECF_TABMARKCURRENT = 0x0000000000000002;
        const ECF_AUTODELETE     = 0x0000000000000004;
        const ECF_NONE           = 0;
    }
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct EditorColor {
    pub struct_size: size_t,
    pub string_number: intptr_t,
    pub color_item: intptr_t,
    pub start_pos: intptr_t,
    pub end_pos: intptr_t,
    pub priority: uintptr_t,
    pub flags: EDITORCOLORFLAGS,
    pub color: FarColor,
    pub owner: GUID,
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct EditorDeleteColor {
    pub struct_size: size_t,
    pub owner: GUID,
    pub string_number: intptr_t,
    pub start_pos: intptr_t,
}

pub const EDITOR_COLOR_NORMAL_PRIORITY: u32 = 0x80000000;

#[repr(C)] #[derive(Clone, Copy)]
pub struct EditorSaveFile {
    pub  struct_size: size_t,
    pub file_name: *const wchar_t,
    pub file_eol: *const wchar_t,
    pub code_page: uintptr_t,
}

#[repr(C)] #[derive(Clone, Copy)]
pub enum EDITOR_CHANGETYPE {
    ECTYPE_CHANGED = 0,
    ECTYPE_ADDED   = 1,
    ECTYPE_DELETED = 2,
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct EditorChange {
    pub struct_size: size_t,
    pub change_type: EDITOR_CHANGETYPE,
    pub string_number: intptr_t,
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct EditorSubscribeChangeEvent {
    pub struct_size: size_t,
    pub plugin_id: GUID,
}

bitflags! {
    pub struct INPUTBOXFLAGS: c_ulonglong {
        const FIB_ENABLEEMPTY      = 0x0000000000000001;
        const FIB_PASSWORD         = 0x0000000000000002;
        const FIB_EXPANDENV        = 0x0000000000000004;
        const FIB_NOUSELASTHISTORY = 0x0000000000000008;
        const FIB_BUTTONS          = 0x0000000000000010;
        const FIB_NOAMPERSAND      = 0x0000000000000020;
        const FIB_EDITPATH         = 0x0000000000000040;
        const FIB_EDITPATHEXEC     = 0x0000000000000080;
        const FIB_NONE             = 0;
    }
}

pub type FARAPIINPUTBOX = extern "system" fn(
    plugin_id: *const GUID,
    id: *const GUID,
    title: *const wchar_t,
    sub_title: *const wchar_t,
    history_name: *const wchar_t,
    src_text: *const wchar_t,
    dest_text: *const wchar_t,
    dest_size: size_t,
    help_topic: *const wchar_t,
    flags: INPUTBOXFLAGS
) -> intptr_t;

#[repr(C)] #[derive(Clone, Copy)]
pub enum FAR_PLUGINS_CONTROL_COMMANDS {
    PCTL_LOADPLUGIN           = 0,
    PCTL_UNLOADPLUGIN         = 1,
    PCTL_FORCEDLOADPLUGIN     = 2,
    PCTL_FINDPLUGIN           = 3,
    PCTL_GETPLUGININFORMATION = 4,
    PCTL_GETPLUGINS           = 5,
}

#[repr(C)] #[derive(Clone, Copy)]
pub enum FAR_PLUGIN_LOAD_TYPE {
    PLT_PATH = 0,
}

#[repr(C)] #[derive(Clone, Copy)]
pub enum FAR_PLUGIN_FIND_TYPE {
    PFMGUID       = 0,
    PFM_MODULENAME = 1,
}

bitflags! {
    pub struct FAR_PLUGIN_FLAGS: c_ulonglong {
        const FPF_LOADED         = 0x0000000000000001;
        const FPF_ANSI           = 0x1000000000000000;
        const FPF_NONE           = 0;
    }
}

#[repr(C)] #[derive(Clone, Copy)]
pub enum FAR_FILE_FILTER_CONTROL_COMMANDS {
    FFCTL_CREATEFILEFILTER          = 0,
    FFCTL_FREEFILEFILTER            = 1,
    FFCTL_OPENFILTERSMENU           = 2,
    FFCTL_STARTINGTOFILTER          = 3,
    FFCTL_ISFILEINFILTER            = 4,
}

#[repr(C)] #[derive(Clone, Copy)]
pub enum FAR_FILE_FILTER_TYPE {
    FFT_PANEL                       = 0,
    FFT_FINDFILE                    = 1,
    FFT_COPY                        = 2,
    FFT_SELECT                      = 3,
    FFT_CUSTOM                      = 4,
}

#[repr(C)] #[derive(Clone, Copy)]
pub enum FAR_REGEXP_CONTROL_COMMANDS {
    RECTL_CREATE                    = 0,
    RECTL_FREE                      = 1,
    RECTL_COMPILE                   = 2,
    RECTL_OPTIMIZE                  = 3,
    RECTL_MATCHEX                   = 4,
    RECTL_SEARCHEX                  = 5,
    RECTL_BRACKETSCOUNT             = 6,
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct RegExpMatch {
    pub start: intptr_t,
    pub end: intptr_t,
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct RegExpSearch {
    pub text: *const wchar_t,
    pub position: intptr_t,
    pub length: intptr_t,
    pub regexp_match: *mut RegExpMatch,
    pub count: intptr_t,
    pub reserved: *mut c_void,
}

#[repr(C)] #[derive(Clone, Copy)]
pub enum FAR_SETTINGS_CONTROL_COMMANDS {
    SCTL_CREATE                     = 0,
    SCTL_FREE                       = 1,
    SCTL_SET                        = 2,
    SCTL_GET                        = 3,
    SCTL_ENUM                       = 4,
    SCTL_DELETE                     = 5,
    SCTL_CREATESUBKEY               = 6,
    SCTL_OPENSUBKEY                 = 7,
}

#[repr(C)] #[derive(Clone, Copy)]
pub enum FARSETTINGSTYPES {
    FST_UNKNOWN                     = 0,
    FST_SUBKEY                      = 1,
    FST_QWORD                       = 2,
    FST_STRING                      = 3,
    FST_DATA                        = 4,
}

#[repr(C)] #[derive(Clone, Copy)]
pub enum FARSETTINGS_SUBFOLDERS {
    FSSF_ROOT                       =  0,
    FSSF_HISTORY_CMD                =  1,
    FSSF_HISTORY_FOLDER             =  2,
    FSSF_HISTORY_VIEW               =  3,
    FSSF_HISTORY_EDIT               =  4,
    FSSF_HISTORY_EXTERNAL           =  5,
    FSSF_FOLDERSHORTCUT_0           =  6,
    FSSF_FOLDERSHORTCUT_1           =  7,
    FSSF_FOLDERSHORTCUT_2           =  8,
    FSSF_FOLDERSHORTCUT_3           =  9,
    FSSF_FOLDERSHORTCUT_4           = 10,
    FSSF_FOLDERSHORTCUT_5           = 11,
    FSSF_FOLDERSHORTCUT_6           = 12,
    FSSF_FOLDERSHORTCUT_7           = 13,
    FSSF_FOLDERSHORTCUT_8           = 14,
    FSSF_FOLDERSHORTCUT_9           = 15,
    FSSF_CONFIRMATIONS              = 16,
    FSSF_SYSTEM                     = 17,
    FSSF_PANEL                      = 18,
    FSSF_EDITOR                     = 19,
    FSSF_SCREEN                     = 20,
    FSSF_DIALOG                     = 21,
    FSSF_INTERFACE                  = 22,
    FSSF_PANELLAYOUT                = 23,
}

#[repr(C)] #[derive(Clone, Copy)]
pub enum FAR_PLUGIN_SETTINGS_LOCATION {
    PSL_ROAMING = 0,
    PSL_LOCAL   = 1,
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct FarSettingsCreate {
    pub struct_size: size_t,
    pub guid: GUID,
    pub handle: HANDLE,
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct FarSettingsItem_Data {
    pub size: size_t,
    pub data: *const c_void,
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct FarSettingsItem {
    pub struct_size: size_t,
    pub root: size_t,
    pub name: *const wchar_t,
    pub settings_type: FARSETTINGSTYPES,
    pub value: FarSettingsValue,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union FarSettingsItemValue {
    pub number: c_ulonglong,
    pub string: *const wchar_t,
    pub data: FarSettingsItemData,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FarSettingsItemData {
    pub size: size_t,
    pub data: *const c_void,
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct FarSettingsName {
    pub name: *const wchar_t,
    pub settings_type: FARSETTINGSTYPES,
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct FarSettingsHistory {
    pub name: *const wchar_t,
    pub param: *const wchar_t,
    pub plugin_id: GUID,
    pub file: *const wchar_t,
    pub time: FILETIME,
    pub lock: BOOL,
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct FarSettingsEnum {
    pub struct_size: size_t,
    pub root: size_t,
    pub count: size_t,
    pub value: FarSettingsEnumValue,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union FarSettingsEnumValue {
    pub items: *const FarSettingsName,
    pub histories: *const FarSettingsHistory
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct FarSettingsValue {
    pub struct_size: size_t,
    pub root: size_t,
    pub value: *const wchar_t,
}

pub type FARAPIPANELCONTROL = extern "system" fn(
    h_panel: HANDLE,
    command: FILE_CONTROL_COMMANDS,
    param1: intptr_t,
    param2: *mut c_void
) -> intptr_t;

pub type FARAPIADVCONTROL = extern "system" fn(
    plugin_id: *const GUID,
    command: ADVANCED_CONTROL_COMMANDS,
    param1: intptr_t,
    param2: *mut c_void
) -> intptr_t;

pub type FARAPIVIEWERCONTROL = extern "system" fn(
    viewer_id: intptr_t,
    command: VIEWER_CONTROL_COMMANDS,
    param1: intptr_t,
    param2: *mut c_void
) -> intptr_t;

pub type FARAPIEDITORCONTROL = extern "system" fn(
    editor_id: intptr_t,
    command: EDITOR_CONTROL_COMMANDS,
    param1: intptr_t,
    param2: *mut c_void
) -> intptr_t;

pub type FARAPIMACROCONTROL = extern "system" fn(
    plugin_id: *const GUID,
    command: FAR_MACRO_CONTROL_COMMANDS,
    param1: intptr_t,
    param2: *mut c_void
) -> intptr_t;

pub type FARAPIPLUGINSCONTROL = extern "system" fn(
    h_handle: HANDLE,
    command: FAR_PLUGINS_CONTROL_COMMANDS,
    param1: intptr_t,
    param2: *mut c_void
) -> intptr_t;

pub type FARAPIFILEFILTERCONTROL = extern "system" fn(
    h_handle: HANDLE,
    command: FAR_FILE_FILTER_CONTROL_COMMANDS,
    param1: intptr_t,
    param2: *mut c_void
) -> intptr_t;

pub type FARAPIREGEXPCONTROL = extern "system" fn(
    h_handle: HANDLE,
    command: FAR_REGEXP_CONTROL_COMMANDS,
    param1: intptr_t,
    param2: *mut c_void
) -> intptr_t;

pub type FARAPISETTINGSCONTROL = extern "system" fn(
    h_handle: HANDLE,
    command: FAR_SETTINGS_CONTROL_COMMANDS,
    param1: intptr_t,
    param2: *mut c_void
) -> intptr_t;

#[repr(C)] #[derive(Clone, Copy)]
pub enum FARCLIPBOARD_TYPE {
    FCT_ANY = 0,
    FCT_STREAM = 1,
    FCT_COLUMN = 2
}

pub type FARCMPFUNC = extern "system" fn(param1: *const c_void, param2: *const c_void, userparam: *const c_void) -> c_int;
pub type FARSTDQSORT = extern "system" fn(base: *const c_void, nelem: size_t, width: size_t, fcmp: FARCMPFUNC, userparam: *const c_void);
pub type FARSTDBSEARCH = extern "system" fn(key: *const c_void, base: *const c_void, nelem: size_t, width: size_t, fcmp: FARCMPFUNC, userparam: *const c_void) -> *const c_void;
pub type FARSTDGETFILEOWNER = extern "system" fn(computer: *const wchar_t, name: *const wchar_t, owner: *const wchar_t, size: size_t) -> size_t;
pub type FARSTDGETNUMBEROFLINKS = extern "system" fn(name: *const wchar_t) -> size_t;
pub type FARSTDATOI = extern "system" fn(s: *const wchar_t) -> c_int;
pub type FARSTDATOI64 = extern "system" fn(s: *const wchar_t) -> c_longlong;
pub type FARSTDITOA64 = extern "system" fn(value: c_longlong, str: *mut wchar_t, radix: c_int) -> wchar_t;
pub type FARSTDITOA = extern "system" fn(value: c_int, str: *mut wchar_t, radix: c_int) -> wchar_t;
pub type FARSTDLTRIM = extern "system" fn(str: *mut wchar_t) -> wchar_t;
pub type FARSTDRTRIM = extern "system" fn(str: *mut wchar_t) -> wchar_t;
pub type FARSTDTRIM = extern "system" fn(str: *mut wchar_t) -> wchar_t;
pub type FARSTDTRUNCSTR = extern "system" fn(str: *mut wchar_t, max_length: intptr_t) -> wchar_t;
pub type FARSTDTRUNCPATHSTR = extern "system" fn(str: *mut wchar_t, max_length: intptr_t) -> wchar_t;
pub type FARSTDQUOTESPACEONLY = extern "system" fn(str: *mut wchar_t) -> wchar_t;
pub type FARSTDPOINTTONAME = extern "system" fn(path: *const wchar_t) -> *const wchar_t;
pub type FARSTDADDENDSLASH = extern "system" fn(path: *mut wchar_t) -> BOOL;
pub type FARSTDCOPYTOCLIPBOARD = extern "system" fn(clipboard_type: FARCLIPBOARD_TYPE, data: *const wchar_t) -> BOOL;
pub type FARSTDPASTEFROMCLIPBOARD = extern "system" fn(clipboard_type: FARCLIPBOARD_TYPE, data: *mut wchar_t, size: size_t) -> size_t;
pub type FARSTDLOCALISLOWER = extern "system" fn(ch: wchar_t) -> c_int;
pub type FARSTDLOCALISUPPER = extern "system" fn(ch: wchar_t) -> c_int;
pub type FARSTDLOCALISALPHA = extern "system" fn(ch: wchar_t) -> c_int;
pub type FARSTDLOCALISALPHANUM = extern "system" fn(ch: wchar_t) -> c_int;
pub type FARSTDLOCALUPPER = extern "system" fn(lower_char: wchar_t) -> wchar_t;
pub type FARSTDLOCALLOWER = extern "system" fn(upper_char: wchar_t) -> wchar_t;
pub type FARSTDLOCALUPPERBUF = extern "system" fn(buf: *mut wchar_t, length: intptr_t);
pub type FARSTDLOCALLOWERBUF = extern "system" fn(buf: *mut wchar_t, length: intptr_t);
pub type FARSTDLOCALSTRUPR = extern "system" fn(s1: *mut wchar_t);
pub type FARSTDLOCALSTRLWR = extern "system" fn(s1: *mut wchar_t);
pub type FARSTDLOCALSTRICMP = extern "system" fn(s1: *const wchar_t, s2: *const wchar_t) -> c_int; // Deprecated, don't use
pub type FARSTDLOCALSTRNICMP = extern "system" fn(s1: *const wchar_t, s2: *const wchar_t, n: intptr_t) -> c_int; // Deprecated, don't use
pub type FARSTDFARCLOCK = extern "system" fn() -> c_ulonglong;
pub type FARSTDCOMPARESTRINGS = extern "system" fn(str1: *const wchar_t, size1: size_t, str2: *const wchar_t, size2: size_t);

bitflags! {
    pub struct PROCESSNAME_FLAGS: c_ulonglong {
        //             0xFFFF - length
        //           0xFF0000 - mode
        // 0xFFFFFFFFFF000000 - flags
        const PN_CMPNAME          = 0x0000000000000000;
        const PN_CMPNAMELIST      = 0x0000000000010000;
        const PN_GENERATENAME     = 0x0000000000020000;
        const PN_CHECKMASK        = 0x0000000000030000;

        const PN_SKIPPATH         = 0x0000000001000000;
        const PN_SHOWERRORMESSAGE = 0x0000000002000000;
        const PN_NONE             = 0;
    }
}

pub type FARSTDPROCESSNAME = extern "system" fn(param1: *const wchar_t, param2: *const wchar_t, size: size_t, flags: PROCESSNAME_FLAGS) -> size_t;

pub type FARSTDUNQUOTE = extern "system" fn(Str: *mut wchar_t);

bitflags! {
    pub struct XLAT_FLAGS: c_ulonglong {
        const XLAT_SWITCHKEYBLAYOUT  = 0x0000000000000001;
        const XLAT_SWITCHKEYBBEEP    = 0x0000000000000002;
        const XLAT_USEKEYBLAYOUTNAME = 0x0000000000000004;
        const XLAT_CONVERTALLCMDLINE = 0x0000000000010000;
        const XLAT_NONE              = 0;
    }
}

pub type FARSTDINPUTRECORDTOKEYNAME = extern "system" fn(key: *const INPUT_RECORD, key_text: *mut wchar_t, size: size_t) -> size_t;

pub type FARSTDXLAT = extern "system" fn(line: *mut wchar_t, start_pos: intptr_t, end_pos: intptr_t, flags: XLAT_FLAGS) -> *const wchar_t;

pub type FARSTDKEYNAMETOINPUTRECORD = extern "system" fn(name: *const wchar_t, key: *mut INPUT_RECORD) -> BOOL;

pub type FRSUSERFUNC = extern "system" fn(
    f_data: *const PluginPanelItem,
    full_name: *const wchar_t,
    param: *const c_void
) -> c_int;

bitflags! {
    pub struct FRSMODE: c_ulonglong {
        const FRS_RETUPDIR             = 0x0000000000000001;
        const FRS_RECUR                = 0x0000000000000002;
        const FRS_SCANSYMLINK          = 0x0000000000000004;
        const FRS_NONE                 = 0;
    }
}

pub type FARSTDRECURSIVESEARCH = extern "system" fn(init_dir: *const wchar_t, mask: *const wchar_t, func: FRSUSERFUNC,
    flags: FRSMODE, param: *mut c_void);
pub type FARSTDMKTEMP = extern "system" fn(dest: *const wchar_t, dest_size: size_t, prefix: *const wchar_t) -> size_t;
pub type FARSTDGETPATHROOT = extern "system" fn(path: *const wchar_t, root: *const wchar_t, dest_size: size_t) -> size_t;

#[repr(C)] #[derive(Clone, Copy)]
pub enum LINK_TYPE {
    LINK_HARDLINK         = 1,
    LINK_JUNCTION         = 2,
    LINK_VOLMOUNT         = 3,
    LINK_SYMLINKFILE      = 4,
    LINK_SYMLINKDIR       = 5,
    LINK_SYMLINK          = 6,
}

bitflags! {
    pub struct MKLINK_FLAGS: c_ulonglong {
        const MLF_SHOWERRMSG       = 0x0000000000010000;
        const MLF_DONOTUPDATEPANEL = 0x0000000000020000;
        const MLF_HOLDTARGET       = 0x0000000000040000;
        const MLF_NONE             = 0;
    }
}

pub type FARSTDMKLINK = extern "system" fn(src: *const wchar_t, dest: *mut wchar_t, link_type: LINK_TYPE, flags: MKLINK_FLAGS) -> BOOL;
pub type FARGETREPARSEPOINTINFO = extern "system" fn(src: *const wchar_t, dest: *mut wchar_t, dest_size: size_t) -> size_t;

#[repr(C)] #[derive(Clone, Copy)]
pub enum CONVERTPATHMODES {
    CPM_FULL                        = 0,
    CPM_REAL                        = 1,
    CPM_NATIVE                      = 2,
}

pub type FARCONVERTPATH = extern "system" fn(mode: CONVERTPATHMODES, src: *const wchar_t, dest: *mut wchar_t, dest_size: size_t) -> size_t;

pub type FARGETCURRENTDIRECTORY = extern "system" fn(size: size_t, buffer: *mut wchar_t) -> size_t;

bitflags! {
    pub struct FARFORMATFILESIZEFLAGS: c_ulonglong {
        const FFFS_COMMAS                 = 0x0100000000000000;
        const FFFS_FLOATSIZE              = 0x0200000000000000;
        const FFFS_SHOWBYTESINDEX         = 0x0400000000000000;
        const FFFS_ECONOMIC               = 0x0800000000000000;
        const FFFS_THOUSAND               = 0x1000000000000000;
        const FFFS_MINSIZEINDEX           = 0x2000000000000000;
        const FFFS_MINSIZEINDEX_MASK      = 0x0000000000000003;
        const FFFS_NONE                   = 0;
    }
}

pub type FARFORMATFILESIZE = extern "system" fn(size: c_ulonglong, width: intptr_t, flags: FARFORMATFILESIZEFLAGS, dest: *mut wchar_t, dest_size: size_t) -> size_t;

#[repr(C)] #[derive(Clone, Copy)]
pub struct FarStandardFunctions {
    pub struct_size: size_t,

    pub atoi: FARSTDATOI,
    pub atoi64: FARSTDATOI64,
    pub itoa: FARSTDITOA,
    pub itoa64: FARSTDITOA64,
    pub qsort: FARSTDQSORT,
    pub bsearch: FARSTDBSEARCH,

    pub l_is_lower: FARSTDLOCALISLOWER,
    pub l_is_upper: FARSTDLOCALISUPPER,
    pub l_is_alpha: FARSTDLOCALISALPHA,
    pub l_is_alphanum: FARSTDLOCALISALPHANUM,
    pub l_upper: FARSTDLOCALUPPER,
    pub l_lower: FARSTDLOCALLOWER,
    pub l_upper_buf: FARSTDLOCALUPPERBUF,
    pub l_lower_buf: FARSTDLOCALLOWERBUF,
    pub l_strupr: FARSTDLOCALSTRUPR,
    pub l_strlwr: FARSTDLOCALSTRLWR,
    pub l_stricmp: FARSTDLOCALSTRICMP, // Deprecated, don't use
    pub l_strnicmp: FARSTDLOCALSTRNICMP, // Deprecated, don't use

    pub unquote: FARSTDUNQUOTE,
    pub l_trim: FARSTDLTRIM,
    pub r_trim: FARSTDRTRIM,
    pub trim: FARSTDTRIM,
    pub trunc_str: FARSTDTRUNCSTR,
    pub trunc_path_str: FARSTDTRUNCPATHSTR,
    pub quote_space_only: FARSTDQUOTESPACEONLY,
    pub point_to_name: FARSTDPOINTTONAME,
    pub get_path_root: FARSTDGETPATHROOT,
    pub add_end_slash: FARSTDADDENDSLASH,
    pub copy_to_clipboard: FARSTDCOPYTOCLIPBOARD,
    pub paste_from_clipboard: FARSTDPASTEFROMCLIPBOARD,
    pub far_input_record_to_name: FARSTDINPUTRECORDTOKEYNAME,
    pub far_name_to_input_record: FARSTDKEYNAMETOINPUTRECORD,
    pub x_lat: FARSTDXLAT,
    pub get_file_owner: FARSTDGETFILEOWNER,
    pub get_number_of_links: FARSTDGETNUMBEROFLINKS,
    pub far_recursive_search: FARSTDRECURSIVESEARCH,
    pub mk_temp: FARSTDMKTEMP,
    pub process_name: FARSTDPROCESSNAME,
    pub mk_link: FARSTDMKLINK,
    pub convert_path: FARCONVERTPATH,
    pub get_reparse_point_info: FARGETREPARSEPOINTINFO,
    pub get_current_directory: FARGETCURRENTDIRECTORY,
    pub format_file_size: FARFORMATFILESIZE,
    pub far_clock: FARSTDFARCLOCK,
    pub compare_strings: FARSTDCOMPARESTRINGS,
}

#[repr(C)] #[derive(Copy)]
pub struct PluginStartupInfo {
    pub struct_size: size_t,
    pub module_name: *const wchar_t,
    pub menu: FARAPIMENU,
    pub message: FARAPIMESSAGE,
    pub get_msg: FARAPIGETMSG,
    pub panel_control: FARAPIPANELCONTROL,
    pub save_screen: FARAPISAVESCREEN,
    pub restore_screen: FARAPIRESTORESCREEN,
    pub get_dir_list: FARAPIGETDIRLIST,
    pub get_plugin_dir_list: FARAPIGETPLUGINDIRLIST,
    pub free_dir_list: FARAPIFREEDIRLIST,
    pub free_plugin_dir_list: FARAPIFREEPLUGINDIRLIST,
    pub viewer: FARAPIVIEWER,
    pub editor: FARAPIEDITOR,
    pub text: FARAPITEXT,
    pub editor_control: FARAPIEDITORCONTROL,

    pub far_standard_functions: *mut FarStandardFunctions,

    pub show_help: FARAPISHOWHELP,
    pub adv_control: FARAPIADVCONTROL,
    pub input_box: FARAPIINPUTBOX,
    pub color_dialog: FARAPICOLORDIALOG,
    pub dialog_init: FARAPIDIALOGINIT,
    pub dialog_run: FARAPIDIALOGRUN,
    pub dialog_free: FARAPIDIALOGFREE,

    pub send_dlg_message: FARAPISENDDLGMESSAGE,
    pub def_dlg_proc: FARAPIDEFDLGPROC,
    pub viewer_control: FARAPIVIEWERCONTROL,
    pub plugins_control: FARAPIPLUGINSCONTROL,
    pub file_filter_control: FARAPIFILEFILTERCONTROL,
    pub reg_exp_control: FARAPIREGEXPCONTROL,
    pub macro_control: FARAPIMACROCONTROL,
    pub settings_control: FARAPISETTINGSCONTROL,
    pub private: *mut c_void,
    pub instance: *const c_void,
}

impl Clone for PluginStartupInfo {

    #[inline]
    fn clone(&self) -> PluginStartupInfo {
        PluginStartupInfo {struct_size: self.struct_size, module_name: self.module_name, menu: self.menu, message: self.message, get_msg: self.get_msg, panel_control: self.panel_control, save_screen: self.save_screen, restore_screen: self.restore_screen, get_dir_list: self.get_dir_list, get_plugin_dir_list: self.get_plugin_dir_list, free_dir_list: self.free_dir_list, free_plugin_dir_list: self.free_plugin_dir_list, viewer: self.viewer, editor: self.editor, text: self.text, editor_control: self.editor_control, far_standard_functions: self.far_standard_functions, show_help: self.show_help, adv_control: self.adv_control, input_box: self.input_box, color_dialog: self.color_dialog, dialog_init: self.dialog_init, dialog_run: self.dialog_run, dialog_free: self.dialog_free, send_dlg_message: self.send_dlg_message, def_dlg_proc: self.def_dlg_proc, viewer_control: self.viewer_control, plugins_control: self.plugins_control, file_filter_control: self.file_filter_control, reg_exp_control: self.reg_exp_control, macro_control: self.macro_control, settings_control: self.settings_control, private: self.private, instance: self.instance}
    }
}

impl PluginStartupInfo {

    pub fn input_box(&self, plugin_id: *const GUID, id: *const GUID, title: *const wchar_t,
            sub_title: *const wchar_t, history_name: *const wchar_t, src_text: *const wchar_t,
            dest_text: *const wchar_t, dest_size: size_t, help_topic: *const wchar_t,
            flags: INPUTBOXFLAGS) -> intptr_t {
        (self.input_box)(plugin_id, id, title, sub_title, history_name, src_text,
            dest_text, dest_size, help_topic, flags)
    }

    pub fn menu(&self, plugin_id: *const GUID,
                    id: *const GUID,
                    x: intptr_t,
                    y: intptr_t,
                    max_height: intptr_t,
                    flags: FARMENUFLAGS,
                    title: *const wchar_t,
                    bottom: *const wchar_t,
                    help_topic: *const wchar_t,
                    break_keys: *const FarKey,
                    break_code: *const intptr_t,
                    item: *const FarMenuItem,
                    items_number: size_t) -> intptr_t {
        (self.menu)(plugin_id, id, x, y, max_height, flags, title, bottom, help_topic,
                    break_keys, break_code, item, items_number)
    }

    pub fn message(&self, plugin_id: *const GUID, id: *const GUID, flags: FARMESSAGEFLAGS, help_topic: *const wchar_t, items: *const *const wchar_t, items_number: size_t, buttons_number: intptr_t) -> intptr_t {
        (self.message)(plugin_id, id, flags, help_topic, items, items_number, buttons_number)
    }

    pub fn get_msg(&self, plugin_id: *const GUID, msg_id: intptr_t) -> *const wchar_t {
        (self.get_msg)(plugin_id, msg_id)
    }

    pub fn viewer(&self, file_name: *const wchar_t, title: *const wchar_t, x1: intptr_t, y1: intptr_t, x2: intptr_t, y2: intptr_t, flags: VIEWER_FLAGS, code_page: uintptr_t) -> intptr_t {
        (self.viewer)(file_name, title, x1, y1, x2, y2, flags, code_page)
    }

    pub fn editor(&self, file_name: *const wchar_t, title: *const wchar_t, x1: intptr_t, y1: intptr_t, x2: intptr_t, y2: intptr_t, flags: EDITOR_FLAGS, start_line: intptr_t, start_char: intptr_t, code_page: uintptr_t) -> intptr_t {
        (self.editor)(file_name, title, x1, y1, x2, y2, flags, start_line, start_char, code_page)
    }

    pub fn show_help(&self, module_name: *const wchar_t, topic: *const wchar_t, flags: FARHELPFLAGS) -> BOOL {
        (self.show_help)(module_name, topic, flags)
    }

    pub fn color_dialog(&self, plugin_id: *const GUID, flags: COLORDIALOGFLAGS, color: *mut FarColor) -> BOOL {
        (self.color_dialog)(plugin_id, flags, color)
    }

    pub fn dialog_init(&self, plugin_id: *const GUID, id: *const GUID, x1: intptr_t, y1: intptr_t, x2: intptr_t, y2: intptr_t,
            help_topic: *const wchar_t, item: *const FarDialogItem, items_number: size_t, reserved: intptr_t,
            flags: FARDIALOGFLAGS, dlg_proc: FARWINDOWPROC, param: *mut c_void) -> HANDLE {
        (self.dialog_init)(plugin_id, id, x1, y1, x2, y2, help_topic, item, items_number, reserved, flags, dlg_proc, param)
    }

    pub fn dialog_run(&self, h_dlg: HANDLE) -> intptr_t {
        (self.dialog_run)(h_dlg)
    }

    pub fn dialog_free(&self, h_dlg: HANDLE) -> intptr_t {
        (self.dialog_free)(h_dlg)
    }

    pub fn send_dlg_message(&self, h_dlg: HANDLE, msg: intptr_t, param1: intptr_t, param2: *mut c_void) -> intptr_t {
        (self.send_dlg_message)(h_dlg, msg, param1, param2)
    }

    pub fn def_dlg_proc(&self, h_dlg: HANDLE, msg: intptr_t, param1: intptr_t, param2: *mut c_void) -> intptr_t {
        (self.def_dlg_proc)(h_dlg, msg, param1, param2)
    }

    pub fn panel_control(&self, h_panel: HANDLE, command: FILE_CONTROL_COMMANDS, param1: intptr_t, param2: *mut c_void) -> intptr_t {
        (self.panel_control)(h_panel, command, param1, param2)
    }

    pub fn file_filter_control(&self, h_handle: HANDLE, command: FAR_FILE_FILTER_CONTROL_COMMANDS, param1: intptr_t, param2: *mut c_void) -> intptr_t {
        (self.file_filter_control)(h_handle, command, param1, param2)
    }

    pub fn free_dir_list(&self, p_panel_items: *mut PluginPanelItem, n_items_number: size_t) {
        (self.free_dir_list)(p_panel_items, n_items_number)
    }

    pub fn free_plugin_dir_list(&self, h_panel: HANDLE, p_panel_items: *mut PluginPanelItem, n_items_number: size_t) {
        (self.free_plugin_dir_list)(h_panel, p_panel_items, n_items_number)
    }

    pub fn get_dir_list(&self, dir: *const wchar_t, p_panel_items: *mut *mut PluginPanelItem, p_items_number: *mut size_t) -> intptr_t {
        (self.get_dir_list)(dir, p_panel_items, p_items_number)
    }

    pub fn get_plugin_dir_list(&self, plugin_id: *const GUID, h_panel: HANDLE, dir: *const wchar_t, p_panel_items: *mut *mut PluginPanelItem, p_items_number: *mut size_t) -> intptr_t {
        (self.get_plugin_dir_list)(plugin_id, h_panel, dir, p_panel_items, p_items_number)
    }

}

pub type FARAPICREATEFILE = extern "system" fn(object: *const wchar_t, desired_access: DWORD, share_mode: DWORD,
    security_attributes: LPSECURITY_ATTRIBUTES, creation_distribution: DWORD,
    flags_and_attributes: DWORD, template_file: HANDLE) -> HANDLE;
pub type FARAPIGETFILEATTRIBUTES = extern "system" fn(file_name: *const wchar_t) -> DWORD;
pub type FARAPISETFILEATTRIBUTES = extern "system" fn(file_name: *const wchar_t, dw_file_attributes: DWORD) -> BOOL;
pub type FARAPIMOVEFILEEX = extern "system" fn(existing_file_name: *const wchar_t, new_file_name: *const wchar_t,
    dw_flags: DWORD) -> BOOL;
pub type FARAPIDELETEFILE = extern "system" fn(file_name: *const wchar_t) -> BOOL;
pub type FARAPIREMOVEDIRECTORY = extern "system" fn(dir_name: *const wchar_t) -> BOOL;
pub type FARAPICREATEDIRECTORY = extern "system" fn(path_name: *const wchar_t, lp_security_attributes: LPSECURITY_ATTRIBUTES) -> BOOL;

#[repr(C)] #[derive(Clone, Copy)]
pub struct ArclitePrivateInfo {
    pub struct_size: size_t,
    pub create_file: FARAPICREATEFILE,
    pub get_file_attributes: FARAPIGETFILEATTRIBUTES,
    pub set_file_attributes: FARAPISETFILEATTRIBUTES,
    pub move_file_ex: FARAPIMOVEFILEEX,
    pub delete_file: FARAPIDELETEFILE,
    pub remove_directory: FARAPIREMOVEDIRECTORY,
    pub create_directory: FARAPICREATEDIRECTORY,
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct NetBoxPrivateInfo {
    pub struct_size: size_t,
    pub create_file: FARAPICREATEFILE,
    pub get_file_attributes: FARAPIGETFILEATTRIBUTES,
    pub set_file_attributes: FARAPISETFILEATTRIBUTES,
    pub move_file_ex: FARAPIMOVEFILEEX,
    pub delete_file: FARAPIDELETEFILE,
    pub remove_directory: FARAPIREMOVEDIRECTORY,
    pub create_directory: FARAPICREATEDIRECTORY,
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct MacroPluginReturn {
    pub return_type: intptr_t,
    pub count: size_t,
    pub values: *mut FarMacroValue,
}

pub type FARAPICALLFAR = extern "system" fn(check_code: intptr_t, data: *mut FarMacroCall) -> intptr_t;

#[repr(C)] #[derive(Clone, Copy)]
pub struct MacroPrivateInfo {
    pub struct_size: size_t,
    pub call_far: FARAPICALLFAR,
}

bitflags! {
    pub struct PLUGIN_FLAGS: c_ulonglong {
        const PF_PRELOAD        = 0x0000000000000001;
        const PF_DISABLEPANELS  = 0x0000000000000002;
        const PF_EDITOR         = 0x0000000000000004;
        const PF_VIEWER         = 0x0000000000000008;
        const PF_FULLCMDLINE    = 0x0000000000000010;
        const PF_DIALOG         = 0x0000000000000020;
        const PF_NONE           = 0;
    }
}

impl Default for PLUGIN_FLAGS {

    fn default() -> PLUGIN_FLAGS {
        PLUGIN_FLAGS::PF_NONE
    }
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct PluginMenuItem {
    pub guids: *const GUID,
    pub strings: *const *const wchar_t,
    pub count: size_t,
}

impl Default for PluginMenuItem {

    fn default() -> PluginMenuItem {
        PluginMenuItem {
            guids: ptr::null_mut(),
            strings: ptr::null_mut(),
            count: 0
        }
    }
}

#[repr(C)] #[derive(Clone, Copy)]
pub enum VersionStage {
    VS_RELEASE                      = 0,
    VS_ALPHA                        = 1,
    VS_BETA                         = 2,
    VS_RC                           = 3,
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct VersionInfo {
    pub major: DWORD,
    pub minor: DWORD,
    pub revision: DWORD,
    pub build: DWORD,
    pub stage: VersionStage,
}

impl Default for VersionInfo {

    fn default() -> VersionInfo {
        VersionInfo {
            major: 0,
            minor: 0,
            revision: 0,
            build: 0,
            stage: VersionStage::VS_ALPHA
        }
    }
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct GlobalInfo {
    pub struct_size: size_t,
    pub min_far_version: VersionInfo,
    pub version: VersionInfo,
    pub guid: GUID,
    pub title: *const wchar_t,
    pub description: *const wchar_t,
    pub author: *const wchar_t,
    pub instance: *const c_void,
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct PluginInfo {
    pub struct_size: size_t,
    pub flags: PLUGIN_FLAGS,
    pub disk_menu: PluginMenuItem,
    pub plugin_menu: PluginMenuItem,
    pub plugin_config: PluginMenuItem,
    pub command_prefix: *const wchar_t,
    pub instance: *const c_void,
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct FarGetPluginInformation {
    pub struct_size: size_t,
    pub module_name: *const wchar_t,
    pub flags: FAR_PLUGIN_FLAGS,
    pub p_info: *mut PluginInfo,
    pub g_info: *mut GlobalInfo,
}

bitflags! {
    pub struct INFOPANELLINE_FLAGS: c_ulonglong {
        const IPLFLAGS_SEPARATOR      = 0x0000000000000001;
        const IPLFLAGS_NONE           = 0;
    }
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct InfoPanelLine {
    pub text: *const wchar_t,
    pub data: *const wchar_t,
    pub flags: INFOPANELLINE_FLAGS,
}

bitflags! {
    pub struct PANELMODE_FLAGS: c_ulonglong {
        const PMFLAGS_FULLSCREEN      = 0x0000000000000001;
        const PMFLAGS_DETAILEDSTATUS  = 0x0000000000000002;
        const PMFLAGS_ALIGNEXTENSIONS = 0x0000000000000004;
        const PMFLAGS_CASECONVERSION  = 0x0000000000000008;
        const PMFLAGS_NONE            = 0;
    }
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct PanelMode {
    pub column_types: *const wchar_t,
    pub column_widths: *const wchar_t,
    pub column_titles: *const *const wchar_t,
    pub status_column_types: *const wchar_t,
    pub status_column_widths: *const wchar_t,
    pub flags: PANELMODE_FLAGS,
}

bitflags! {
    pub struct OPENPANELINFO_FLAGS: c_ulonglong {
        const OPIF_DISABLEFILTER           = 0x0000000000000001;
        const OPIF_DISABLESORTGROUPS       = 0x0000000000000002;
        const OPIF_DISABLEHIGHLIGHTING     = 0x0000000000000004;
        const OPIF_ADDDOTS                 = 0x0000000000000008;
        const OPIF_RAWSELECTION            = 0x0000000000000010;
        const OPIF_REALNAMES               = 0x0000000000000020;
        const OPIF_SHOWNAMESONLY           = 0x0000000000000040;
        const OPIF_SHOWRIGHTALIGNNAMES     = 0x0000000000000080;
        const OPIF_SHOWPRESERVECASE        = 0x0000000000000100;
        const OPIF_COMPAREFATTIME          = 0x0000000000000400;
        const OPIF_EXTERNALGET             = 0x0000000000000800;
        const OPIF_EXTERNALPUT             = 0x0000000000001000;
        const OPIF_EXTERNALDELETE          = 0x0000000000002000;
        const OPIF_EXTERNALMKDIR           = 0x0000000000004000;
        const OPIF_USEATTRHIGHLIGHTING     = 0x0000000000008000;
        const OPIF_USECRC32                = 0x0000000000010000;
        const OPIF_USEFREESIZE             = 0x0000000000020000;
        const OPIF_SHORTCUT                = 0x0000000000040000;
        const OPIF_NONE                    = 0;
    }
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct KeyBarLabel {
    pub key: FarKey,
    pub text: *const wchar_t,
    pub long_text: *const wchar_t,
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct KeyBarTitles {
    pub count_labels: size_t,
    pub labels: *mut KeyBarLabel,
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct FarSetKeyBarTitles {
    pub struct_size: size_t,
    pub titles: *mut KeyBarTitles,
}

bitflags! {
    pub struct OPERATION_MODES: c_ulonglong {
        const OPM_SILENT     = 0x0000000000000001;
        const OPM_FIND       = 0x0000000000000002;
        const OPM_VIEW       = 0x0000000000000004;
        const OPM_EDIT       = 0x0000000000000008;
        const OPM_TOPLEVEL   = 0x0000000000000010;
        const OPM_DESCR      = 0x0000000000000020;
        const OPM_QUICKVIEW  = 0x0000000000000040;
        const OPM_PGDN       = 0x0000000000000080;
        const OPM_COMMANDS   = 0x0000000000000100;
        const OPM_NONE       = 0;
    }
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct OpenPanelInfo {
    pub struct_size: size_t,
    pub h_panel: HANDLE,
    pub flags: OPENPANELINFO_FLAGS,
    pub host_file: *const wchar_t,
    pub cur_dir: *const wchar_t,
    pub format: *const wchar_t,
    pub panel_title: *const wchar_t,
    pub info_lines: *const InfoPanelLine,
    pub info_lines_number: size_t,
    pub descr_files: *const *const wchar_t,
    pub descr_files_number: size_t,
    pub panel_modes_array: *const PanelMode,
    pub panel_modes_number: size_t,
    pub start_panel_mode: intptr_t,
    pub start_sort_mode: OPENPANELINFO_SORTMODES,
    pub start_sort_order: intptr_t,
    pub key_bar: *const KeyBarTitles,
    pub shortcut_data: *const wchar_t,
    pub free_size: c_ulonglong,
    pub user_data: UserDataItem,
    pub instance: *const c_void,
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct AnalyseInfo {
    pub struct_size: size_t,
    pub file_name: *const wchar_t,
    pub buffer: *const c_void,
    pub buffer_size: size_t,
    pub op_mode: OPERATION_MODES,
    pub instance: *const c_void,
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct OpenAnalyseInfo {
    pub struct_size: size_t,
    pub info: *const AnalyseInfo,
    pub handle: HANDLE,
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct OpenMacroInfo {
    pub struct_size: size_t,
    pub count: size_t,
    pub values: *mut FarMacroValue,
}

bitflags! {
    pub struct FAROPENSHORTCUTFLAGS: c_ulonglong {
        const FOSF_ACTIVE = 0x0000000000000001;
        const FOSF_NONE   = 0;
    }
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct OpenShortcutInfo {
    pub struct_size: size_t,
    pub host_file: *const wchar_t,
    pub shortcut_data: *const wchar_t,
    pub flags: FAROPENSHORTCUTFLAGS,
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct OpenCommandLineInfo {
    pub struct_size: size_t,
    pub command_line: *const wchar_t,
}

#[repr(C)] #[derive(Clone, Copy)]
pub enum OPENFROM {
    OPEN_LEFTDISKMENU       = 0,
    OPEN_PLUGINSMENU        = 1,
    OPEN_FINDLIST           = 2,
    OPEN_SHORTCUT           = 3,
    OPEN_COMMANDLINE        = 4,
    OPEN_EDITOR             = 5,
    OPEN_VIEWER             = 6,
    OPEN_FILEPANEL          = 7,
    OPEN_DIALOG             = 8,
    OPEN_ANALYSE            = 9,
    OPEN_RIGHTDISKMENU      = 10,
    OPEN_FROMMACRO          = 11,
    OPEN_LUAMACRO           = 100,
}

#[repr(C)] #[derive(Clone, Copy)]
pub enum MACROCALLTYPE {
    MCT_MACROPARSE         = 0,
    MCT_LOADMACROS         = 1,
    MCT_ENUMMACROS         = 2,
    MCT_WRITEMACROS        = 3,
    MCT_GETMACRO           = 4,
    MCT_RECORDEDMACRO      = 5,
    MCT_DELMACRO           = 6,
    MCT_RUNSTARTMACRO      = 7,
    MCT_EXECSTRING         = 8,
    MCT_PANELSORT          = 9,
    MCT_GETCUSTOMSORTMODES = 10,
    MCT_ADDMACRO           = 11,
    MCT_KEYMACRO           = 12,
    MCT_CANPANELSORT       = 13,
}

#[repr(C)] #[derive(Clone, Copy)]
pub enum MACROPLUGINRETURNTYPE {
    MPRT_NORMALFINISH  = 0,
    MPRT_ERRORFINISH   = 1,
    MPRT_ERRORPARSE    = 2,
    MPRT_KEYS          = 3,
    MPRT_PRINT         = 4,
    MPRT_PLUGINCALL    = 5,
    MPRT_PLUGINMENU    = 6,
    MPRT_PLUGINCONFIG  = 7,
    MPRT_PLUGINCOMMAND = 8,
    MPRT_USERMENU      = 9,
    MPRT_HASNOMACRO    = 10,
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct OpenMacroPluginInfo {
    pub call_type: MACROCALLTYPE,
    pub data: *mut FarMacroCall,
    pub ret: MacroPluginReturn,
}

#[repr(C)] #[derive(Clone, Copy)]
pub enum FAR_EVENTS {
    FE_CHANGEVIEWMODE   = 0,
    FE_REDRAW           = 1,
    FE_IDLE             = 2,
    FE_CLOSE            = 3,
    FE_BREAK            = 4,
    FE_COMMAND          = 5,

    FE_GOTFOCUS         = 6,
    FE_KILLFOCUS        = 7,
    FE_CHANGESORTPARAMS = 8,
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct OpenInfo {
    pub struct_size: size_t,
    pub open_from: OPENFROM,
    pub guid: *const GUID,
    pub data: intptr_t,
    pub instance: *const c_void,
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct SetDirectoryInfo {
    pub struct_size: size_t,
    pub h_panel: HANDLE,
    pub dir: *const wchar_t,
    pub reserved: intptr_t,
    pub op_mode: OPERATION_MODES,
    pub user_data: UserDataItem,
    pub instance: *const c_void,
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct SetFindListInfo {
    pub struct_size: size_t,
    pub h_panel: HANDLE,
    pub panel_item: *const PluginPanelItem,
    pub items_number: size_t,
    pub instance: *const c_void,
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct PutFilesInfo {
    pub struct_size: size_t,
    pub h_panel: HANDLE,
    pub panel_item: *mut PluginPanelItem,
    pub items_number: size_t,
    pub move_file: BOOL,
    pub src_path: *const wchar_t,
    pub op_mode: OPERATION_MODES,
    pub instance: *const c_void,
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct ProcessHostFileInfo {
    pub struct_size: size_t,
    pub h_panel: HANDLE,
    pub panel_item: *mut PluginPanelItem,
    pub items_number: size_t,
    pub op_mode: OPERATION_MODES,
    pub instance: *const c_void,
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct MakeDirectoryInfo {
    pub struct_size: size_t,
    pub h_panel: HANDLE,
    pub name: *const wchar_t,
    pub op_mode: OPERATION_MODES,
    pub instance: *const c_void,
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct CompareInfo {
    pub struct_size: size_t,
    pub h_panel: HANDLE,
    pub item1: *const PluginPanelItem,
    pub item2: *const PluginPanelItem,
    pub mode: OPENPANELINFO_SORTMODES,
    pub instance: *const c_void,
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct GetFindDataInfo {
    pub struct_size: size_t,
    pub h_panel: HANDLE,
    pub panel_item: *const PluginPanelItem,
    pub items_number: size_t,
    pub op_mode: OPERATION_MODES,
    pub instance: *const c_void,
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct FreeFindDataInfo {
    pub struct_size: size_t,
    pub h_panel: HANDLE,
    pub panel_item: *mut PluginPanelItem,
    pub items_number: size_t,
    pub instance: *const c_void,
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct GetFilesInfo {
    pub struct_size: size_t,
    pub h_panel: HANDLE,
    pub panel_item: *mut PluginPanelItem,
    pub items_number: size_t,
    pub move_file: BOOL,
    pub dest_path: *const wchar_t,
    pub op_mode: OPERATION_MODES,
    pub instance: *const c_void,
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct DeleteFilesInfo {
    pub struct_size: size_t,
    pub h_panel: HANDLE,
    pub panel_item: *mut PluginPanelItem,
    pub items_number: size_t,
    pub op_mode: OPERATION_MODES,
    pub instance: *const c_void,
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct ProcessPanelInputInfo {
    pub struct_size: size_t,
    pub h_panel: HANDLE,
    pub rec: INPUT_RECORD,
    pub instance: *const c_void,
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct ProcessEditorInputInfo {
    pub struct_size: size_t,
    pub rec: INPUT_RECORD,
    pub instance: *const c_void,
}

bitflags! {
    pub struct PROCESSCONSOLEINPUT_FLAGS: c_ulonglong {
        const PCIF_NONE     = 0;
    }
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct ProcessConsoleInputInfo {
    pub struct_size: size_t,
    pub flags: PROCESSCONSOLEINPUT_FLAGS,
    pub rec: INPUT_RECORD,
    pub instance: *const c_void,
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct ExitInfo {
    pub struct_size: size_t,
    pub instance: *const c_void,
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct ProcessPanelEventInfo {
    pub struct_size: size_t,
    pub event: FAR_EVENTS,
    pub param: *const c_void,
    pub h_panel: HANDLE,
    pub instance: *const c_void,
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct ProcessEditorEventInfo {
    pub struct_size: size_t,
    pub event: intptr_t,
    pub param: *mut c_void,
    pub editor_id: intptr_t,
    pub instance: *const c_void,
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct ProcessDialogEventInfo {
    pub struct_size: size_t,
    pub event: intptr_t,
    pub param: *mut FarDialogEvent,
    pub instance: *const c_void,
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct ProcessSynchroEventInfo {
    pub struct_size: size_t,
    pub event: intptr_t,
    pub param: *mut c_void,
    pub instance: *const c_void,
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct ProcessViewerEventInfo {
    pub struct_size: size_t,
    pub event: intptr_t,
    pub param: *mut c_void,
    pub viewer_id: intptr_t,
    pub instance: *const c_void,
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct ClosePanelInfo {
    pub struct_size: size_t,
    pub h_panel: HANDLE,
    pub instance: *const c_void,
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct CloseAnalyseInfo {
    pub struct_size: size_t,
    pub handle: HANDLE,
    pub instance: *const c_void,
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct ConfigureInfo {
    pub struct_size: size_t,
    pub guid: *const GUID,
    pub instance: *const c_void,
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct GetContentFieldsInfo
{
    pub struct_size: size_t,
    pub count: size_t,
    pub names: *const *const wchar_t,
    pub instance: *const c_void,
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct GetContentDataInfo
{
    pub struct_size: size_t,
    pub file_path: *const *const wchar_t,
    pub count: size_t,
    pub names: *const *const wchar_t,
    pub values: *const *mut wchar_t,
    pub instance: *const c_void,
}

#[repr(C)] #[derive(Clone, Copy)]
pub struct ErrorInfo
{
    pub struct_size: size_t,
    pub summary: *const wchar_t,
    pub description: *const wchar_t,
}

#[allow(non_upper_case_globals)]
pub const DEFAULT_GUID: GUID = GUID {
    Data1: 0x00000000,
    Data2: 0x0000,
    Data3: 0x0000,
    Data4: [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
};

// Exported Functions
//pub type AnalyseW = extern "system" fn(Info: *const AnalyseInfo) -> HANDLE;
//pub type CloseAnalyseW = extern "system" fn(Info: *const CloseAnalyseInfo);
//pub type ClosePanelW = extern "system" fn(Info: *const ClosePanelInfo);
//pub type CompareW = extern "system" fn(Info: *const CompareInfo) -> intptr_t;
//pub type ConfigureW = extern "system" fn(Info: *const ConfigureInfo) -> intptr_t;
//pub type DeleteFilesW = extern "system" fn(Info: *const DeleteFilesInfo) -> intptr_t;
//pub type ExitFARW = extern "system" fn(Info: *const ExitInfo);
//pub type FreeFindDataW = extern "system" fn(Info: *const FreeFindDataInfo);
//pub type GetFilesW = extern "system" fn(Info: *mut GetFilesInfo) -> intptr_t;
//pub type GetFindDataW = extern "system" fn(Info: *mut GetFindDataInfo) -> intptr_t;
//pub type GetGlobalInfoW = extern "system" fn(Info: *mut GlobalInfo);
//pub type GetOpenPanelInfoW = extern "system" fn(Info: *mut OpenPanelInfo);
//pub type GetPluginInfoW = extern "system" fn(Info: *mut PluginInfo);
//pub type MakeDirectoryW = extern "system" fn(Info: *mut MakeDirectoryInfo) -> intptr_t;
//pub type OpenW = extern "system" fn(Info: *const OpenInfo) -> HANDLE;
//pub type ProcessDialogEventW = extern "system" fn(Info: *const ProcessDialogEventInfo) -> intptr_t;
//pub type ProcessEditorEventW = extern "system" fn(Info: *const ProcessEditorEventInfo) -> intptr_t;
//pub type ProcessEditorInputW = extern "system" fn(Info: *const ProcessEditorInputInfo) -> intptr_t;
//pub type ProcessPanelEventW = extern "system" fn(Info: *const ProcessPanelEventInfo) -> intptr_t;
//pub type ProcessHostFileW = extern "system" fn(Info: *const ProcessHostFileInfo) -> intptr_t;
//pub type ProcessPanelInputW = extern "system" fn(Info: *const ProcessPanelInputInfo) -> intptr_t;
//pub type ProcessConsoleInputW = extern "system" fn(Info: *mut ProcessConsoleInputInfo) -> intptr_t;
//pub type ProcessSynchroEventW = extern "system" fn(Info: *const ProcessSynchroEventInfo) -> intptr_t;
//pub type ProcessViewerEventW = extern "system" fn(Info: *const ProcessViewerEventInfo) -> intptr_t;
//pub type PutFilesW = extern "system" fn(Info: *const PutFilesInfo) -> intptr_t;
//pub type SetDirectoryW = extern "system" fn(Info: *const SetDirectoryInfo) -> intptr_t;
//pub type SetFindListW = extern "system" fn(Info: *const SetFindListInfo) -> intptr_t;
//pub type SetStartupInfoW = extern "system" fn(Info: *const PluginStartupInfo);
//pub type GetContentFieldsW = extern "system" fn(info: *const GetContentFieldsInfo) -> intptr_t;
//pub type GetContentDataW = extern "system" fn(info: *const GetContentDataInfo) -> intptr_t;
//pub type FreeContentDataW = extern "system" fn(info: *const GetContentDataInfo);
