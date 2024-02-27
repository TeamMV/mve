use crate::ui::consts::{ELEMENTS, META_FILE, META_PATH, UI_ANNOTATION, UI_COMPILED_PATH, UI_EXTENSIONS, UI_PATH};
use mvutils::utils::Recover;
use std::fs::{create_dir_all, read_dir, File, OpenOptions, ReadDir};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

pub mod compiler;
pub mod consts;
pub mod meta;
pub mod parser;

pub fn compile() {
    if let Ok(dir) = read_dir(UI_PATH) {
        if let Ok(file) = OpenOptions::new().read(true).open(META_PATH) {
            load_meta(file).expect(&format!("Failed to load {} file", META_FILE));
        } else {
            println!("No {} file found, if you have custom ui elements, please ensure they are annotated with #[{}(tag)]", META_FILE, UI_ANNOTATION);
        }
        process(dir, "".into()).expect("Failed to read assets UI directory");
    }
}

fn process(dir: ReadDir, path: PathBuf) -> Result<(), std::io::Error> {
    for entry in dir {
        let entry = entry?;
        let meta = entry.metadata()?;

        let new_path = path.join(entry.file_name());
        let mut file_name = entry.file_name().into_string().unwrap();
        let file = UI_EXTENSIONS.iter().any(|e| file_name.ends_with(e));

        if meta.is_dir() {
            process(read_dir(entry.path())?, new_path)?;
        } else if file {
            process_file(entry.path(), new_path)?;
        }
    }
    Ok(())
}

fn process_file(file_path: PathBuf, relative: PathBuf) -> Result<(), std::io::Error> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let compiled_contents = compiler::compile(contents);

    let mut output_path = Path::new(UI_COMPILED_PATH).join(relative);
    output_path.set_extension("rs");

    if let Some(parent) = output_path.parent() {
        create_dir_all(parent)?;
    }

    let mut output_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(output_path)?;
    output_file.write_all(compiled_contents.as_bytes())?;

    Ok(())
}

fn load_meta(mut file: File) -> Result<(), std::io::Error> {
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    contents
        .split(';')
        .map(|pair| pair.split_once('=').expect(&format!("Failed to parse {} file",META_FILE)))
        .for_each(|(tag, path)| {
            ELEMENTS
                .lock()
                .recover()
                .push((tag.trim().to_string(), path.trim().to_string()))
        });

    Ok(())
}
