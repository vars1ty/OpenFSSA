use crate::{
    experimental::FIBER_STD,
    fiber_structs::{FLoop, FMacro},
    utils::erase_comment_lines,
};

/// Parses IPXScript to PXScript.
pub fn parse_ipx(code: String) -> String {
    let mut code = code;
    code.push_str(FIBER_STD);

    erase_comment_lines(use_macros(&use_loops(&code)))
        .trim()
        .to_owned()
}

/// Gets all the loops from the specified code.
fn get_loops(code: &str) -> Vec<FLoop> {
    let lines: Vec<&str> = code.lines().collect();
    let len = lines.len();
    let mut result: Vec<FLoop> = Vec::new();
    const KEYWORD: &str = "loop";
    const BODY_BEGIN: &str = "{";
    const BODY_END: &str = "}";
    for i in 0..len {
        let line = lines[i];
        // Ensure the lines length is long enough, then grab the 4 first characters of the line and
        // eq-check it.
        if line.len() >= 6 && line[..KEYWORD.len()].eq(KEYWORD) {
            let mut iterations: u8 = line[KEYWORD.len() + 1..].parse().unwrap_or_else(|_| {
                crash!(
                    &format!("Line '{line}' has an invalid iteration count!"),
                    true
                )
            });

            // Never allow loops to be below 2 in iterations.
            if iterations < 2 {
                log!(2, &format!("[WARN] '{line}' cannot have {iterations} iterations, it has to be at least 2. Internally falling back to 2!"));
                iterations = 2;
            }

            // Ensure there's enough lines below.
            if len - i > 1 {
                // Begin grabbing the content.
                let mut loop_content = String::new();
                let mut has_starting = false;
                let mut has_ending = false;
                // Loop over lines 1-by-1, map the current index into j.
                (i + 1..len).for_each(|j| {
                    let line = lines[j];
                    if !has_starting && line == BODY_BEGIN {
                        // Start of body: found!
                        has_starting = true;
                    }

                    if !has_ending && line == BODY_END {
                        // End of body: found!
                        has_ending = true;
                    }

                    // Ensure that we never add the body characters to loop_content.
                    if line != BODY_BEGIN && line != BODY_END {
                        loop_content.push_str(line.trim());
                    } else if line == BODY_END {
                        // We reached the body end, prepare the output.
                        // Duplicate the content (x) amount of times, specifided in iterations.
                        let loop_content_cloned = loop_content.clone();
                        // - 1 because we already have 1 present.
                        for _ in 0..iterations - 1 {
                            loop_content.push_str(loop_content_cloned.trim());
                        }

                        // Add it to the results vector.
                        result.push(FLoop {
                            code_to_loop: loop_content_cloned,
                            iterations,
                            px_code: loop_content.to_owned(),
                        })
                    }
                });

                // Ensure that the loop had a valid body before returning.
                if !has_starting {
                    crash!(
                        &format!("'{line}' doesn't specify the start of the loop body ('{{')!"),
                        true
                    )
                }

                if !has_ending {
                    crash!(
                        &format!("'{line}' has a body, but it never ends ('}}')!"),
                        true
                    )
                }
            }
        }
    }

    result
}

// Checks if the code has any loops, then unrolls them and removes excess junk.
fn use_loops(code: &str) -> String {
    let mut code = code.to_owned();
    for r#loop in get_loops(&code) {
        code = code
            .replace(&r#loop.code_to_loop, "")
            .replace(&format!("loop {}\n{{", r#loop.iterations), &r#loop.px_code);
    }

    code.replace("\n}", "")
}

/// Checks if the code has any macros, then constructs them.
pub fn get_macros(code: &str) -> Vec<FMacro> {
    let lines = code.lines();
    let mut result: Vec<FMacro> = Vec::new();
    const MACRO_START: char = '$';
    const MACRO_ASSIGN: char = '=';
    const MACRO_END: char = ';';
    for line in lines {
        // Macro example:
        // $name="Hello";
        if line.len() >= 2 && line.starts_with(MACRO_START) && line.contains(MACRO_ASSIGN) {
            if !line.ends_with(MACRO_END) {
                crash!(&format!("'{line}' is missing a semicolon at the end. I'm not magic, fix it yourself!"), true)
            }

            let (name, value) = line
                .split_once(MACRO_ASSIGN)
                .unwrap_or_else(|| crash!(&format!("Failed getting value from '{line}'!"), true));

            // Deny spaces in macro names.
            if name.contains(' ') {
                crash!(&format!("Macro '{name}' contains spaces in the name, please remove any spaces and replace them with underscores!"), true);
            }

            if name.is_empty() {
                crash!(
                    &format!(
                        "Your macro needs to be called something unique, the name cannot be empty!"
                    ),
                    true
                );
            }

            let mut value = value.to_owned();
            value.pop();

            // Prevent macros with the same name.
            if result.iter().any(|f| f.name.eq_ignore_ascii_case(name)) {
                crash!(&format!("Macro '{name}' cannot be defined multiple times, how the fuck are you expecting me to differ the two?"), true)
            }

            // Add the macro.
            result.push(FMacro {
                name: name.to_owned(),
                value,
                full: line.to_owned(),
            })
        }
    }

    result
}

/// Replaces any matching macros in the code, with the macro value.
fn use_macros(code: &str) -> String {
    let mut code = code.to_owned();
    for r#macro in get_macros(&code) {
        let mut full_macro = r#macro.full;
        full_macro.push('\n');
        if code.contains(&r#macro.name) {
            code = code
                // Remove the macro from the code, then use the value of it.
                .replace(&full_macro, "")
                .replace(&r#macro.name, &r#macro.value)
        } else {
            // Not used, remove the macro from the code to preserve space.
            code = code.replace(&full_macro, "");
        }
    }

    code
}
