use std::io;
use std::io::Write;
use std::str::FromStr;

/// Reads an integer from user input
pub fn read_number(msg: &str) -> isize { read(msg, "integer") }
/// Reads a boolean from user input
pub fn read_bool(msg: &str) -> bool { read(msg, "boolean") }
/// Reads a float from user input
pub fn read_float(msg: &str) -> f64 { read(msg, "float") }

/// Reads a line from user input and try to parse it as type `T`.
pub fn read<T: FromStr>(msg: &str, name: &str) -> T {
    print!("{} ", msg);
    flush();
    loop {
        let mut s = String::new();

        if let Err(_) = io::stdin().read_line(&mut s) {
            print!("That was no valid UTF8! Please try again: ");
            flush();
            continue;
        };

        // Try to parse data as integer
        match T::from_str(&s[..s.len()-1]) {
            Err(_) => {
                print!("That was not a valid {}! Please try again: ", name);
                flush();
                continue;
            },
            Ok(v) => return v,
        }
    }
}


fn flush() {
    // This will panic if not all contents could be flushed
    io::stdout().flush().unwrap();
}
