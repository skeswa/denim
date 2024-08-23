use xshell::{cmd, Shell};

use crate::CodegenCommands;
use std::fmt::Write;

pub fn add_preamble(command: &CodegenCommands, mut text: String) -> String {
    let preamble = format!("//! Generated by `cargo xtask {command}`, do not edit by hand.\n\n");
    text.insert_str(0, &preamble);
    text
}

pub fn reformat(text: String) -> String {
    let sh = Shell::new().unwrap();

    ensure_rustfmt(&sh);

    let mut stdout = cmd!(sh, "rustup run stable rustfmt --config fn_single_line=true")
        .stdin(text)
        .read()
        .unwrap();
    if !stdout.ends_with('\n') {
        stdout.push('\n');
    }
    stdout
}

pub fn write_doc_comment(contents: &[String], dest: &mut String) {
    for line in contents {
        writeln!(dest, "///{line}").unwrap();
    }
}

fn ensure_rustfmt(sh: &Shell) {
    let version = cmd!(sh, "rustup run stable rustfmt --version")
        .read()
        .unwrap_or_default();
    if !version.contains("stable") {
        panic!(
            "Failed to run rustfmt from toolchain 'stable'. \
                 Please run `rustup component add rustfmt --toolchain stable` to install it.",
        );
    }
}
