# Grepnir
```
                            _      
  __ _ _ __ ___ _ __  _ __ (_)_ __ 
 / _` | '__/ _ \ '_ \| '_ \| | '__|
| (_| | | |  __/ |_) | | | | | |   
 \__, |_|  \___| .__/|_| |_|_|_|   
 |___/         |_|                 


                            _      
                           (_)     
  __ _ _ __ ___ _ __  _ __  _ _ __ 
 / _` | '__/ _ \ '_ \| '_ \| | '__|
| (_| | | |  __/ |_) | | | | | |   
 \__, |_|  \___| .__/|_| |_|_|_|   
  __/ |        | |                 
 |___/         |_|                 

                               __       
.-----.----.-----.-----.-----.|__|.----.
|  _  |   _|  -__|  _  |     ||  ||   _|
|___  |__| |_____|   __|__|__||__||__|  
|_____|          |__|                   

                            _      
                           (_)     
  __ _ _ __ ___ _ __  _ __  _ _ __ 
 / _` | '__/ _ \ '_ \| '_ \| | '__|
| (_| | | |  __/ |_) | | | | | |   
 \__, |_|  \___| .__/|_| |_|_|_|   
  __/ |        | |                 
 |___/         |_|                 

                                    .__        
   ___________   ____ ______   ____ |__|______ 
  / ___\_  __ \_/ __ \\____ \ /    \|  \_  __ \
 / /_/  >  | \/\  ___/|  |_> >   |  \  ||  | \/
 \___  /|__|    \___  >   __/|___|  /__||__|   
/_____/             \/|__|        \/           

                                    __         
   __   _ __    __   _____     ___ /\_\  _ __  
 /'_ `\/\`'__\/'__`\/\ '__`\ /' _ `\/\ \/\`'__\
/\ \L\ \ \ \//\  __/\ \ \L\ \/\ \/\ \ \ \ \ \/ 
\ \____ \ \_\\ \____\\ \ ,__/\ \_\ \_\ \_\ \_\ 
 \/___L\ \/_/ \/____/ \ \ \/  \/_/\/_/\/_/\/_/ 
   /\____/             \ \_\                   
   \_/__/               \/_/      
```

Grepnir is a Rust-based command-line tool designed for efficient text processing and searching within files. Named after the mythological Norse net, Grepnir aims to capture the essence of quick and powerful searches, providing users with the ability to sift through text with ease and precision. Whether you're handling simple text files or delving into more complex search patterns, Grepnir offers a robust solution for your searching needs.

## Installation
To install and run Grepnir on your system, you will need Rust's package manager, Cargo. If you haven't installed Rust and Cargo, please follow the [official Rust installation guide](https://www.rust-lang.org/tools/install).

Once Rust and Cargo are set up, you can clone the repository and build the project using the following commands:

```
git clone git@github.com:renatodinizc/grepnir.git
cd grepnir
cargo build --release
```

The executable will be located in `./target/release/`.

## Flags and Usage Examples

### Arguments

- `<patterns>`: `grepnir` searches for PATTERNS in each specified PATH. PATTERNS is one or more patterns separated by newline characters. `grepnir` prints each line from the file that matches a pattern. It is recommended to quote PATTERNS when using `grepnir` in a shell command to prevent shell pattern expansion.

- `[path]...`: Specify one or more paths to search for the patterns. A path of "-" stands for standard input, allowing `grepnir` to search through text piped from another command. If no path is provided, `grepnir` reads from standard input by default.

### Flags

- `-i`, `--ignore-case`: Ignore case distinctions in both the patterns and the input data. This makes the search case-insensitive, allowing characters that differ only in case to match each other.

- `-r`, `--recursive`: Read all files under each directory recursively. This option follows symbolic links only if they are explicitly included on the command line.

- `-v`, `--invert-match`: Invert the sense of matching to select lines that do not match the given patterns.

- `-h`, `--help`: Print help information about `grepnir` and its options.

- `-V`, `--version`: Print the version information of `grepnir`.

### Usage Examples

**Basic Search with Multiple Patterns:**

To search for multiple patterns "error" or "warning" in `log.txt`:

```
./target/release/grepnir 'error\nwarning' log.txt
```

**Recursive Search in Directory:**

To recursively search for "todo" in all files under the `src` directory:

```
./target/release/grepnir -r 'todo' src/
```

**Case-Insensitive Search:**

To perform a case-insensitive search for "Error" in `file.txt`:

```
./target/release/grepnir -i 'Error' file.txt
```

**Inverting the Match:**

To find lines that do not contain "deprecated" in `codebase.txt`:

```
./target/release/grepnir -v 'deprecated' codebase.txt
```

**Search Through Piped Input:**

To search for "function" in files listed by `find`:

```
find . -name '*.rs' | xargs cat | ./target/release/grepnir 'function' -
```

**Displaying Help Information:**

```
./target/release/grepnir --help
```

**Displaying Version Information:**

```
./target/release/grepnir --version
```

#### Contributing
Contributions to the Grepnir project are welcome! If you're interested in improving the application or adding new features, please consider submitting a pull request. For major changes, please open an issue first to discuss what you would like to change.

Ensure to update tests as appropriate.

### License
This project is licensed under the MIT License - see the [LICENSE](LICENSE.md) file for details.
