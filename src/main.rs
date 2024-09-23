use pico_args::Arguments;
use std::collections::HashSet;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut args = Arguments::from_env();

    let quiet_mode = args.contains(["-q", "--quiet"]);
    let dry_run = args.contains(["-d", "--dry-run"]);
    let trim = args.contains(["-t", "--trim"]);

    let file_name = args.free_from_str::<String>().ok();

    let mut lines = HashSet::new();

    // If a file is provided, read existing lines into the set
    let mut file_writer: Option<BufWriter<File>> = None;
    if let Some(file_name) = &file_name {
        if let Ok(file) = File::open(file_name) {
            let reader = BufReader::new(file);
            for content in reader.lines().map_while(Result::ok) {
                let content = if trim {
                    content.trim().to_string()
                } else {
                    content
                };
                lines.insert(content);
            }
        }

        if !dry_run {
            match OpenOptions::new().append(true).create(true).open(file_name) {
                Ok(file) => {
                    file_writer = Some(BufWriter::new(file));
                }
                Err(e) => {
                    eprintln!("Failed to open file for writing: {}", e);
                    std::process::exit(1);
                }
            }
        }
    }

    // Read lines from stdin
    let stdin = io::stdin();
    let reader = BufReader::new(stdin.lock());
    for content in reader.lines().map_while(Result::ok) {
        let content = if trim {
            content.trim().to_string()
        } else {
            content
        };
        if lines.contains(&content) {
            continue;
        }

        lines.insert(content.clone());

        if !quiet_mode {
            println!("{}", content);
        }

        if !dry_run {
            if let Some(ref mut writer) = file_writer {
                if let Err(e) = writeln!(writer, "{}", content) {
                    eprintln!("Failed to write to file: {}", e);
                }
            }
        }
    }
}
