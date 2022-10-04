use assert_cmd::prelude::*;
use escargot::CargoBuild;
use predicates as pd;

/// Build `rmx` bin that accepts every command line interaction
fn no_interactive_bin() -> std::process::Command {
    CargoBuild::new()
        .bin("rmx")
        .features("auto-interactive")
        .run()
        .unwrap()
        .command()
}

#[test]
fn last_interactivity_flag_wins() {
    let mut cmd = no_interactive_bin();
    let assert = cmd
        .arg("--interactive=never")
        .arg("-i")
        .arg("-I")
        .args(&["file", "file1", "file2", "file3"])
        .assert();
    // interactive=once wins
    assert.stdout(pd::str::contains("remove 4 arguments?"));

    let mut cmd = no_interactive_bin();
    let assert = cmd
        .arg("-i")
        .arg("-I")
        .arg("--interactive=always")
        .args(&["file", "file1", "file2", "file3"])
        .assert();
    // interactive=always wins
    assert.stdout(pd::str::contains("cannot remove"));
}
