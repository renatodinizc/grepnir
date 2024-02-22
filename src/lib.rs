use clap::{command, Arg, ArgAction};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use walkdir::WalkDir;

pub struct Input {
    paths: Vec<String>,
    ignore_case: bool,
    recursive: bool,
    pattern: String,
}

pub fn get_args() -> Input {
    let matches = command!()
    .arg(
        Arg::new("patterns")
        .help(concat!("grepnir searches for PATTERNS in each PATH.",
            "PATTERNS is one or more patterns separated by newline characters, and grepnir prints each line that matches a pattern.",
            "Typically PATTERNS should be quoted when grepnir is used in a shell command.")
        )
        .index(1)
        .required(true)
    )
    .arg(
        Arg::new("ignore_case")
        .help("Ignore case distinctions in patterns and input data, so that characters that differ only in case match each other")
        .short('i')
        .long("ignore-case")
        .action(ArgAction::SetTrue),
    )
    .arg(
        Arg::new("recursive")
        .help("Read all files under each directory, recursively, following symbolic links only if  they  are  on  the command line.")
        .short('r')
        .long("recursive")
        .action(ArgAction::SetTrue),
    )
    .arg(
        Arg::new("path")
        .help(concat!("A PATH of “-” stands for standard input.",
            "If no PATH is given, recursive searches examine the working directory, and nonrecursive searches read standard input.")
        )
        .action(ArgAction::Append)
        .index(2)
    )
    .get_matches();

    let recursive = *matches.get_one::<bool>("recursive").unwrap();

    let paths = matches
        .get_many::<String>("path")
        .map(|values| values.map(|v| v.to_owned()).collect::<Vec<String>>())
        .unwrap_or_else(|| {
            if recursive {
                vec![".".to_string()]
            } else {
                vec!["-".to_string()]
            }
        });

    Input {
        paths,
        recursive,
        pattern: matches.get_one::<String>("patterns").unwrap().to_owned(),
        ignore_case: matches.get_one::<bool>("ignore_case").unwrap().to_owned(),
    }
}

pub fn execute(input: Input) {
    for path in &input.paths {
        if path == "-" {
            let buffer = BufReader::new(io::stdin());
            read(buffer, None, &input)
        } else {
            traversal_path(path, &input);
        }
    }
}

fn traversal_path(path: &String, input: &Input) {
    let recursive_option = |entry: &walkdir::DirEntry| {
        if !input.recursive && entry.file_type().is_dir() {
            eprintln!("grepnir: {}: Is a directory", entry.path().display());
            false
        } else {
            true
        }
    };

    let verify_path = |e| match e {
        Err(e) => {
            eprintln!("grepnir: {}:", e);
            None
        }
        Ok(entry) => Some(entry),
    };

    let restrict_to_files = |entry: &walkdir::DirEntry| entry.file_type().is_file();

    let verify_file_opening = |entry: walkdir::DirEntry| match File::open(entry.path()) {
        Err(e) => {
            eprintln!("grepnir: {}: {}:", entry.path().display(), e);
            None
        }
        Ok(file) => Some((file, entry.path().display().to_string())),
    };

    let read_file = |(file, path)| {
        let buffer = BufReader::new(&file);
        read(buffer, Some(path), input);
    };

    WalkDir::new(path)
        .into_iter()
        .filter_entry(recursive_option)
        .filter_map(verify_path)
        .filter(restrict_to_files)
        .filter_map(verify_file_opening)
        .for_each(read_file);
}

fn read(buffer: impl BufRead, path: Option<String>, input: &Input) {
    let verify_file_opening = |e| match e {
        Err(e) => {
            eprintln!("{e}");
            None
        }
        Ok(content) => Some(content),
    };

    let ignore_case_option = |line: &String| {
        input.ignore_case && line.to_uppercase().contains(&input.pattern.to_uppercase())
            || line.contains(&input.pattern)
    };

    buffer
        .lines()
        .filter_map(verify_file_opening)
        .filter(ignore_case_option)
        .for_each(|line| {
            if input.recursive {
                println!("{:?}: {}", path, line)
            } else {
                println!("{}", line)
            }
        });
}
