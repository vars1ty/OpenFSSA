#[macro_export]
/// Logs a formatted message to stdout.
macro_rules! log {
    ($type_id:expr, $msg:expr) => {
        $crate::utils::log_message($type_id, $msg)
    };
}

#[macro_export]
/// Defines another macro, but with a simpler syntax which follows the style of C.
macro_rules! define {
    ($name:ident, $value:expr) => {
        macro_rules! $name {
            () => {
                $value
            };
        }
    };
}

#[macro_export]
macro_rules! crash {
    ($message:expr, $solvable_by_user:expr) => {
        $crate::crash_log::create(file!(), $message, $solvable_by_user)
    };
}
