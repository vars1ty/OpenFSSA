use crate::utils;
use indicatif::ProgressBar;
use std::fs::read_to_string;

/// Overrides a string in a file, then writes it.
pub fn override_string(path: String, from: String, to: String, pb: &ProgressBar) {
    pb.inc(1);
    pb.set_message("override_string: Starting...");
    utils::replace_and_write(
        path,
        from.as_bytes(),
        generate_remaining_for(from.to_owned(), to, pb).as_bytes(),
        pb,
    );

    pb.inc(1);
    pb.set_message("override_string: Patch done!")
}

/// Generates the remaining null-bytes (`\0`) needed for `new_code` to be the exact same length as
/// `old_code`.
/// If the value of `new_code` is longer than `old_code`, then its truncated to fit the size which
/// may lead to issues in-game.
pub fn generate_remaining_for(old_code: String, new_code: String, pb: &ProgressBar) -> String {
    let mut new_code = new_code;
    if new_code.len() > old_code.len() {
        panic!(
            "{}",
            "[ERROR] Your patch is larger than the buffers capacity, please shorten your code!"
        );
    } else {
        pb.set_message(format!(
            "generate_remaining_for: You have {} bytes left to use out of the initial {}",
            old_code.len() - new_code.len(),
            old_code.len()
        ))
    }

    let len = new_code.len();
    for _ in 0..old_code.len() - len {
        new_code.push('\0')
    }

    new_code
}

/// Reads the specified patch file.
pub fn read_patch(patch: &String) -> String {
    let read = read_to_string(patch).unwrap(); // unwrap() shouldn't be triggered as it's checked
                                               // prior to calling this function.
    if read.contains("set_appearance") {
        log!(
            2,
            "If you plan on leaking horses with FSSA, link the forum while you're at it."
        )
    }

    utils::erase_comment_lines(read)
}
