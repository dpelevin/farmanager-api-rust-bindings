use farmanager_codegen::Langpack;
use farmanager::basic;

#[derive(Langpack, Copy, Clone)]
#[langpack(name = "showcase")]
#[language(code = "en", value = "English,English")]
#[language(code = "ru", value = "Russian,Russian (Русский)")]
pub enum Lng {

    #[msg(language = "en", value = "HelloRust: API Showcase")]
    #[msg(language = "ru", value = "HelloRust: API Showcase")]
    MenuItemTitle,

    #[msg(language = "en", value = "Hello, Rust!")]
    #[msg(language = "ru", value = "Здравствуй, Rust!")]
    MessageTitle,

    #[msg(language = "en", value = "Hello, Rust! (with FMSG_ALLINONE flag)")]
    #[msg(language = "ru", value = "Здравствуй, Rust! (с флагом FMSG_ALLINONE)")]
    MessageTitleAllInOne,

    #[msg(language = "en", value = "Hello, Rust! (from commandline)")]
    #[msg(language = "ru", value = "Здравствуй, Rust! (из командной строки)")]
    MessageTitleCommandline,

    #[msg(language = "en", value = "")]
    #[msg(language = "ru", value = "")]
    MessageLine0,

    #[msg(language = "en", value = "HelloRust.rs: compiling...")]
    #[msg(language = "ru", value = "HelloRust.rs: компиляция...")]
    MessageLine1,

    #[msg(language = "en", value = "   13 error(s), 8 warning(s) :-)")]
    #[msg(language = "ru", value = "   13 ошибок, 8 предупреждений :-)")]
    MessageLine2,

    #[msg(language = "en", value = "")]
    #[msg(language = "ru", value = "")]
    MessageLine3,

    #[msg(language = "en", value = "&Ok")]
    #[msg(language = "ru", value = "Угу")]
    MessageButton,

    #[msg(language = "en", value = "Opened from left disk menu")]
    #[msg(language = "ru", value = "Запущен из левого меню дисков")]
    MessageFromLeftDiskMenu,

    #[msg(language = "en", value = "Opened from right disk menu")]
    #[msg(language = "ru", value = "Запущен из правого меню дисков")]
    MessageFromRightDiskMenu,

    #[msg(language = "en", value = "Opened from analyse")]
    #[msg(language = "ru", value = "Запущен после анализа файла")]
    MessageFromAnalyse,

    #[msg(language = "en", value = "Line 1")]
    #[msg(language = "ru", value = "Строка 1")]
    PanelMessageLine1,

    #[msg(language = "en", value = "Data 1")]
    #[msg(language = "ru", value = "Данные 1")]
    PanelMessageData1,

    #[msg(language = "en", value = "Line 2")]
    #[msg(language = "ru", value = "Строка 2")]
    PanelMessageLine2,

    #[msg(language = "en", value = "Data 2")]
    #[msg(language = "ru", value = "Данные 2")]
    PanelMessageData2,

    #[msg(language = "en", value = "Separator 2")]
    #[msg(language = "ru", value = "Разделитель 2")]
    PanelMessageSeparator1,

    #[msg(language = "en", value = "Create directory")]
    #[msg(language = "ru", value = "Создание папки")]
    MessageTitleCreateDirectory,

    #[msg(language = "en", value = "Directory name")]
    #[msg(language = "ru", value = "Имя папки")]
    MessageCreateDirectoryName,

    #[msg(language = "en", value = "Plugin configuration")]
    #[msg(language = "ru", value = "Параметры плагина")]
    MessageTitleConfiguration,

    #[msg(language = "en", value = "<once they'll be here>")]
    #[msg(language = "ru", value = "<когда-нибудь они тут будут>")]
    MessageConfiguration,

    #[msg(language = "en", value = "This API is not yet implemented!")]
    #[msg(language = "ru", value = "Данное API еще не поддерживается!")]
    MessageApiIsNotImplemented,

    #[msg(language = "en", value = "Error")]
    #[msg(language = "ru", value = "Ошибка")]
    ErrorTitle,

    #[msg(language = "en", value = "Cause")]
    #[msg(language = "ru", value = "Причина")]
    ErrorCause,

    #[msg(language = "en", value = "Backtrace")]
    #[msg(language = "ru", value = "Бэктрейс")]
    ErrorBacktrace,

}

