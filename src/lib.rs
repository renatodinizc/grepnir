use clap::{command, Arg, ArgAction};
use regex::{RegexBuilder, Regex};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use walkdir::WalkDir;

pub struct Input {
    paths: Vec<String>,
    recursive: bool,
    pattern: Regex,
    invert_match: bool,
}

pub fn get_args() -> Input {
    let matches = command!()
    .arg(
        Arg::new("patterns")
        .help(concat!("grepnir searches for PATTERNS in each PATH. ",
            "PATTERNS is one or more patterns separated by newline characters, and grepnir prints each line that matches a pattern. ",
            "Typically PATTERNS should be quoted when grepnir is used in a shell command.")
        )
        .index(1)
        .required(true)
    )
    .arg(
        Arg::new("ignore_case")
        .help("Ignore case distinctions in patterns and input data, so that characters that differ only in case match each other.")
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
        Arg::new("invert_match")
        .help("Invert the sense of matching, to select non-matching lines.")
        .short('v')
        .long("invert-match")
        .action(ArgAction::SetTrue),
    )
    .arg(
        Arg::new("path")
        .help("A PATH of “-” stands for standard input.")
        .action(ArgAction::Append)
        .index(2)
        .default_value("-"),
    )
    .get_matches();

    let paths = matches
        .get_many::<String>("path")
        .unwrap()
        .map(|v| v.to_string())
        .collect::<Vec<String>>();

    let ignore_case = matches.get_one::<bool>("ignore_case").unwrap().to_owned();

    let pattern = matches
        .get_one::<String>("patterns")
        .map(|input| { 
            RegexBuilder::new(input).case_insensitive(ignore_case)
            .build()
            .expect("Pattern invalid")
})
        .unwrap();

    Input {
        paths,
        pattern,
        recursive: matches.get_one::<bool>("recursive").unwrap().to_owned(),
        invert_match: matches.get_one::<bool>("invert_match").unwrap().to_owned(),
    }
}

pub fn execute(input: Input) {
    for path in &input.paths {
        if path == "-" {
            let buffer = BufReader::new(io::stdin());
            traversal_path(&".".to_string(), &input);
            read(buffer, None, &input);
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

    let pattern = |line: &String| {
        input.invert_match && !input.pattern.to_owned().is_match(line)
        || input.pattern.to_owned().is_match(line)
    };

    buffer
        .lines()
        .filter_map(verify_file_opening)
        .filter(pattern)
        .for_each(|line| {
            if input.recursive {
                println!("{}: {}", path.as_ref().unwrap(), line)
            } else {
                println!("{}", line)
            }
        });
}
