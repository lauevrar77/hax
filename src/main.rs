use rand::prelude::*;
use std::{
    env, fs,
    io::{self, Write},
    process, thread, time,
};

use walkdir::WalkDir;

fn find_files(dir: String) -> Vec<String> {
    let extensions: [&str; 2] = ["py", "go"];
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
    let files = find_files(args[1].to_owned());
    let mut rng = rand::thread_rng();
    loop {
        let file_index = rng.gen_range(0..(files.len()));
        let file = files[file_index].to_owned();
        let content = fs::read_to_string(file).expect("Cannot read file");
        let mut parts_index = 0;
        let parts: Vec<&str> = content.split(" ").collect();
        while parts_index < parts.len() {
            let nb_parts = rng.gen_range(0..10);

            let mut printed_parts = 0;
            while printed_parts < nb_parts && (parts_index + printed_parts) < parts.len() {
                print!("{}", parts[parts_index + printed_parts]);
                print!(" ");
                printed_parts += 1;
            }
            io::stdout().flush().unwrap();
            let sleep_time = rng.gen_range(10..200);
            thread::sleep(time::Duration::from_millis(sleep_time));

            parts_index += nb_parts;
        }

        thread::sleep(time::Duration::from_secs(1));
    }
}
