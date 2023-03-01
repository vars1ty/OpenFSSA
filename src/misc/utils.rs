use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
};
use indicatif::ProgressBar;
use std::{
    fmt::Display,
    fs::{write, File},
    io::{stdout, BufReader, Read},
    path::Path,
};

/// Gets only ascii characters from the string.
pub fn get_ascii(string: &str) -> String {
    string.chars().filter(|c| c.is_ascii()).collect()
}

/// Remove lines that are just purely a comment.
pub fn erase_comment_lines(patch: String) -> String {
    let mut new_patch = String::default();
    const COMMENT_STD: &str = "//";
    const COMMENT_HSH: char = '#';
    for line in patch.lines() {
        if !line.starts_with(COMMENT_STD) && !line.starts_with(COMMENT_HSH) {
            new_patch.push_str(line);
            new_patch.push('\n')
        }
    }

    new_patch
}

/// Read a file as bytes and return it as a `Vec<u8>`.
pub fn read_as_bytes(path: impl Display + AsRef<Path>) -> Vec<u8> {
    let file =
        File::open(&path).unwrap_or_else(|_| panic!("[ERROR] Couldn't read the file '{path}'!"));
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer).unwrap_or_else(|_| {
        crash!(
            &format!("Couldn't read to the end of '{path}', corrupted file?"),
            false
        )
    });
    buffer
}

/// Replaces a byte-array with another byte-array.
/// For example: `replace_bytes(source, b"Hello", b"World")`
pub fn replace_bytes(source: &[u8], from: &[u8], to: &[u8], pb: &ProgressBar) -> Vec<u8> {
    let mut result = source.to_vec();
    let from_len = from.len();
    let to_len = to.len();

    let mut i = 0;
    while i + from_len <= result.len() {
        if result[i..].starts_with(from) {
            result.splice(i..i + from_len, to.iter().cloned());
            pb.inc(1);
            pb.set_message("replace_bytes: Result found, replaced");
            i += to_len;
        } else {
            i += 1;
        }
    }

    pb.inc(1);
    pb.set_message("replace_bytes: Finished!");
    result
}

/// Replaces a byte-array with another byte-array in a file, then writes it to the file.
pub fn replace_and_write(
    path: impl Display + AsRef<Path>,
    from: &[u8],
    to: &[u8],
    pb: &ProgressBar,
) {
    pb.inc(1);
    pb.set_message("replace_and_write: Reading CSA as bytes");
    let read_bytes = read_as_bytes(&path);
    pb.inc(1);
    pb.set_message("replace_and_write: Moving to write");
    write(&path, replace_bytes(&read_bytes, from, to, pb)).unwrap_or_else(|_| {
        crash!(&format!("[ERROR] Couldn't write to '{path}', perhaps FSSA is lacking the required permissions?"), true)
    });
}

/// Converts a Hex-value to a String.
pub fn hex_to_string(hex: String) -> String {
    String::from_utf8(
        hex::decode(&hex)
            .unwrap_or_else(|_| crash!(&format!("Hex '{hex}' couldn't be decoded!"), false)),
    )
    .unwrap()
}

/// Logs a message with color.
pub fn log_message(type_id: u8, str: &str) {
    let color = match type_id {
        0 => Color::Reset,
        1 => Color::Green,
        2 => Color::Yellow,
        _ => {
            crash!(
                &format!("Log Message ID {type_id} isn't valid, use 0, 1 or 2!"),
                false
            )
        }
    };

    execute!(
        stdout(),
        SetForegroundColor(color),
        Print(format!("[FSSA] » {str}\n")),
        ResetColor
    )
    .unwrap_or_else(|_| crash!("Failed writing to stdout!", false));
}
