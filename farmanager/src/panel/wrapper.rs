use std::ptr;

use crate::common::ffi::AsInner;
use crate::common::ffi::AsMutInner;
use crate::common::string::WideString;
use crate::ffi;
use crate::common::ffi::Array;
use crate::common::string::WideStringArray;

pub(super) struct PluginPanelItem {
    inner: ffi::PluginPanelItem,
    #[allow(dead_code)]
    file_name: WideString,
    #[allow(dead_code)]
    alternate_file_name: Option<WideString>,
    #[allow(dead_code)]
    description: Option<WideString>,
    #[allow(dead_code)]
    owner: Option<WideString>,
}

impl AsInner<ffi::PluginPanelItem> for PluginPanelItem {
    fn as_inner(&self) -> &ffi::PluginPanelItem {
        &self.inner
    }
}

impl AsMutInner<ffi::PluginPanelItem> for PluginPanelItem {
    fn as_mut_inner(&mut self) -> &mut ffi::PluginPanelItem {
        &mut self.inner
    }
}

impl From<crate::panel::PluginPanelItem> for PluginPanelItem {
    fn from(src: crate::panel::PluginPanelItem) -> Self {
        PluginPanelItem::from(&src)
    }
}

impl From<&crate::panel::PluginPanelItem> for PluginPanelItem {
    fn from(src: &crate::panel::PluginPanelItem) -> Self {
        let file_name = WideString::from(src.file_name.as_str());
        let alternate_file_name = src.alternate_file_name.as_ref().map(|s| WideString::from(s.as_str()));
        let description = src.description.as_ref().map(|s| WideString::from(s.as_str()));
        let owner = src.owner.as_ref().map(|s| WideString::from(s.as_str()));

        let inner: ffi::PluginPanelItem = ffi::PluginPanelItem {
            creation_time: src.creation_time,
            last_access_time: src.last_access_time,
            last_write_time: src.last_write_time,
            change_time: src.change_time,
            file_size: src.file_size,
            allocation_size: src.allocation_size,
            file_name: file_name.as_ptr(),
            alternate_file_name: match &alternate_file_name {
                Some(value) => value.as_ptr(),
                None => ptr::null(),
            },
            description: match &description {
                Some(value) => value.as_ptr(),
                None => ptr::null(),
            },
            owner: match &owner {
                Some(value) => value.as_ptr(),
                None => ptr::null(),
            },
            // TODO implement support
            custom_column_data: ptr::null(),
            // TODO implement support
            custom_column_number: 0,
            flags: ffi::PLUGINPANELITEMFLAGS::PPIF_NONE,
            // TODO implement support
            user_data: ffi::UserDataItem {
                data: ptr::null_mut(),
                free_data: None,
            },
            file_attributes: src.file_attributes.bits(),
            number_of_links: 0,
            crc32: 0,
            reserved: [0; 2],
        };
        PluginPanelItem {
            inner,
            file_name,
            alternate_file_name,
            description,
            owner,
        }
    }
}

pub(super) struct OpenPanelInfo {
    pub(super) inner: ffi::OpenPanelInfo,
    pub(super) host_file: Option<WideString>,
    pub(super) cur_dir: WideString,
    pub(super) format: Option<WideString>,
    pub(super) panel_title: WideString,
    pub(super) info_lines: Array<InfoPanelLine, ffi::InfoPanelLine>,
    pub(super) descr_files: Option<WideStringArray>,
}

#[allow(dead_code)]
pub(crate) struct InfoPanelLine {
    pub(super) inner: ffi::InfoPanelLine,
    pub(super) text: WideString,
    pub(super) data: WideString,
}

impl AsInner<ffi::InfoPanelLine> for InfoPanelLine {
    fn as_inner(&self) -> &ffi::InfoPanelLine {
        &self.inner
    }
}

impl From<&crate::panel::InfoPanelLine> for InfoPanelLine {
    fn from(src: &crate::panel::InfoPanelLine) -> Self {
        let text = WideString::from(src.text.as_str());
        let data = WideString::from(src.data.as_str());
        InfoPanelLine {
            inner: ffi::InfoPanelLine {
                text: text.as_ptr(),
                data: data.as_ptr(),
                flags: src.flags,
            },
            text,
            data,
        }
    }
}
