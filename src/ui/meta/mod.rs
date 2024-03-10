use crate::ui::consts::{META_FILE, META_PATH, UI_ANNOTATION, UI_PATH};
use crate::ui::meta::lexer::Lexer;
use crate::ui::meta::token::{Keyword, Token};
use std::fs::{create_dir_all, read_dir, File, OpenOptions, ReadDir};
use std::io::{Read, Write};
use std::path::PathBuf;

pub mod lexer;
pub mod token;

pub fn generate_meta() {
    if let Ok(dir) = read_dir("src") {
        create_dir_all(UI_PATH).unwrap();
        let res = search(dir, vec![]).expect("Failed to read assets UI directory");
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(META_PATH)
            .expect(&format!("Failed to generate {} file", META_FILE));
        file.write_all(res.as_bytes())
            .expect(&format!("Failed to generate {} file", META_FILE));
    }
}

fn search(dir: ReadDir, path: Vec<String>) -> Result<String, std::io::Error> {
    let mut res = String::new();
    for entry in dir {
        let entry = entry?;
        let meta = entry.metadata()?;

        let mut new_path = path.clone();
        let mut file_name = entry.file_name().into_string().unwrap();
        let file = file_name.ends_with(".rs");
        if file {
            file_name.truncate(file_name.len() - 3);
            if !((file_name == "lib" || file_name == "main") && new_path.is_empty())
                && (file_name != "mod" || new_path.is_empty())
            {
                new_path.push(file_name);
            }
        } else {
            new_path.push(file_name);
        }

        if meta.is_dir() {
            res.push_str(&search(read_dir(entry.path())?, new_path)?);
        } else if file {
            res.push_str(&check_file(entry.path(), new_path)?);
        }
    }
    Ok(res)
}

fn check_file(file_path: PathBuf, path: Vec<String>) -> Result<String, std::io::Error> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let path = path.join("::");

    let mut res = String::new();

    let mut tokens = Lexer::new(contents);
    let mut tag;

    while let Some(mut token) = tokens.next() {
        if token == Token::EOF {
            break;
        }

        if token != Token::Hash {
            continue;
        }
        token = tokens.next().unwrap();
        if token != Token::LBracket {
            continue;
        }
        token = tokens.next().unwrap();
        if token != Token::Ident(UI_ANNOTATION.to_string()) {
            continue;
        }
        token = tokens.next().unwrap();
        if token != Token::LParen {
            continue;
        }
        token = tokens.next().unwrap();
        if let Token::Ident(ident) = token {
            tag = ident;
        } else {
            continue;
        }
        token = tokens.next().unwrap();
        if token != Token::RParen {
            continue;
        }
        token = tokens.next().unwrap();
        if token != Token::RBracket {
            continue;
        }
        token = tokens.next().unwrap();
        if token == Token::Keyword(Keyword::Pub) {
            token = tokens.next().unwrap();
            if token == Token::LParen {
                while token != Token::RParen {
                    token = tokens.next().unwrap();
                }
                token = tokens.next().unwrap();
            }
        }
        if token != Token::Keyword(Keyword::Struct) {
            continue;
        }
        token = tokens.next().unwrap();
        if let Token::Ident(ident) = token {
            if path.is_empty() {
                res.push_str(&format!("{}={};", tag, ident));
            } else {
                res.push_str(&format!("{}={}::{};", tag, path, ident));
            }
        }
    }

    Ok(res)
}
