use rand::prelude::*;
use std::borrow::Cow;
use std::path::Path;
use std::{env, process, thread, time};

use std::io::BufRead;
use syntect::dumps::{dump_to_file, from_dump_file};
use syntect::easy::HighlightFile;
use syntect::highlighting::{Style, Theme, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::util::as_24_bit_terminal_escaped;
use walkdir::WalkDir;

fn load_theme(tm_file: &str, enable_caching: bool) -> Theme {
    let tm_path = Path::new(tm_file);

    if enable_caching {
        let tm_cache = tm_path.with_extension("tmdump");

        if tm_cache.exists() {
            from_dump_file(tm_cache).unwrap()
        } else {
            let theme = ThemeSet::get_theme(tm_path).unwrap();
            dump_to_file(&theme, tm_cache).unwrap();
            theme
        }
    } else {
        ThemeSet::get_theme(tm_path).unwrap()
    }
}

fn find_files(dir: String) -> Vec<String> {
    let extensions: [&str; 10] = [
        "py", "go", "rs", "vue", "js", "java", "yaml", "proto", "yml", "json",
    ];
    let mut files: Vec<String> = Vec::new();
    for entry in WalkDir::new(dir) {
        if let Ok(file) = entry {
            if file.path().is_file() {
                if let Some(extension) = file.path().extension() {
                    let ext = extension.to_str().unwrap();
                    if extensions.contains(&ext) {
                        files.push(file.path().display().to_string());
                    }
                }
            }
        }
    }

    return files;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage {} <dir>", args[0]);
        process::exit(1);
    }

    let ss = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();

    let theme_file: String = "base16-eighties.dark".to_string();

    let theme = ts
        .themes
        .get(&theme_file)
        .map(Cow::Borrowed)
        .unwrap_or_else(|| Cow::Owned(load_theme(&theme_file, true)));

    let files = find_files(args[1].to_owned());
    let mut rng = rand::thread_rng();
    loop {
        let file_index = rng.gen_range(0..(files.len()));
        let file = files[file_index].to_owned();

        let mut highlighter = HighlightFile::new(file, &ss, &theme).unwrap();

        let mut line = String::new();
        while highlighter.reader.read_line(&mut line).unwrap() > 0 {
            {
                let parts: Vec<(Style, &str)> = highlighter
                    .highlight_lines
                    .highlight_line(&line, &ss)
                    .unwrap();
                print!("{}", as_24_bit_terminal_escaped(&parts[..], false));
                let sleep_time = rng.gen_range(30..300);
                thread::sleep(time::Duration::from_millis(sleep_time));
            }
            line.clear();
        }

        // Clear the formatting
        println!("\x1b[0m");
        thread::sleep(time::Duration::from_secs(1));
    }
}
