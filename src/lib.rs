use clap::{command, Arg, ArgAction};
use std::fs::File;
use std::io::{BufRead, BufReader};
use walkdir::WalkDir;

#[derive(Debug)]
pub struct Input {
    pub paths: Vec<String>,
    pub ignore_case: bool,
    recursive: bool,
    pattern: String,
}

pub fn get_args() -> Input {
    let matches = command!()
    .arg(
        Arg::new("pattern")
        .help("pattern to be matched")
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
          Arg::new("paths")
              .action(ArgAction::Append)
              .index(2)
              .default_value("-"),
      )
      .get_matches();

    Input {
        pattern: matches.get_one::<String>("pattern").unwrap().clone(),
        paths: matches
            .get_many::<String>("paths")
            .unwrap()
            .map(|v| v.to_string())
            .collect::<Vec<String>>(),
        ignore_case: *matches.get_one::<bool>("ignore_case").unwrap(),
        recursive: *matches.get_one::<bool>("recursive").unwrap(),
    }
}

pub fn execute(input: Input) {
    for path in input.paths {
        WalkDir::new(path)
            .into_iter()
            .filter_entry(|entry| {
                if !input.recursive && entry.file_type().is_dir() {
                    eprintln!("grepnir: {}: Is a directory", entry.path().display());
                    false
                } else {
                    true
                }
            })
            .filter_map(|e| match e {
                Err(e) => {
                    eprintln!("grepnir: {}:", e);
                    None
                }
                Ok(entry) => Some(entry),
            })
            .filter(|entry| entry.file_type().is_file())
            .filter_map(|entry| match File::open(entry.path()) {
                Err(e) => {
                    eprintln!("grepnir: {}: {}:", entry.path().display(), e);
                    None
                }
                Ok(file) => Some((file, entry.path().display().to_string())),
            })
            .for_each(|(file, path)| read_from_file(file, path, &input.pattern, input.ignore_case));
    }
}

fn read_from_file(file: File, path: String, pattern: &String, ignore_case: bool) {
    let buffer = BufReader::new(&file);

    buffer
        .lines()
        .filter_map(|e| match e {
            Err(e) => {
                eprintln!("{e}");
                None
            }
            Ok(content) => Some(content),
        })
        .filter(|line| {
            if ignore_case {
                line.to_uppercase().contains(&pattern.to_uppercase())
            } else {
                line.contains(pattern)
            }
        })
        .for_each(|line| println!("{}: {}", path, line));
}
