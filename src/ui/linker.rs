use crate::ui::consts::{LIB_PATH, MAIN_PATH, UI_COMPILED_PATH, UI_MOD_PATH};
use crate::ui::meta::lexer::Lexer;
use crate::ui::meta::token::{Keyword, Token};
use hashbrown::HashSet;
use std::fs::{read_dir, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

pub fn generate_modules(paths: Vec<PathBuf>) -> Result<(), std::io::Error> {
    let mut modules: HashSet<PathBuf> = ["".into()].into();

    for path in paths {
        if let Ok(relative_path) = path.strip_prefix(UI_COMPILED_PATH) {
            let parent = relative_path.parent().unwrap();

            for ancestor in parent.ancestors() {
                if ancestor != Path::new("") {
                    modules.insert(ancestor.to_path_buf());
                }
            }
        }
    }

    for module in modules {
        let path = Path::new(UI_COMPILED_PATH).join(&module);
        let mut file = File::create(path.join("mod.rs"))?;

        for entry in read_dir(path)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                let module_name = path.file_name().unwrap().to_str().unwrap();
                writeln!(file, "pub mod {};", module_name)?;
            } else if let Some(ext) = path.extension() {
                if ext == "rs" {
                    let stem = path.file_stem().unwrap().to_str().unwrap();
                    if stem != "mod" {
                        writeln!(file, "pub mod {};", stem)?;
                    }
                }
            }
        }
    }

    update_ui_mod_file()?;
    format();

    Ok(())
}

fn update_ui_mod_file() -> Result<(), std::io::Error> {
    let generated = "generated";
    let ui = "ui";

    if !update_file_if_exists(UI_MOD_PATH, generated)? {
        let mut ui_mod_file = File::create(UI_MOD_PATH)?;
        writeln!(ui_mod_file, "pub {}", generated)?;
    }

    if !update_file_if_exists(MAIN_PATH, ui)? {
        update_file_if_exists(LIB_PATH, ui)?;
    }

    Ok(())
}

fn update_file_if_exists(file: &str, module: &str) -> Result<bool, std::io::Error> {
    let path = Path::new(file);
    if path.exists() {
        let mut contents = String::new();
        File::open(path)?.read_to_string(&mut contents)?;

        let lexer = Lexer::new(contents);

        if !mod_exists(lexer, module) {
            let mut file = OpenOptions::new().append(true).open(path)?;
            writeln!(file, "\npub mod {};", module)?;
        }
        Ok(true)
    } else {
        Ok(false)
    }
}

fn mod_exists(lexer: Lexer, module: &str) -> bool {
    let mut lexer = lexer.peekable();
    while let Some(token) = lexer.next() {
        match token {
            Token::Keyword(Keyword::Mod) => {
                if let Some(Token::Ident(name)) | Some(Token::RawIdent(name)) = lexer.peek() {
                    if name == module {
                        return true;
                    }
                }
            }
            Token::EOF => break,
            _ => continue,
        }
    }

    false
}

fn format() {
    let status = Command::new("cargo")
        .arg("fmt")
        .status()
        .expect("Failed to execute cargo fmt");

    if !status.success() {
        eprintln!("Cargo fmt failed with status: {}", status);
    }
}
