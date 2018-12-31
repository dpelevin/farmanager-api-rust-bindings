use std::collections::HashMap;

use syn;

const ATTR_NAME_LANGPACK: &str = "langpack";
const ATTR_NAME_LANGPACK_NAME: &str = "name";
const ATTR_FORMAT_LANGPACK: &str = "#[langpack(name = \"<name>\")]";

const ATTR_NAME_LANGUAGE: &str = "language";
const ATTR_NAME_LANGUAGE_CODE: &str = "code";
const ATTR_NAME_LANGUAGE_VALUE: &str = "value";
const ATTR_FORMAT_LANGUAGE: &str = "#[language(code = \"<code>\", value = \"<value>\")]";

const ATTR_NAME_MESSAGE: &str = "msg";
const ATTR_NAME_MESSAGE_LANGUAGE: &str = "language";
const ATTR_NAME_MESSAGE_VALUE: &str = "value";
const ATTR_FORMAT_MESSAGE: &str = "#[msg(language = \"<language_code>\", value = \"<value>\")]";

#[derive(Debug)]
pub struct Langpack {
    pub definition: LangpackDefinition,
    pub messages: HashMap<String, Vec<String>>
}

#[derive(Debug)]
pub struct LangpackDefinition {
    pub name: String,
    pub languages: HashMap<String, String>
}

fn is_langpack_enum(item: &syn::Item) -> bool {
    return match item {
        syn::Item::Enum(e) => {
            let mut is_langpack_attr_defined = false;
            let mut is_language_attr_defined = false;
            for a in &e.attrs {
                let meta: syn::Meta = a.interpret_meta().unwrap();
                match get_attr_name(&meta).as_str() {
                    ATTR_NAME_LANGPACK => is_langpack_attr_defined = true,
                    ATTR_NAME_LANGUAGE => is_language_attr_defined = true,
                    _ => {}
                }
            }
            is_langpack_attr_defined && is_language_attr_defined
        },
        _ => false
    }
}

fn get_attr_name(meta: &syn::Meta) -> String {
    match meta {
        syn::Meta::List(m) => m.ident.to_string(),
        syn::Meta::NameValue(m) => m.ident.to_string(),
        syn::Meta::Word(ident) => ident.to_string()
    }
}

fn parse_langpack_definition(e: &syn::ItemEnum) -> LangpackDefinition {
    let mut langpack_name: Option<String> = None;
    let mut languages: HashMap<String, String> = HashMap::new();

    for a in &e.attrs {
        let meta: syn::Meta = a.interpret_meta().unwrap();
        match get_attr_name(&meta).as_str() {
            ATTR_NAME_LANGPACK => {
                langpack_name = Some(parse_langpack_attr(&meta));
            },
            ATTR_NAME_LANGUAGE => {
                let (code, value) = parse_language_attr(&meta);
                languages.insert(code, value);
            },
            _ => {}
        }
    }

    if let Some(name) = langpack_name {
        LangpackDefinition {
            name,
            languages
        }
    } else {
        panic!("'{}' attribute should have format: '{}'", ATTR_NAME_LANGPACK, ATTR_FORMAT_LANGPACK)
    }
}

fn parse_langpack_attr(meta: &syn::Meta) -> String {
    let meta_list: &syn::MetaList = match meta {
        syn::Meta::List(meta_list) => meta_list,
        _ => panic!("'{}' attribute should have format: '{}'", ATTR_NAME_LANGPACK, ATTR_FORMAT_LANGPACK)
    };

    let name_value: &syn::MetaNameValue = meta_list.nested.iter().map(|nested_meta: &syn::NestedMeta| {
        match nested_meta {
            syn::NestedMeta::Meta(syn::Meta::NameValue(name_value)) => {
                name_value
            },
            _ => panic!("'{}' attribute should have format: '{}'", ATTR_NAME_LANGPACK, ATTR_FORMAT_LANGPACK)
        }
    }).next().unwrap_or_else(|| panic!("'{}' attribute should have format: '{}'", ATTR_NAME_LANGPACK, ATTR_FORMAT_LANGPACK));

    match (name_value.ident.to_string().as_str(), &name_value.lit) {
        (ATTR_NAME_LANGPACK_NAME, syn::Lit::Str(value)) => value.value(),
        _ => panic!("'{}' attribute should have format: '{}'", ATTR_NAME_LANGPACK, ATTR_FORMAT_LANGPACK)
    }
}

