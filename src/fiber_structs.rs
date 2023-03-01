/// Structure for `loop [x]`
pub struct FLoop {
    pub code_to_loop: String,
    pub iterations: u8,
    pub px_code: String,
}

/// Structure for `$macro_name`.
pub struct FMacro {
    pub name: String,
    pub value: String,
    pub full: String,
}
