use clap::{command, Arg, ArgAction};
use std::fs::{self, File};
// use std::io;
// use std::ops::Add;

pub struct Input {
    pub paths: Vec<String>,
    ignore_case: bool,
    recursive: bool,
}

pub fn get_args() -> Input {
    let matches = command!()
      .arg(
          Arg::new("byte_count")
              .help("print the byte count")
              .short('c')
              .long("bytes")
              .action(ArgAction::SetTrue),
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
