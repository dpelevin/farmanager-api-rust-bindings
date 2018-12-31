#![warn(bare_trait_objects)]

use std::env;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

mod lng;

pub fn generate_lng_files() {
    let mut langpack: Option<lng::Langpack> = None;

    let target_path : PathBuf = Path::new(&env::var("OUT_DIR").unwrap()).join("..").join("..").join("..");
    let f0_path = target_path.join("build_output.txt");
    let mut f0 = File::create(&f0_path).unwrap();

    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let source_path : PathBuf = Path::new(&dir).join("src");
    copy_resources(&source_path.as_path(), &target_path.as_path());

    let paths = fs::read_dir(&source_path.as_path()).unwrap();

    for path in paths {
        let p = path.unwrap().path();
        if p.is_file() {
            if p.extension().is_some() {
                let extension = p.extension().unwrap();
                if "rs" == extension {
                    let mut file = File::open(p.as_path()).unwrap();
                    let mut content = String::new();
                    file.read_to_string(&mut content).unwrap();

                    langpack = lng::search_for_langpack(content);

                    if langpack.is_some() {
                        break;
                    }
                }
            }
        }
    }

    match langpack {
        Some(lp) => {
            let _ = f0.write_all(format!("{:?}\n", lp).as_bytes());
            for (lang, attr) in lp.definition.languages {
                let path = target_path.join(format!("{}_{}.lng", lp.definition.name, lang));
                let _ = f0.write_all(format!("Output path: {:?}\n", path).as_bytes());
                let mut f = File::create(&path).unwrap();
                let _ = f.write_all(format!("\u{feff}.Language={}\n\n", attr).as_bytes());

                match lp.messages.get(&lang) {
                    Some(messages) => {
                        for m in messages {
                            let _ = f.write_all(format!("\"{}\"\n", m).as_bytes());
                        }
                    },
                    None => {}
                }
                let _ = f.flush();
            }
        },
        None => {
            let _ = f0.write_all(format!("None!\n").as_bytes());
        }
    }
    let _ = f0.flush();
}

fn copy_resources(src_path: &Path, target_path: &Path) {

    let paths = fs::read_dir(src_path).unwrap();

    for path in paths {
        let p = path.unwrap().path();
        if p.is_file() {
            if p.extension().is_some() {
                let extension = p.extension().unwrap();
                if "hlf" == extension {
                    let file_name = p.file_name().unwrap();
                    let target_file_path = target_path.join(file_name);
                    fs::copy(&p, target_file_path.as_path()).unwrap();
                }
            }
        }
    }

}
