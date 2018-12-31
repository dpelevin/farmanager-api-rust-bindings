use farmanager_codegen::Langpack;
use farmanager::basic;

#[derive(Langpack, Copy, Clone)]
#[langpack(name = "hellorust")]
#[language(code = "en", value = "English,English")]
#[language(code = "ru", value = "Russian,Russian (Русский)")]
pub enum Lng {

    #[msg(language = "en", value = "HelloRust")]
    #[msg(language = "ru", value = "HelloRust")]
    MenuItemTitle,

    #[msg(language = "en", value = "Hello, Rust!")]
    #[msg(language = "ru", value = "Здравствуй, Rust!")]
    MessageTitle,

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
    MessageButton
}

