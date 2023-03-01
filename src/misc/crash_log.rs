/// Prints a basic crash log.
pub fn create(file: &str, message: &str, solvable_by_user: bool) -> ! {
    println!(
        "[FSSA] » FSSA has crashed, here's some information regarding the crash:\n{file}: \"{message}\", manually solvable by you: {solvable_by_user}"
    );
    std::process::exit(-1);
}
