#![no_main]

use crate::{constants::*, extensions::ArgumentsExtension};
use arguments::Arguments;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs::read_to_string;

#[macro_use]
mod macros;

mod constants;
#[path = "misc/crash_log.rs"]
mod crash_log;
#[path = "imports/experimental.rs"]
mod experimental;
mod extensions;
mod fiber;
mod fiber_structs;
mod patcher;
mod patches;
#[path = "misc/utils.rs"]
mod utils;

/// Setup CLI Arguments.
fn setup_args() -> Arguments {
    let args = arguments::parse(std::env::args())
        .unwrap_or_else(|_| crash!("Failed parsing CLI Arguments!", false));

    if args.exists("help") {
        log!(0, "USAGE: fssa --pack-files [PATH] --patch [PATCH_NAME]");
        log!(0, "OPTIONAL FLAGS: --hwid | --revert");
        log!(0, "EXAMPLE: .\\fssa --pack-files \"C:/Program Files/Star Stable Online/client/PackFiles/\" --patch change_name --uid youruserid");
        log!(0, "PATCH TYPES (case-sensitive): GAME_START, INV_CLICK_EXTENDED, QUESTS_CLICK, COLLECTIONS_CLICK, CSHEET_CLICK");
        log!(
            2,
            "All back-slashes (\\) have to be replaced with forward ones (/)."
        );

        std::process::exit(0)
    };

    args
}

/// Creates a basic progress bar.
fn create_progress_bar() -> ProgressBar {
    let style = ("█▓▒░  ", "white");
    let bar = ProgressBar::new(50);
    bar.set_style(
        ProgressStyle::with_template(&format!("[FSSA] » {{msg}} {{bar:.{}}}", style.1))
            .unwrap()
            .progress_chars(style.0),
    );

    bar.set_prefix(style.0);
    bar
}

/// Startup function.
#[no_mangle]
#[tokio::main]
async fn main() {
    println!("{}", SPLASH);
    let pb = create_progress_bar();
    pb.set_message("Starting FSSA");
    let args = setup_args();

    // Collect the arguments.
    let mut pack_files = args
        .get::<String>("pack-files")
        .unwrap_or_else(|| crash!("No --pack-files [...] specified, read --help!", true));
    let mut patch = args
        .get::<String>("patch")
        .unwrap_or_else(|| crash!("No --patch [...] specified, read --help!", true));

    let revert = args.get_or::<bool>("revert", false);

    if !pack_files.ends_with('/') {
        pack_files.push('/');
    }

    if !patch.ends_with(".patch") {
        patch.push_str(".patch");
    }

    log!(2,"I am not responsible for any damages that this may cause to your account. Use FSSA at your own risk!");
    log!(
        2,
        "To undo **all** your patches, press \"Repair Game Files\" in the game launcher."
    );
    log!(
        2,
        "To undo a specific patch, add --revert at the end of the command you used when patching."
    );

    log!(1, &format!("Patch specified: '{patch}'"));

    // Skip using read_patch() here since it's using additional logic, which isn't needed when we
    // are just determining the patch type.
    pb.inc(1);
    pb.set_message("main: Verifying patch");
    let read_patch = read_to_string(&patch);
    if let Ok(read_patch) = read_patch {
        pb.inc(1);
        pb.set_message("main: Verifying patch type");
        let patch_type = utils::get_ascii(
            read_patch
                .lines()
                .next()
                .unwrap_or_else(|| crash!("Failed finding the patch type!", true)),
        );

        pb.inc(1);
        pb.set_message("main: Patching game");
        patch_game(&patch_type, pack_files.to_owned(), patch, revert, &pb);
        pb.inc(1);
        pb.finish_with_message("main: All patches done!");
        log!(0, "We're all done, you may now close FSSA.")
    } else {
        crash!("The patch you specified wasn't found!", true);
    }
}

/// Patches the game.
fn patch_game(patch_type: &str, pack_files: String, patch: String, revert: bool, pb: &ProgressBar) {
    let patch_type = patch_type.replace(' ', "");
    let patch_type = patch_type.as_str();
    match patch_type {
        "GAME_START" => patches::patch(
            &pack_files,
            patch,
            "02",
            GAME_HMARKET_START,
            true,
            revert,
            pb,
        ),
        "INV_CLICK_EXTENDED" => {
            // First patch the INV_CLICK callback with calling OnActionStart on Introduction1.
            patches::patch(
                &pack_files,
                r#"global/ABTests/SkipIntroMovie.RunScript("Introduction1_OnActionStart");"#
                    .to_owned(),
                "02",
                GAME_INV_CLICK,
                false,
                revert,
                pb,
            );

            // Patch the callback with user-defined code.
            patches::patch(&pack_files, patch, "02", GAME_INTRO_START, true, revert, pb);
        }
        "QUESTS_CLICK" => patches::patch(
            &pack_files,
            patch,
            "02",
            GAME_QUESTS_CLICK,
            true,
            revert,
            pb,
        ),
        "COLLECTIONS_CLICK" => patches::patch(
            &pack_files,
            patch,
            "02",
            GAME_COLLECTIONS_CLICK,
            true,
            revert,
            pb,
        ),
        "CSHEET_CLICK" => patches::patch(
            &pack_files,
            patch,
            "02",
            GAME_CSHEET_CLICK,
            true,
            revert,
            pb,
        ),
        _ => {
            crash!(
                "Your script has an invalid patch type, see --help for all the available types.",
                true
            )
        }
    }
}
