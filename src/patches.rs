use crate::{
    experimental, fiber,
    patcher::{self, generate_remaining_for},
    utils,
};
use indicatif::ProgressBar;

/// Removes the patch type from the string.
pub fn remove_type(patch: String) -> String {
    let mut patch = patch;
    const EMPTY: &str = "";
    let types = [
        "GAME_START",
        "INV_CLICK_EXTENDED",
        "QUESTS_CLICK",
        "COLLECTIONS_CLICK",
        "CSHEET_CLICK",
    ];

    // Remove the type from the string.
    for r#type in types {
        patch = patch.replace(r#type, EMPTY);
    }

    patch.trim().to_owned()
}

/// Base patch function.
fn base_patch(
    path: &str,
    patch: String,
    csa: &str,
    lookup_hex: String,
    read_file_op: bool,
    revert: bool,
    pb: &ProgressBar,
) {
    if !path.ends_with('/') {
        crash!(
            &format!("[ERROR] The path '{path}' has to end with a forwardslash (/)!"),
            true
        )
    }

    pb.inc(1);
    pb.set_message("base_patch: Starting");
    let mut path = path.to_owned();
    path.push_str(csa);
    path.push_str(".csa");
    let decoded = utils::hex_to_string(lookup_hex);
    let mut patch = patch;
    pb.inc(1);

    // If true, read `patch` as a file and store the content.
    if read_file_op {
        patch = fiber::parse_ipx(patcher::read_patch(&patch));
        pb.set_message("base_patch: Checking for dynamic snippets");
        patch = experimental::check_horse_purchase(patch);
        pb.set_message("base_patch: Parsed User Patch")
    } else {
        pb.set_message("base_patch: Skipped internal IPX")
    }

    // Remove the type.
    patch = remove_type(patch);

    pb.inc(1);
    pb.set_message("base_patch: Calling patcher");
    if revert {
        pb.inc(1);
        pb.set_message("base_patch: Reverting User Patch");
        patcher::override_string(
            path,
            // We need to find the patch, but since it has null-bytes at the end, we have to use
            // generate_remaining_for() to get those bytes, then search and replace it.
            generate_remaining_for(decoded.clone(), patch, pb),
            decoded,
            pb,
        )
    } else {
        patcher::override_string(path, decoded, patch, pb);
    }
    pb.inc(1);
    pb.set_message("base_patch: Patch done!");
}

/// Patches the specified CSA file with code.
pub fn patch(
    path: &str,
    patch: String,
    csa_id: &str,
    lookup_hex: &str,
    read_file_op: bool,
    revert: bool,
    pb: &ProgressBar,
) {
    pb.inc(1);
    pb.set_message("patch: Preparing patch...");
    base_patch(
        path,
        patch,
        &format!("p_000000{csa_id}"),
        lookup_hex.to_owned(),
        read_file_op,
        revert,
        pb,
    )
}
