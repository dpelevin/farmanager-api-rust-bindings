use std::ptr;

use failure::*;
use log::*;

use crate::common::ffi::AsMutInner;
use crate::far_api;
use crate::ffi;

use super::*;

pub enum FileFilterType {
    Panel,
    FindFile,
    Copy,
    Select,
    Custom
}

enum FileFilterControlCommand<'a> {
    CreateFileFilter(Panel, FileFilterType),
    FreeFileFilter(ffi::HANDLE),
    OpenFiltersMenu(ffi::HANDLE),
    StartingToFilter(ffi::HANDLE),
    IsFileInFilter(ffi::HANDLE, &'a PluginPanelItem)
}

enum FileFilterControlResult {
    Handle(ffi::HANDLE),
    ReturnCode(ReturnCode),
    Match(bool)
}

pub struct FileFilter {
    handle: ffi::HANDLE
}

impl From<ffi::HANDLE> for FileFilter {
    fn from(handle: ffi::HANDLE) -> Self {
        FileFilter {
            handle
        }
    }
}

impl Drop for FileFilter {
    fn drop(&mut self) {
        let _ = file_filter_control(FileFilterControlCommand::FreeFileFilter(self.handle));
    }
}

impl FileFilter {
    pub fn starting_to_filter(&self) {
        let _ = file_filter_control(FileFilterControlCommand::StartingToFilter(self.handle));
    }

    pub fn open_filters_menu(&self) {
        let _ = file_filter_control(FileFilterControlCommand::OpenFiltersMenu(self.handle));
    }

    pub fn is_file_in_filter(&self, panel_item: &PluginPanelItem) -> bool {
        let result = file_filter_control(FileFilterControlCommand::IsFileInFilter(self.handle, panel_item));
        match result {
            Ok(result) => match result {
                FileFilterControlResult::Match(is_match) => is_match,
                _ => false
            },
            Err(_) => false,
        }
    }
}

pub fn file_filter(panel: Panel, filter_type: FileFilterType) -> Result<FileFilter> {
    let result = file_filter_control(FileFilterControlCommand::CreateFileFilter(panel, filter_type))?;
    match result {
        FileFilterControlResult::Handle(handle) => Ok(FileFilter::from(handle)),
        _ => unimplemented!()
    }
}

fn file_filter_control(command: FileFilterControlCommand) -> Result<FileFilterControlResult> {
    trace!(">file_filter_control()");
    let result = far_api(|far_api: &mut ffi::PluginStartupInfo| {
        match command {
            FileFilterControlCommand::CreateFileFilter(panel, filter_type) => {
                let handle = panel.into();
                let param1 = match filter_type {
                    FileFilterType::Panel => ffi::FAR_FILE_FILTER_TYPE::FFT_PANEL,
                    FileFilterType::FindFile => ffi::FAR_FILE_FILTER_TYPE::FFT_FINDFILE,
                    FileFilterType::Copy => ffi::FAR_FILE_FILTER_TYPE::FFT_COPY,
                    FileFilterType::Select => ffi::FAR_FILE_FILTER_TYPE::FFT_SELECT,
                    FileFilterType::Custom => ffi::FAR_FILE_FILTER_TYPE::FFT_CUSTOM
                };
                let mut param2: ffi::HANDLE = ptr::null_mut();
                let result = far_api.file_filter_control(handle,
                                                         ffi::FAR_FILE_FILTER_CONTROL_COMMANDS::FFCTL_CREATEFILEFILTER,
                                                         param1 as libc::intptr_t,
                                                         &mut param2 as *mut _ as *mut libc::c_void);
                match result {
                    0 => Err(format_err!("")),
                    _ => Ok(FileFilterControlResult::Handle(param2))
                }
            },
            FileFilterControlCommand::FreeFileFilter(handle) => {
                let _ = far_api.file_filter_control(handle,
                                                    ffi::FAR_FILE_FILTER_CONTROL_COMMANDS::FFCTL_FREEFILEFILTER,
                                                    0,
                                                    ptr::null_mut());
                Ok(FileFilterControlResult::ReturnCode(ReturnCode::Success))
            },
            FileFilterControlCommand::OpenFiltersMenu(handle) => {
                let result = far_api.file_filter_control(handle,
                                                         ffi::FAR_FILE_FILTER_CONTROL_COMMANDS::FFCTL_OPENFILTERSMENU,
                                                         0,
                                                         ptr::null_mut());
                match result {
                    0 => Ok(FileFilterControlResult::ReturnCode(ReturnCode::UserCancel)),
                    _ => Ok(FileFilterControlResult::ReturnCode(ReturnCode::Success))
                }
            },
            FileFilterControlCommand::StartingToFilter(handle) => {
                let _ = far_api.file_filter_control(handle,
                                                    ffi::FAR_FILE_FILTER_CONTROL_COMMANDS::FFCTL_STARTINGTOFILTER,
                                                    0,
                                                    ptr::null_mut());
                Ok(FileFilterControlResult::ReturnCode(ReturnCode::Success))
            },
            FileFilterControlCommand::IsFileInFilter(handle, panel_item) => {
                let mut item: wrp::PluginPanelItem = wrp::PluginPanelItem::from(panel_item);
                let raw_item: &mut ffi::PluginPanelItem = item.as_mut_inner();
                let result = far_api.file_filter_control(handle,
                                                         ffi::FAR_FILE_FILTER_CONTROL_COMMANDS::FFCTL_ISFILEINFILTER,
                                                         0,
                                                         raw_item as *mut _ as *mut libc::c_void);
                match result {
                    0 => Ok(FileFilterControlResult::Match(false)),
                    _ => Ok(FileFilterControlResult::Match(true))
                }
            },
        }
    });
    trace!(">file_filter_control()");
    return result;
}
