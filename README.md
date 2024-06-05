## A Simple CLI App in Rust

grss (pronounced grass) is simple grep clone. It takes a file and a pattern and returns the content matching the pattern
`cargo run -- main src/main.rs`

Some main points learnt on this project

1.  ## Parsing Command line arguments

    Using `std::env::args` then upgrading to `Clap::Parser`

2.  ## Using a buffer to read large files

    Using `std::fs::read_to_string(&path)` could potentially be harmful as it loads entire file into memory. We use `std::io::BufReader` instead

3.  ## Error handling and reporting

    ### Unwrapping options:

    #### a. unsafe unwrap

          ```Rust
          let result = std::fs::read_to_string("test.txt").unwrap();
          ```

    #### b. match

    ```Rust
    let content = match result {
      Ok(content) => { content }
      Err(error) => { panic!("Can't deal with {}, just exit here", error); }
    };
    ```

    #### c. Question Mark ?

    Just like calling `.unwrap()` is a shortcut for the match with panic, using ? is a shortcut for the match that return `S`

    ```Rust
      fn main() -> Result<(), Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string("test.txt")?;
        println!("file content: {}", content);
        Ok(())
      }
    ```

4.  ## Output for humans and machines

    Use `println!` macro to print to the console to `stdout`
    use `eprintln!` to print errors to the `stderr`

    #### Logging

        Logging is the same as using println!, except that you can specify the importance of a message. The levels you can usually use are error, warn, info, debug, and trace (error has the highest priority, trace the lowest)

        ```Rust
          use log::{info, warn};

          fn main() {
              env_logger::init();
              info!("starting up");
              warn!("oops, nothing implemented!");
          }
        ```
