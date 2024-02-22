use clap::{command, Arg, ArgAction};
use walkdir::WalkDir;

#[derive(Debug)]
pub struct Input {
    pub paths: Vec<String>,
    pub ignore_case: bool,
    recursive: bool,
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
            .for_each(|item| println!("{}", item.path().display()));
    }
}
