use crate::ui::consts::{
    ELEMENTS, META_FILE, META_PATH, STYLE_EXTENSIONS, UI_ANNOTATION, UI_COMPILED_PATH,
    UI_EXTENSIONS, UI_PATH,
};
use mvutils::utils::Recover;
use std::fs::{create_dir_all, read_dir, remove_dir_all, File, OpenOptions, ReadDir};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

pub mod consts;
pub mod linker;
pub mod meta;
pub mod style;
pub mod xml;

pub fn compile() {
    if let Ok(dir) = read_dir(UI_PATH) {
        if let Ok(file) = OpenOptions::new().read(true).open(META_PATH) {
            load_meta(file).expect(&format!("Failed to load {} file", META_FILE));
        } else {
            println!("No {} file found, if you have custom ui elements, please ensure they are annotated with #[{}(tag)]", META_FILE, UI_ANNOTATION);
        }
        let mut styles = Vec::new();
        let mut uis = Vec::new();
        process(dir, "".into(), &mut styles, &mut uis).expect("Failed to read assets UI directory");

        if Path::new(UI_COMPILED_PATH).exists() {
            if let Err(e) = remove_dir_all(UI_COMPILED_PATH) {
                eprintln!("Error removing directory '{}': {}", UI_COMPILED_PATH, e);
                return;
            }
        }

        //TODO: styles

        let mut modules = Vec::new();

        for (ui, new) in uis {
            let mut output = Path::new(UI_COMPILED_PATH).join(new);
            output.set_extension("rs");
            modules.push(output.clone());
            let err = format!("Error processing UI file '{:?}'", ui);
            process_file(ui, output).expect(&err);
        }

        linker::generate_modules(modules).expect("Failed to generate mod.rs files");
    }
}

fn process(
    dir: ReadDir,
    path: PathBuf,
    styles: &mut Vec<PathBuf>,
    uis: &mut Vec<(PathBuf, PathBuf)>,
) -> Result<(), std::io::Error> {
    for entry in dir {
        let entry = entry?;
        let meta = entry.metadata()?;

        let new_path = path.join(entry.file_name());
        let mut file_name = entry.file_name().into_string().unwrap();
        let file = UI_EXTENSIONS.iter().any(|e| file_name.ends_with(e));
        let style = STYLE_EXTENSIONS.iter().any(|e| file_name.ends_with(e));

        if meta.is_dir() {
            process(read_dir(entry.path())?, new_path, styles, uis)?;
        } else if file {
            uis.push((entry.path(), new_path));
        } else if style {
            styles.push(entry.path());
        }
    }
    Ok(())
}

fn process_file(file_path: PathBuf, output: PathBuf) -> Result<(), std::io::Error> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let compiled_contents = xml::compile(contents);

    if let Some(parent) = output.parent() {
        create_dir_all(parent)?;
    }

    let mut output_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(output)?;
    output_file.write_all(compiled_contents.as_bytes())?;

    Ok(())
}

fn load_meta(mut file: File) -> Result<(), std::io::Error> {
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    contents
        .split(';')
        .filter(|item| !item.is_empty())
        .map(|pair| {
            pair.split_once('=')
                .expect(&format!("Failed to parse {} file", META_FILE))
        })
        .for_each(|(tag, path)| {
            ELEMENTS
                .write()
                .recover()
                .push((tag.trim().to_string(), path.trim().to_string()))
        });

    Ok(())
}