fn parse_language_attr(meta: &syn::Meta) -> (String, String) {
    let meta_list: &syn::MetaList = match meta {
        syn::Meta::List(meta_list) => meta_list,
        _ => panic!("'{}' attribute should have format: '{}'", ATTR_NAME_LANGUAGE, ATTR_FORMAT_LANGUAGE)
    };

    let mut code: Option<String> = None;
    let mut value: Option<String> = None;
    meta_list.nested.iter().map(|nested_meta: &syn::NestedMeta| {
        match nested_meta {
            syn::NestedMeta::Meta(syn::Meta::NameValue(name_value)) => {
                name_value
            },
            _ => panic!("'{}' attribute should have format: '{}'", ATTR_NAME_LANGUAGE, ATTR_FORMAT_LANGUAGE)
        }
    }).for_each(|name_value: &syn::MetaNameValue| {
        match (name_value.ident.to_string().as_str(), &name_value.lit) {
            (ATTR_NAME_LANGUAGE_CODE, syn::Lit::Str(v)) => code = Some(v.value()),
            (ATTR_NAME_LANGUAGE_VALUE, syn::Lit::Str(v)) => value = Some(v.value()),
            _ => panic!("'{}' attribute should have format: '{}'", ATTR_NAME_LANGUAGE, ATTR_FORMAT_LANGUAGE)
        };
    });

    match (code, value) {
        (Some(code), Some(value)) => (code, value),
        _ => panic!("'{}' attribute should have format: '{}'", ATTR_NAME_LANGUAGE, ATTR_FORMAT_LANGUAGE)
    }
}

fn parse_messages(langpack: LangpackDefinition, e: &syn::ItemEnum) -> Langpack {
    let mut messages: HashMap<String, Vec<String>> = HashMap::new();
    for (lng, _) in &langpack.languages {
        messages.insert(lng.to_string(), Vec::new());
    }

    for variant in &e.variants {
        let mut msgs_of_attr: HashMap<String, String> = HashMap::new();
        for a in &variant.attrs {
            let meta: syn::Meta = a.interpret_meta().unwrap();
            match get_attr_name(&meta).as_str() {
                ATTR_NAME_MESSAGE => {
                    let (lng_code, message) = parse_msg_attr(&meta);
                    msgs_of_attr.insert(lng_code, message);
                },
                _ => {}
            }
        }

        if msgs_of_attr.len() < langpack.languages.len() {
            panic!("Enum variant '{}' do not contain messages for all defined languages of langpack", &variant.ident);
        }

        for (lng, mgs_vec) in &mut messages {
            let v = msgs_of_attr.remove(lng);
            match v {
                Some(msg) => mgs_vec.push(msg),
                _ => unreachable!()
            };
        }
    }

    Langpack {
        definition: langpack,
        messages
    }
}

fn parse_msg_attr(meta: &syn::Meta) -> (String, String) {
    let meta_list: &syn::MetaList = match meta {
        syn::Meta::List(meta_list) => meta_list,
        _ => panic!("'{}' attribute should have format: '{}'", ATTR_NAME_MESSAGE, ATTR_FORMAT_MESSAGE)
    };

    let mut lng_code: Option<String> = None;
    let mut value: Option<String> = None;
    meta_list.nested.iter().map(|nested_meta: &syn::NestedMeta| {
        match nested_meta {
            syn::NestedMeta::Meta(syn::Meta::NameValue(name_value)) => {
                name_value
            },
            _ => panic!("'{}' attribute should have format: '{}'", ATTR_NAME_MESSAGE, ATTR_FORMAT_MESSAGE)
        }
    }).for_each(|name_value: &syn::MetaNameValue| {
        match (name_value.ident.to_string().as_str(), &name_value.lit) {
            (ATTR_NAME_MESSAGE_LANGUAGE, syn::Lit::Str(v)) => lng_code = Some(v.value()),
            (ATTR_NAME_MESSAGE_VALUE, syn::Lit::Str(v)) => value = Some(v.value()),
            _ => panic!("'{}' attribute should have format: '{}'", ATTR_NAME_MESSAGE, ATTR_FORMAT_MESSAGE)
        };
    });

    match (lng_code, value) {
        (Some(code), Some(value)) => (code, value),
        _ => panic!("'{}' attribute should have format: '{}'", ATTR_NAME_MESSAGE, ATTR_FORMAT_MESSAGE)
    }
}

pub fn search_for_langpack(content: String) -> Option<Langpack> {
    let ast: syn::File = syn::parse_file(&content).unwrap();

    let items: &Vec<syn::Item> = &ast.items;
    let item: Option<&syn::ItemEnum> = items.iter().filter(|item|is_langpack_enum(item)).map(|item| {
        if let syn::Item::Enum(enum_item) = item { enum_item } else { unreachable!() }
    }).next();
    match item {
        Some(item_enum) => {
            let langpack_def = parse_langpack_definition(item_enum);
            Some(parse_messages(langpack_def, item_enum))
        },
        None => None
    }
}
