# Far Manager Plugins API bindings for Rust

Though bindings are at PoC stage, it is already possible to create Far Manager plugins in Rust.

## Useful links

* [Far Manager website](https://www.farmanager.com/index.php?l=en)
* [Far Manager Plugins API reference](https://api.farmanager.com/ru/index.html)

## Usage

Examples:

* ['HelloWorld' plugin](examples/hellorust)
* ['API Showcase' plugin](examples/showcase)

## Implementation status

### Basic API

Export functions

- [x] [ExitFARW](https://api.farmanager.com/ru/exported_functions/exitfarw.html)
- [x] [GetGlobalInfoW](https://api.farmanager.com/ru/exported_functions/getglobalinfow.html)
- [x] [GetPluginInfoW](https://api.farmanager.com/ru/exported_functions/getplugininfow.html)
- [x] [OpenW](https://api.farmanager.com/ru/exported_functions/openw.html)
- [x] [SetStartupInfoW](https://api.farmanager.com/ru/exported_functions/setstartupinfow.html)

Service functions

- [x] [GetMsg](https://api.farmanager.com/ru/service_functions/getmsg.html)
- [x] [InputBox](https://api.farmanager.com/ru/service_functions/inputbox.html)
- [x] [Menu](https://api.farmanager.com/ru/service_functions/menu.html)
- [x] [Message](https://api.farmanager.com/ru/service_functions/message.html)
- [x] [ShowHelp](https://api.farmanager.com/ru/service_functions/showhelp.html)

### Panel API

Export functions

- [x] [AnalyseW](https://api.farmanager.com/ru/exported_functions/analysew.html)
- [x] [CloseAnalyseW](https://api.farmanager.com/ru/exported_functions/closeanalysew.html)
- [x] [ClosePanelW](https://api.farmanager.com/ru/exported_functions/closepanelw.html)
- [x] [CompareW](https://api.farmanager.com/ru/exported_functions/comparew.html)
- [x] [DeleteFilesW](https://api.farmanager.com/ru/exported_functions/deletefilesw.html)
- [x] [FreeFindDataW](https://api.farmanager.com/ru/exported_functions/freefinddataw.html)
- [x] [GetFilesW](https://api.farmanager.com/ru/exported_functions/getfilesw.html)
- [x] [GetFindDataW](https://api.farmanager.com/ru/exported_functions/getfinddataw.html)
- [x] [GetOpenPanelInfoW](https://api.farmanager.com/ru/exported_functions/getopenpanelinfow.html)
- [x] [MakeDirectoryW](https://api.farmanager.com/ru/exported_functions/makedirectoryw.html)
- [x] [ProcessPanelEventW](https://api.farmanager.com/ru/exported_functions/processpaneleventw.html)
- [x] [ProcessHostFileW](https://api.farmanager.com/ru/exported_functions/processhostfilew.html)
- [x] [ProcessPanelInputW](https://api.farmanager.com/ru/exported_functions/processpanelinputw.html)
- [x] [PutFilesW](https://api.farmanager.com/ru/exported_functions/putfilesw.html)
- [x] [SetDirectoryW](https://api.farmanager.com/ru/exported_functions/setdirectoryw.html)
- [x] [SetFindListW](https://api.farmanager.com/ru/exported_functions/setfindlistw.html)

Service functions

- [x] [PanelControl](https://api.farmanager.com/ru/service_functions/panelcontrol.html)
- [x] [FileFilterControl](https://api.farmanager.com/ru/service_functions/filefiltercontrol.html)
- [x] [FreeDirList](https://api.farmanager.com/ru/service_functions/freedirlist.html)
- [x] [FreePluginDirList](https://api.farmanager.com/ru/service_functions/freeplugindirlist.html)
- [x] [GetDirList](https://api.farmanager.com/ru/service_functions/getdirlist.html)
- [x] [GetPluginDirList](https://api.farmanager.com/ru/service_functions/getplugindirlist.html)

### Editor API

Export functions

- [ ] [ProcessEditorInputW](https://api.farmanager.com/ru/exported_functions/processeditorinputw.html)
- [ ] [ProcessEditorEventW](https://api.farmanager.com/ru/exported_functions/processeditoreventw.html)

Service functions

- [x] [Editor](https://api.farmanager.com/ru/service_functions/editor.html)
- [ ] [EditorControl](https://api.farmanager.com/ru/service_functions/editorcontrol.html)

### Viewer API

Export functions

- [ ] [ProcessViewerEventW](https://api.farmanager.com/ru/exported_functions/processviewereventw.html)

Service functions

- [x] [Viewer](https://api.farmanager.com/ru/service_functions/viewer.html)
- [ ] [ViewerControl](https://api.farmanager.com/ru/service_functions/viewercontrol.html)

### Dialog API

Export functions

- [ ] [ProcessDialogEventW](https://api.farmanager.com/ru/exported_functions/processdialogeventw.html)

Service functions

- [ ] [DefDlgProc](https://api.farmanager.com/ru/service_functions/defdlgproc.html)
- [ ] [DialogFree](https://api.farmanager.com/ru/service_functions/dialogfree.html)
- [ ] [DialogInit](https://api.farmanager.com/ru/service_functions/dialoginit.html)
- [ ] [DialogRun](https://api.farmanager.com/ru/service_functions/dialogrun.html)
- [ ] [SendDlgMessage](https://api.farmanager.com/ru/service_functions/senddlgmessage.html)

### Settings API

Export functions

- [x] [ConfigureW](https://api.farmanager.com/ru/exported_functions/configurew.html)

Service functions

- [ ] [SettingsControl](https://api.farmanager.com/ru/service_functions/settingscontrol.html)

### Plugin Manager API

Service functions

- [ ] [PluginsControl](https://api.farmanager.com/ru/service_functions/pluginscontrol.html)

### Miscellaneous API

Export functions

- [ ] [ProcessConsoleInputW](https://api.farmanager.com/ru/exported_functions/processconsoleinputw.html)
- [ ] [ProcessSynchroEventW](https://api.farmanager.com/ru/exported_functions/processsynchroeventw.html)

Service functions

- [ ] [AdvControl](https://api.farmanager.com/ru/service_functions/advcontrol.html)
- [x] [ColorDialog](https://api.farmanager.com/ru/service_functions/colordialog.html)
- [ ] [RegExpControl](https://api.farmanager.com/ru/service_functions/regexpcontrol.html)
- [ ] [RestoreScreen](https://api.farmanager.com/ru/service_functions/restorescreen.html)
- [ ] [SaveScreen](https://api.farmanager.com/ru/service_functions/savescreen.html)
- [ ] [Text](https://api.farmanager.com/ru/service_functions/text.html)

### Macro API

Service functions

- [ ] [MacroControl](https://api.farmanager.com/ru/service_functions/macrocontrol.html)

## License
[license]: #license

This project is licensed under the terms of both the MIT license and the Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE), [LICENSE-MIT](LICENSE-MIT)