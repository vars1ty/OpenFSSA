use arguments::Arguments;
use std::str::FromStr;

/// Basic extensions for `Arguments`.
pub trait ArgumentsExtension {
    /// Gets the specified argument if present. Otherwise uses the fallback value from `or` and
    /// returns it.
    fn get_or<T: FromStr>(&self, name: &str, or: T) -> T;

    /// Checks if the argument is present or not.
    fn exists(&self, name: &str) -> bool;
}

impl ArgumentsExtension for Arguments {
    fn get_or<T: FromStr>(&self, name: &str, or: T) -> T {
        self.get::<T>(name).unwrap_or(or)
    }

    fn exists(&self, name: &str) -> bool {
        self.get::<String>(name).is_some()
    }
}
